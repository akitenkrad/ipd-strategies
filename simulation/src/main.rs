use std::fmt::Write as _;
use std::fs;
use std::sync::Arc;

use rand::Rng;
use serde::{Deserialize, Serialize};
use socsim_core::{AgentId, SimRng};
use socsim_games::{
    assemble, build_world, AssignGroup, AssignKind, Assignment, Automaton, AutomatonKind,
    AutomatonPolicyState, BeliefState, BuiltinPolicyState, DecisionContext, DecisionRule,
    EstimateState, ExperimentState, FictitiousPlay, FixedPlayers, InitialActions, LearnContext,
    LogitChoice, MatrixGame, NormalFormOutcome, PlayOptions, PropensityState, RothErev, RuleBook,
    RuleError, RuleInitContext, Slot, Stateless, TftState, TitForTat, UniformRandom,
};

const ROUNDS: usize = 200;
const C: usize = 0; // 行動 0 = 協力
const D: usize = 1; // 行動 1 = 裏切り
const GENEROSITY: f64 = 1.0 / 3.0; // Generous TFT が裏切りを許す確率
const JOSS_DEFECT: f64 = 0.1; // Joss が協力の代わりに裏切る確率
const SEEDS: usize = 30;
const STRATEGIES: [&str; 18] = [
    "all_c",
    "all_d",
    "tit_for_tat",
    "grim",
    "wsls",
    "fictitious_play",
    "uniform_random",
    "logit",
    "roth_erev",
    "tit_for_two_tats",
    "suspicious_tft",
    "generous_tft",
    "joss",
    "alternator",
    "extort2",
    "gradual",
    "prober",
    "soft_majority",
];

// ---- 実験状態: 規則の状態を入れる型を自分で決める ----

/// エージェントごとの規則の状態．組込み規則と状態を持つ自作規則を収める．
#[derive(Clone, Debug, Serialize, Deserialize)]
enum Policy {
    Builtin(BuiltinPolicyState),
    Alternator {
        next: usize,
    },
    Gradual {
        remaining_defections: usize,
        remaining_calming: usize,
    },
}

/// 組込みの規則（TFT や wsls）が自分の状態を取り出せるよう，`Builtin` に委ねる．
macro_rules! builtin_slot {
    ($t:ty) => {
        impl Slot<$t> for Policy {
            fn slot(&self) -> Option<&$t> {
                match self {
                    Policy::Builtin(s) => s.slot(),
                    _ => None,
                }
            }
            fn slot_mut(&mut self) -> Option<&mut $t> {
                match self {
                    Policy::Builtin(s) => s.slot_mut(),
                    _ => None,
                }
            }
            fn from_slot(value: $t) -> Self {
                Policy::Builtin(<BuiltinPolicyState as Slot<$t>>::from_slot(value))
            }
        }
    };
}
builtin_slot!(Stateless);
builtin_slot!(TftState);
builtin_slot!(AutomatonPolicyState);
builtin_slot!(BeliefState);
builtin_slot!(EstimateState);
builtin_slot!(PropensityState);

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Exp;

impl ExperimentState for Exp {
    type PolicyState = Policy;
    type View<'a> = ();
    fn view_for(&self, _me: AgentId) -> Self::View<'_> {}
}

type Ctx<'a> = DecisionContext<'a, MatrixGame, Exp>;

// ---- 履歴だけで手が決まる規則（状態を持たない） ----

/// 規則が読む履歴: 相手の過去の手（古い順）と，自分の前回の手．
struct Seen {
    opp: Vec<usize>,
    mine: Option<usize>,
}

fn seen(ctx: &Ctx<'_>) -> Result<Seen, RuleError> {
    let opponent = *ctx
        .opponents()
        .first()
        .ok_or_else(|| RuleError::Unsupported("no opponent".into()))?;
    let history = ctx
        .opponent_history(opponent)
        .ok_or_else(|| RuleError::Unsupported("no opponent history".into()))?;
    Ok(Seen {
        opp: history.actions().flatten().copied().collect(),
        mine: ctx.my_history().last().and_then(|r| r.action),
    })
}

