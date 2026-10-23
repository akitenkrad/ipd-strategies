//! 18 戦略の総当たり (既定) と，TFT との 1 対 1 の対戦・Pavlov の検算 (`checks`) を振り分ける入口．

mod arena;
mod checks;
mod state;
mod strategies;
mod tournament;

fn main() {
    if std::env::args().nth(1).as_deref() == Some("checks") {
        checks::checks();
    } else {
        tournament::tournament().unwrap();
    }
}

#[cfg(test)]
mod tests;
