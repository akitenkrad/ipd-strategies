//! 18 戦略を全組 × 30 seed で総当たりさせ，利得行列と順位表を `out/` に書き出す．

use std::fmt::Write as _;
use std::fs;

use crate::arena::{letters, play, ROUNDS};
use crate::state::{C, D};

pub const SEEDS: usize = 30;
pub const STRATEGIES: [&str; 18] = [
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

#[derive(Clone, Copy)]
pub struct RankingRow {
    pub strategy: &'static str,
    pub mean_payoff: f64,
    pub std_over_seeds: f64,
    pub wins: usize,
    pub losses: usize,
    pub ties: usize,
    pub stochastic: bool,
}

pub fn mean_and_std(xs: &[f64]) -> (f64, f64) {
    if xs.iter().all(|value| *value == xs[0]) {
        return (xs[0], 0.0);
    }
    let mean = xs.iter().sum::<f64>() / xs.len() as f64;
    let variance = xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / xs.len() as f64;
    (mean, variance.sqrt())
}

pub fn is_stochastic(strategy: &str) -> bool {
    matches!(
        strategy,
        "uniform_random" | "logit" | "roth_erev" | "generous_tft" | "joss" | "extort2"
    )
}

/// 総当たりの結果．`cells[i][j]` は戦略 i が戦略 j と当たったときの，seed ごとの
/// 1 ラウンドあたり利得．同じ戦略どうしの対戦では 2 人の平均を入れる．
pub struct MatchResults {
    pub cells: Vec<Vec<Vec<f64>>>,
    pub matches_csv: String,
    pub first_moves_csv: String,
}

/// `matches.csv` に 1 対戦ぶんの行（累積利得と，各自の C の回数，手の組ごとの回数）を足す．
pub fn write_match_row(
    csv: &mut String,
    a: &str,
    b: &str,
    seed: usize,
    score: [f64; 2],
    moves: &[[usize; 2]],
) {
    let counts = [
        moves.iter().filter(|m| m[0] == C).count(),
        moves.iter().filter(|m| m[1] == C).count(),
        moves.iter().filter(|m| **m == [C, C]).count(),
        moves.iter().filter(|m| **m == [C, D]).count(),
        moves.iter().filter(|m| **m == [D, C]).count(),
        moves.iter().filter(|m| **m == [D, D]).count(),
    ];
    writeln!(
        csv,
        "{a},{b},{seed},{},{},{},{},{},{},{},{}",
        score[0], score[1], counts[0], counts[1], counts[2], counts[3], counts[4], counts[5]
    )
    .unwrap();
}

/// 全組 × 全 seed を対戦させ，利得のセルと `matches.csv`・`first_moves.csv` の中身を作る．
pub fn play_all() -> MatchResults {
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
                write_match_row(&mut matches_csv, a, b, seed, score, &moves);

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
    MatchResults {
        cells,
        matches_csv,
        first_moves_csv,
    }
}

/// 利得行列の CSV を作る．`standard_deviation` が真なら seed 間の標準偏差，偽なら平均を書く．
pub fn matrix_csv(cells: &[Vec<Vec<f64>>], standard_deviation: bool) -> String {
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
}

/// 戦略ごとの平均利得と勝ち・負け・引き分けを数え，平均利得の高い順に並べた順位表を作る．
pub fn rank_strategies(cells: &[Vec<Vec<f64>>]) -> Vec<RankingRow> {
    let n = STRATEGIES.len();
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
    ranking
}

/// 順位表の CSV を作る．
pub fn ranking_csv(ranking: &[RankingRow]) -> String {
    let header = "rank,strategy,mean_payoff,std_over_seeds,wins,losses,ties,stochastic\n";
    let mut csv = String::from(header);
    for (index, row) in ranking.iter().enumerate() {
        writeln!(
            csv,
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
    csv
}

/// 確率的な戦略を含まない組が seed によらず同じ結果になっているか確かめ，
/// そうでなければ WARNING を出す．
pub fn warn_seed_dependent_deterministic(cells: &[Vec<Vec<f64>>]) {
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
}

/// 総当たりを走らせ，結果を `out/` に書き出して，順位表を標準出力に出す．
pub fn tournament() -> std::io::Result<()> {
    let results = play_all();
    let ranking_csv = ranking_csv(&rank_strategies(&results.cells));

    fs::create_dir_all("out")?;
    fs::write("out/matches.csv", &results.matches_csv)?;
    fs::write("out/matrix_mean.csv", matrix_csv(&results.cells, false))?;
    fs::write("out/matrix_std.csv", matrix_csv(&results.cells, true))?;
    fs::write("out/ranking.csv", &ranking_csv)?;
    fs::write("out/first_moves.csv", &results.first_moves_csv)?;
    print!("{ranking_csv}");

    warn_seed_dependent_deterministic(&results.cells);
    Ok(())
}
