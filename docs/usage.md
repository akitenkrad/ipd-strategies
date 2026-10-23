**English** | [日本語](usage.ja.md)

# Usage

All commands run from the repository root. The simulation crate lives in `simulation/` and builds a
binary named `ipd-strategies`. Its dependencies pin socsim-games and socsim-core to commit
`3b246d2` of [rs-social-simulation-tools](https://github.com/akitenkrad/rs-social-simulation-tools).

## Commands

```bash
# Run the round-robin tournament and write CSV files to out/
cargo run --release -q

# Play TFT against six custom strategies one-on-one,
# and check that the custom Pavlov matches the built-in wsls
cargo run --release -q -- checks

# Run unit and CLI tests
cargo test

# Run lints
cargo clippy --all-targets -- -D warnings

# Check formatting
cargo fmt --check
```

`rustfmt.toml` fixes `max_width` at 100 (the rustfmt default), so a `max_width` in your home
directory does not reformat the code.

## Tournament

Without arguments, the program plays all 171 pairs of the 18 strategies (including each strategy
against itself) for seeds 0 to 29, 200 rounds each. It writes five CSV files to `out/` in the
current directory and prints the ranking to standard output.

| File | Contents |
|---|---|
| `matches.csv` | One row per pair and seed: cumulative payoffs, the number of C moves of each side, and the counts of (C, C), (C, D), (D, C) and (D, D) |
| `matrix_mean.csv` | The 18 × 18 payoff matrix: the per-round payoff the row strategy got against the column strategy, averaged over seeds |
| `matrix_std.csv` | The standard deviation of the same matrix across seeds |
| `ranking.csv` | The ranking: mean payoff, standard deviation across seeds, wins, losses, ties, and whether the strategy is stochastic |
| `first_moves.csv` | The first 16 moves of every pair at seed 0 |

The columns of `ranking.csv` are
`rank,strategy,mean_payoff,std_over_seeds,wins,losses,ties,stochastic`. How these values are
computed is described in [Strategies](strategies.md#ranking).

Pairs of deterministic strategies must give the same result for every seed. If one does not, the
program prints a `WARNING` line after the ranking.

## `checks` mode

`cargo run --release -q -- checks` prints three things:

1. TFT against TF2T, Pavlov, Generous TFT, Alternator, Suspicious TFT and Joss over seeds 0 to 9:
   cumulative payoffs, C counts, and the counts of (C, C) and (D, D), followed by the first 16 moves
   of each match.
2. Generous TFT against Joss, in the same format.
3. A check that the custom Pavlov and the built-in `wsls` play identical moves and payoffs, printed
   as `pavlov == wsls: 60/60 runs identical`.