/// 履歴と乱数から手を 1 つ選ぶ関数．
type Choose = fn(&Seen, &mut SimRng) -> usize;

/// 手の選び方を関数 1 つで持つだけの規則．
struct HistoryRule {
    name: &'static str,
    choose: Choose,
}

impl DecisionRule<MatrixGame, Exp> for HistoryRule {
    fn name(&self) -> &str {
        self.name
    }
    fn init_state(&self, _: &RuleInitContext, _: &MatrixGame) -> Result<Policy, RuleError> {
        Ok(Policy::Builtin(BuiltinPolicyState::Stateless(Stateless)))
    }
    fn decide(&self, _: &mut Policy, ctx: &Ctx<'_>, rng: &mut SimRng) -> Result<usize, RuleError> {
        Ok((self.choose)(&seen(ctx)?, rng))
    }
}

/// 相手が 2 回続けて裏切ったときだけ裏切る．
fn tit_for_two_tats(s: &Seen, _: &mut SimRng) -> usize {
    if s.opp.ends_with(&[D, D]) {
        D
    } else {
        C
    }
}

/// 初回は裏切り，以降は相手の前回の手をまねる．
fn suspicious_tft(s: &Seen, _: &mut SimRng) -> usize {
    s.opp.last().copied().unwrap_or(D)
}

/// TFT と同じだが，相手の裏切りを確率 GENEROSITY で許す．
fn generous_tft(s: &Seen, rng: &mut SimRng) -> usize {
    match s.opp.last() {
        Some(&D) if !rng.gen_bool(GENEROSITY) => D,
        _ => C,
    }
}

/// TFT と同じだが，協力すべき場面で確率 JOSS_DEFECT で裏切る（初回は協力）．
fn joss(s: &Seen, rng: &mut SimRng) -> usize {
    match s.opp.last() {
        None => C,
        Some(&D) => D,
        Some(_) if rng.gen_bool(JOSS_DEFECT) => D,
        Some(_) => C,
    }
}

/// Win-Stay, Lose-Shift: 前回の自分と相手の手が同じなら C，違えば D．
fn pavlov(s: &Seen, _: &mut SimRng) -> usize {
    match (s.mine, s.opp.last()) {
        (Some(m), Some(&o)) if m != o => D,
        _ => C,
    }
}

/// Press--Dyson の Extort-2．前回の手の組に応じた確率で協力する．
fn extort2(s: &Seen, rng: &mut SimRng) -> usize {
    let cooperate = match (s.mine, s.opp.last().copied()) {
        (None, _) => return C,
        (Some(C), Some(C)) => 8.0 / 9.0,
        (Some(C), Some(D)) => 1.0 / 2.0,
        (Some(D), Some(C)) => 1.0 / 3.0,
        (Some(D), Some(D)) => 0.0,
        _ => 0.0,
    };
    usize::from(!rng.gen_bool(cooperate))
}

/// 最初に D, C, C と探り，相手が応じなければ裏切り続け，応じれば TFT に移る．
fn prober(s: &Seen, _: &mut SimRng) -> usize {
    match s.opp.len() {
        0 => D,
        1 | 2 => C,
        _ if s.opp[1] == C && s.opp[2] == C => D,
        _ => s.opp.last().copied().unwrap_or(C),
    }
}

/// 相手の過去の手で協力が裏切り以上なら協力する．
fn soft_majority(s: &Seen, _: &mut SimRng) -> usize {
    let cooperations = s.opp.iter().filter(|&&action| action == C).count();
    let defections = s.opp.len() - cooperations;
    if cooperations >= defections {
        C
    } else {
        D
    }
}

// ---- 状態を持つ規則 ----

/// C と D を交互に出す．次の手を状態に持ち，ステップの終わりに入れ替える．
struct Alternator;

