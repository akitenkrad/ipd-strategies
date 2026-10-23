//! 戦略を登録した戦略集を作り，2 つの戦略を ROUNDS 回対戦させる．

use std::sync::Arc;

use socsim_games::{
    assemble, build_world, AssignGroup, AssignKind, Assignment, Automaton, AutomatonKind,
    FictitiousPlay, FixedPlayers, InitialActions, LogitChoice, MatrixGame, NormalFormOutcome,
    PlayOptions, RothErev, RuleBook, TitForTat, UniformRandom,
};

use crate::state::{Exp, C, D};
use crate::strategies::{
    extort2, generous_tft, joss, prober, soft_majority, suspicious_tft, tit_for_two_tats,
    Alternator, Choose, Gradual, HistoryRule,
};

pub const ROUNDS: usize = 200;

pub fn rule_book() -> RuleBook<MatrixGame, Exp> {
    let history_rules: [(&'static str, Choose); 7] = [
        ("tit_for_two_tats", tit_for_two_tats),
        ("suspicious_tft", suspicious_tft),
        ("generous_tft", generous_tft),
        ("joss", joss),
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
    // 組込みの戦略
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
pub fn play(a: &str, b: &str, seed: u64) -> ([f64; 2], Vec<[usize; 2]>) {
    play_with(&rule_book(), a, b, seed)
}

/// 戦略集を指定して対戦させる．テストで戦略を足すときに使う．
pub fn play_with(
    rules: &RuleBook<MatrixGame, Exp>,
    a: &str,
    b: &str,
    seed: u64,
) -> ([f64; 2], Vec<[usize; 2]>) {
    // 組込み Roth--Erev は全利得が正であるゲームを要求する．その戦略を含む
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
        rules,
        &Assignment::Ordered(groups),
        ROUNDS,
        ROUNDS as u64,
        seed,
    )
    .unwrap();
    let mut sim = assemble(
        world,
        FixedPlayers::pairs(&[(0, 1)]),
        rules.clone(),
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

/// 各ラウンドの手から，プレイヤー `who` の手を `C` と `D` の文字列にする．
pub fn letters(moves: &[[usize; 2]], who: usize) -> String {
    moves
        .iter()
        .map(|m| if m[who] == C { 'C' } else { 'D' })
        .collect()
}
