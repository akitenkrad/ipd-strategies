//! TFT と自作の戦略を 1 対 1 で対戦させた結果 (seed 0〜9) と，Generous TFT と Joss の対戦の結果を表に出す．

use crate::arena::{letters, play};
use crate::state::{C, D};

/// 10 個の値の平均・母標準偏差・最小・最大．
pub fn stats(xs: &[f64]) -> String {
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

/// `a` 対 `b` を seed 0〜9 で走らせ，各自の累積利得・各自の C の回数・
/// (C, C) と (D, D) のラウンド数を集計する．
pub fn summary(a: &str, b: &str) -> Vec<String> {
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

pub fn print_row(label: &str, cells: &[String]) {
    print!("{label:<17}");
    for (cell, width) in cells.iter().zip([24, 24, 18, 18, 18, 18]) {
        print!(" {cell:>width$}");
    }
    println!();
}

pub fn checks() {
    let opponents = [
        "tit_for_two_tats",
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
}