impl DecisionRule<MatrixGame, Exp> for Alternator {
    fn name(&self) -> &str {
        "alternator"
    }
    fn init_state(&self, _: &RuleInitContext, _: &MatrixGame) -> Result<Policy, RuleError> {
        Ok(Policy::Alternator { next: C })
    }
    fn decide(&self, state: &mut Policy, _: &Ctx<'_>, _: &mut SimRng) -> Result<usize, RuleError> {
        match state {
            Policy::Alternator { next } => Ok(*next),
            _ => Err(RuleError::MissingSlot("Alternator")),
        }
    }
    fn learn_step(
        &self,
        state: &mut Policy,
        _: &LearnContext<'_, MatrixGame, Exp>,
    ) -> Result<(), RuleError> {
        match state {
            Policy::Alternator { next } => {
                *next = if *next == C { D } else { C };
                Ok(())
            }
            _ => Err(RuleError::MissingSlot("Alternator")),
        }
    }
}

/// 相手の累積裏切り回数だけ報復し，2 回協力して鎮静する．
struct Gradual;

impl DecisionRule<MatrixGame, Exp> for Gradual {
    fn name(&self) -> &str {
        "gradual"
    }
    fn init_state(&self, _: &RuleInitContext, _: &MatrixGame) -> Result<Policy, RuleError> {
        Ok(Policy::Gradual {
            remaining_defections: 0,
            remaining_calming: 0,
        })
    }
    fn decide(
        &self,
        state: &mut Policy,
        ctx: &Ctx<'_>,
        _: &mut SimRng,
    ) -> Result<usize, RuleError> {
        let Policy::Gradual {
            remaining_defections,
            remaining_calming,
        } = state
        else {
            return Err(RuleError::MissingSlot("Gradual"));
        };
        if *remaining_defections > 0 {
            *remaining_defections -= 1;
            return Ok(D);
        }
        if *remaining_calming > 0 {
            *remaining_calming -= 1;
            return Ok(C);
        }

        let history = seen(ctx)?.opp;
        if history.last() == Some(&D) {
            let defections = history.iter().filter(|&&action| action == D).count();
            *remaining_defections = defections - 1;
            *remaining_calming = 2;
            Ok(D)
        } else {
            Ok(C)
        }
    }
}

// ---- 対戦 ----

