# Iterated Prisoner's Dilemma Strategies

This repository contains companion code for the Japanese-language Zenn article
[ゲーム理論×シミュレーション: 囚人のジレンマにおける戦略と Rust による実装](https://zenn.dev/akitenkrad/articles/9d99e25e96f48b).

The simulation runs a round-robin tournament between 18 iterated prisoner's dilemma strategies:
nine strategies built into `socsim-games` and nine strategies implemented for the article. Each
match consists of 200 rounds and is repeated for seeds 0 through 29.

## Repository layout

- `simulation/src/main.rs` implements the strategies, matches, tournament, and CLI entry point.
- `simulation/src/tests.rs` contains unit tests for the custom strategies.
- `simulation/tests/cli.rs` contains end-to-end tests for both CLI modes and their output files.
- `simulation/tests/fixtures/` contains the expected legacy output and tournament ranking.
- `tools/` is an unused template at present; it is not part of the simulation workflow.

## Commands

Run all commands from the workspace root.

```bash
# Run the round-robin tournament and write CSV files to out/
cargo run --release -q

# Play TFT against six custom strategies one-on-one,
# and check that the custom Pavlov matches the built-in wsls
cargo run --release -q -- legacy

# Run unit and CLI tests
cargo test

# Run lints
cargo clippy --all-targets -- -D warnings

# Check formatting
cargo fmt --check
```

## Tournament output

The tournament creates five CSV files in `out/`:

- `matches.csv` records scores and action counts for every strategy pair and seed.
- `matrix_mean.csv` contains the mean per-round payoff for each ordered strategy pairing.
- `matrix_std.csv` contains the standard deviation across seeds for each ordered pairing.
- `ranking.csv` ranks all strategies by overall mean payoff and includes seed variation and
  win/loss/tie counts. The same content is printed to standard output.
- `first_moves.csv` records the first 16 actions for each strategy pair at seed 0.

## License

See the included [LICENSE](LICENSE).