fn rule_book() -> RuleBook<MatrixGame, Exp> {
    let history_rules: [(&'static str, Choose); 8] = [
        ("tit_for_two_tats", tit_for_two_tats),
        ("suspicious_tft", suspicious_tft),
        ("generous_tft", generous_tft),
        ("joss", joss),
        ("pavlov", pavlov),
        ("extort2", extort2),
        ("prober", prober),
        ("soft_majority", soft_majority),
    ];
    let mut rules = RuleBook::new();
    for (name, choose) in history_rules {
        rules
            .register(name, Arc::new(HistoryRule { name, choose }))
            .unwrap();
    }
    rules.register("alternator", Arc::new(Alternator)).unwrap();
    rules.register("gradual", Arc::new(Gradual)).unwrap();
    // 組込み規則
    for (name, kind) in [
        ("all_c", AutomatonKind::AllC),
        ("all_d", AutomatonKind::AllD),
        ("grim", AutomatonKind::Grim),
        ("wsls", AutomatonKind::Wsls),
    ] {
        rules
            .register(name, Arc::new(Automaton::new(kind, C)))
            .unwrap();
    }
    rules
        .register("tit_for_tat", Arc::new(TitForTat::new(C)))
        .unwrap();
    rules
        .register("fictitious_play", Arc::new(FictitiousPlay::new()))
        .unwrap();
    rules
        .register("uniform_random", Arc::new(UniformRandom))
        .unwrap();
    rules
        .register("logit", Arc::new(LogitChoice::new(1.0)))
        .unwrap();
    rules
        .register("roth_erev", Arc::new(RothErev::new(0.1, 0.2, 10.125)))
        .unwrap();
    rules
}

/// エージェント 0 に `a`，1 に `b` を割り当てて ROUNDS 回対戦させ，
/// 各自の累積利得と，各ラウンドの (0 の手, 1 の手) を返す．
fn play(a: &str, b: &str, seed: u64) -> ([f64; 2], Vec<[usize; 2]>) {
    // 組込み Roth--Erev は全利得が正であるゲームを要求する．その規則を含む
    // 対戦だけ S を最小の正規化正数で近似し，集計には元の S=0 を使う．
    let sucker = if a == "roth_erev" || b == "roth_erev" {
        f64::MIN_POSITIVE
    } else {
        0.0
    };
    let game = MatrixGame::bimatrix(
        &[vec![3.0, sucker], vec![5.0, 1.0]],
        &[vec![3.0, 5.0], vec![sucker, 1.0]],
    )
    .unwrap();
    let rules = rule_book();
    let group = |kind: &str, count| AssignGroup {
        kind: AssignKind::Rule(kind.into()),
        count,
    };
    let groups = if a == b {
        vec![group(a, 2)]
    } else {
        vec![group(a, 1), group(b, 1)]
    };
    let world = build_world(
        game,
        2,
        Exp,
        InitialActions::Same(C),
        &rules,
        &Assignment::Ordered(groups),
        ROUNDS,
        ROUNDS as u64,
        seed,
    )
    .unwrap();
    let mut sim = assemble(
        world,
        FixedPlayers::pairs(&[(0, 1)]),
        rules,
        NormalFormOutcome,
        PlayOptions::new(),
        seed,
    )
    .unwrap()
    .build();

    let (mut score, mut moves) = ([0.0; 2], Vec::with_capacity(ROUNDS));
    for _ in 0..ROUNDS {
        sim.step().unwrap();
        let w = sim.world();
        let actions = [w.held()[0], w.held()[1]];
        let original_payoffs = match actions {
            [C, C] => [3.0, 3.0],
            [C, D] => [0.0, 5.0],
            [D, C] => [5.0, 0.0],
            [D, D] => [1.0, 1.0],
            _ => unreachable!("the prisoner's dilemma has only C and D"),
        };
        score[0] += original_payoffs[0];
        score[1] += original_payoffs[1];
        moves.push(actions);
    }
    (score, moves)
}

/// 10 個の値の平均・母標準偏差・最小・最大．
fn stats(xs: &[f64]) -> String {
    let n = xs.len() as f64;
    let mean = xs.iter().sum::<f64>() / n;
    let sd = (xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n).sqrt();
    let min = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let max = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if sd == 0.0 {
        format!("{mean:.1}")
    } else {
        format!("{mean:.1}±{sd:.1} [{min}, {max}]")
    }
}

fn letters(moves: &[[usize; 2]], who: usize) -> String {
    moves
        .iter()
        .map(|m| if m[who] == C { 'C' } else { 'D' })
        .collect()
}

/// `a` 対 `b` を seed 0〜9 で走らせ，各自の累積利得・各自の C の回数・
/// (C, C) と (D, D) のラウンド数を集計する．
fn summary(a: &str, b: &str) -> Vec<String> {
    let mut cols: [Vec<f64>; 6] = Default::default();
    for seed in 0..10 {
        let (s, m) = play(a, b, seed);
        let count = |f: &dyn Fn(&[usize; 2]) -> bool| m.iter().filter(|x| f(x)).count() as f64;
        let vals = [
            s[0],
            s[1],
            count(&|x| x[0] == C),
            count(&|x| x[1] == C),
            count(&|x| x == &[C, C]),
            count(&|x| x == &[D, D]),
        ];
        for (col, v) in cols.iter_mut().zip(vals) {
            col.push(v);
        }
    }
    cols.iter().map(|c| stats(c)).collect()
}

fn print_row(label: &str, cells: &[String]) {
    print!("{label:<17}");
    for (cell, width) in cells.iter().zip([24, 24, 18, 18, 18, 18]) {
        print!(" {cell:>width$}");
    }
    println!();
}

fn legacy() {
    let opponents = [
        "tit_for_two_tats",
        "pavlov",
        "generous_tft",
        "alternator",
        "suspicious_tft",
        "joss",
    ];
    let header = ["me", "opponent", "me_C", "opp_C", "CC", "DD"].map(String::from);

    // 1. TFT 対 各戦略（seed 0〜9 の集計）
    print_row("TFT vs", &header);
    for opp in opponents {
        print_row(opp, &summary("tit_for_tat", opp));
    }

    // 2. 最初の 16 ラウンドの手（seed 0）
    println!();
    for opp in opponents {
        let (_, m) = play("tit_for_tat", opp, 0);
        println!(
            "{opp:<17} TFT {}  opp {}",
            letters(&m[..16], 0),
            letters(&m[..16], 1)
        );
    }

    // 3. 参考: Joss の相手を TFT から Generous TFT に替える
    println!();
    print_row("GTFT vs", &header);
    print_row("joss", &summary("generous_tft", "joss"));

    // 4. 検算: 自作の pavlov と組込みの wsls は，どの相手・どの seed でも同じ手を打つか
    let (mut same, mut total) = (0, 0);
    for opp in [
        "tit_for_tat",
        "tit_for_two_tats",
        "suspicious_tft",
        "generous_tft",
        "joss",
        "alternator",
    ] {
        for seed in 0..10 {
            total += 1;
            same += usize::from(play(opp, "pavlov", seed) == play(opp, "wsls", seed));
        }
    }
    println!();
    println!("pavlov == wsls: {same}/{total} runs identical");
}

#[derive(Clone, Copy)]
struct RankingRow {
    strategy: &'static str,
    mean_payoff: f64,
    std_over_seeds: f64,
    wins: usize,
    losses: usize,
    ties: usize,
    stochastic: bool,
}

fn mean_and_std(xs: &[f64]) -> (f64, f64) {
    if xs.iter().all(|value| *value == xs[0]) {
        return (xs[0], 0.0);
    }
    let mean = xs.iter().sum::<f64>() / xs.len() as f64;
    let variance = xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / xs.len() as f64;
    (mean, variance.sqrt())
}

fn is_stochastic(strategy: &str) -> bool {
    matches!(
        strategy,
        "uniform_random" | "logit" | "roth_erev" | "generous_tft" | "joss" | "extort2"
    )
}

fn tournament() -> std::io::Result<()> {
    let n = STRATEGIES.len();
    let mut cells = vec![vec![Vec::with_capacity(SEEDS); n]; n];
    let mut matches_csv = String::from("a,b,seed,score_a,score_b,coop_a,coop_b,cc,cd,dc,dd\n");
    let mut first_moves_csv = String::from("a,b,moves_a,moves_b\n");

    for (a_idx, &a) in STRATEGIES.iter().enumerate() {
        for (b_idx, &b) in STRATEGIES.iter().enumerate().skip(a_idx) {
            let mut samples_a = Vec::with_capacity(SEEDS);
            let mut samples_b = Vec::with_capacity(SEEDS);
            for seed in 0..SEEDS {
                let (score, moves) = play(a, b, seed as u64);
                let counts = [
                    moves.iter().filter(|m| m[0] == C).count(),
                    moves.iter().filter(|m| m[1] == C).count(),
                    moves.iter().filter(|m| **m == [C, C]).count(),
                    moves.iter().filter(|m| **m == [C, D]).count(),
                    moves.iter().filter(|m| **m == [D, C]).count(),
                    moves.iter().filter(|m| **m == [D, D]).count(),
                ];
                writeln!(
                    matches_csv,
                    "{a},{b},{seed},{},{},{},{},{},{},{},{}",
                    score[0],
                    score[1],
                    counts[0],
                    counts[1],
                    counts[2],
                    counts[3],
                    counts[4],
                    counts[5]
                )
                .unwrap();

                if a_idx == b_idx {
                    samples_a.push((score[0] + score[1]) / 2.0 / ROUNDS as f64);
                } else {
                    samples_a.push(score[0] / ROUNDS as f64);
                    samples_b.push(score[1] / ROUNDS as f64);
                }

                if seed == 0 {
                    writeln!(
                        first_moves_csv,
                        "{a},{b},{},{}",
                        letters(&moves[..16], 0),
                        letters(&moves[..16], 1)
                    )
                    .unwrap();
                }
            }
            cells[a_idx][b_idx] = samples_a;
            if a_idx != b_idx {
                cells[b_idx][a_idx] = samples_b;
            }
        }
    }

    let matrix_csv = |standard_deviation: bool| {
        let mut csv = String::from("strategy");
        for strategy in STRATEGIES {
            write!(csv, ",{strategy}").unwrap();
        }
        csv.push('\n');
        for (row_idx, strategy) in STRATEGIES.iter().enumerate() {
            write!(csv, "{strategy}").unwrap();
            for cell in &cells[row_idx] {
                let (mean, std) = mean_and_std(cell);
                write!(csv, ",{:.6}", if standard_deviation { std } else { mean }).unwrap();
            }
            csv.push('\n');
        }
        csv
    };

    let mut ranking = Vec::with_capacity(n);
    for (strategy_idx, &strategy) in STRATEGIES.iter().enumerate() {
        let seed_payoffs: Vec<f64> = (0..SEEDS)
            .map(|seed| {
                cells[strategy_idx]
                    .iter()
                    .map(|opponent| opponent[seed])
                    .sum::<f64>()
                    / n as f64
            })
            .collect();
        let (mean_payoff, std_over_seeds) = mean_and_std(&seed_payoffs);
        let (mut wins, mut losses, mut ties) = (0, 0, 0);
        for (opponent_idx, (own_samples, opponent_row)) in
            cells[strategy_idx].iter().zip(cells.iter()).enumerate()
        {
            if opponent_idx == strategy_idx {
                continue;
            }
            let own = own_samples.iter().sum::<f64>();
            let other = opponent_row[strategy_idx].iter().sum::<f64>();
            if (own - other).abs() < 1e-12 {
                ties += 1;
            } else if own > other {
                wins += 1;
            } else {
                losses += 1;
            }
        }
        ranking.push(RankingRow {
            strategy,
            mean_payoff,
            std_over_seeds,
            wins,
            losses,
            ties,
            stochastic: is_stochastic(strategy),
        });
    }
    ranking.sort_by(|a, b| {
        b.mean_payoff
            .total_cmp(&a.mean_payoff)
            .then_with(|| a.strategy.cmp(b.strategy))
    });

    let header = "rank,strategy,mean_payoff,std_over_seeds,wins,losses,ties,stochastic\n";
    let mut ranking_csv = String::from(header);
    for (index, row) in ranking.iter().enumerate() {
        writeln!(
            ranking_csv,
            "{},{},{:.6},{:.6},{},{},{},{}",
            index + 1,
            row.strategy,
            row.mean_payoff,
            row.std_over_seeds,
            row.wins,
            row.losses,
            row.ties,
            row.stochastic
        )
        .unwrap();
    }

    fs::create_dir_all("out")?;
    fs::write("out/matches.csv", matches_csv)?;
    fs::write("out/matrix_mean.csv", matrix_csv(false))?;
    fs::write("out/matrix_std.csv", matrix_csv(true))?;
    fs::write("out/ranking.csv", &ranking_csv)?;
    fs::write("out/first_moves.csv", first_moves_csv)?;
    print!("{ranking_csv}");

    for (a_idx, &a) in STRATEGIES.iter().enumerate() {
        for (b_idx, &b) in STRATEGIES.iter().enumerate().skip(a_idx) {
            if !is_stochastic(a) && !is_stochastic(b) {
                let (_, std_a) = mean_and_std(&cells[a_idx][b_idx]);
                let (_, std_b) = mean_and_std(&cells[b_idx][a_idx]);
                if std_a != 0.0 || std_b != 0.0 {
                    println!(
                        "WARNING: deterministic match {} vs {} has nonzero seed standard deviation",
                        a, b
                    );
                }
            }
        }
    }
    Ok(())
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("legacy") {
        legacy();
    } else {
        tournament().unwrap();
    }
}

#[cfg(test)]
mod tests;
