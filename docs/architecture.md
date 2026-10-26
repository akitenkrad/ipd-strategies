**English** | [日本語](architecture.ja.md)

# Architecture

## Source layout

```
simulation/
├── Cargo.toml
├── src/
│   ├── main.rs            entry point: dispatches to the tournament or `checks`
│   ├── state.rs           action constants, the rule state `Policy`, the experiment state `Exp`
│   ├── strategies.rs      the custom strategies: seven stateless ones,
│   │                      then Alternator and Gradual, which keep their own state
│   ├── arena.rs           the rule book and one match between two strategies
│   ├── checks.rs          TFT one-on-one against the custom strategies
│   ├── tournament.rs      the round robin, the aggregation, and the CSV files
│   └── tests.rs           unit tests of the custom strategies
└── tests/
    ├── cli.rs             end-to-end tests of both CLI modes
    └── fixtures/          the expected `checks` output and `ranking.csv`
```

`tools/` is a Python package. `plot_tournament.py` draws the figures of the article from `out/`
(see [Usage](usage.md#figures)); the other modules are the template of the parent monorepo and are
not used yet.

## Implementing a strategy

A strategy is a type that implements `DecisionRule<MatrixGame, Exp>` from socsim-games. Three
methods are used here:

| Method | When it is called | Used for |
|---|---|---|
| `init_state` | Once at the start | Creating the agent's initial `Policy` |
| `decide` | For every match in a step | Returning one action (`0` = C, `1` = D) |
| `learn_step` | Once at the end of each step, after all matches | Advancing state that must move once per step |

`decide` receives `&self`, the agent's `&mut Policy`, a `DecisionContext`, and the RNG.

- **Rules are shared code.** One rule object serves every agent that uses it, so `decide` takes
  `&self`. A rule must not keep per-agent values in its own fields (for example in a `Cell`): those
  values are not saved, and a run resumed from a snapshot would diverge.
- **Per-agent state lives in `Policy`.** `Policy` is stored inside the `GameWorld`, so a snapshot
  restores it. It is an enum of the built-in rules' state (`Policy::Builtin`) and the state of the
  two custom stateful rules (`Policy::Alternator`, `Policy::Gradual`). The built-in rules find their
  own state through the `Slot<T>` trait; `state.rs` forwards those lookups to `Policy::Builtin`.
- **History comes from `DecisionContext`.** `opponent_history` returns the history of the current
  opponent only and `None` for other agents, so the library enforces the information structure of
  the repeated game.
- **Randomness comes only from the `rng` argument**, so the same seed gives the same result.

### Stateless strategies

`HistoryRule` (in the first half of `strategies.rs`) holds a name and one function
`fn(&Seen, &mut SimRng) -> usize`. `Seen` is the opponent's past moves (oldest first) and the
agent's own previous move. Seven strategies are written as such functions: TF2T, Suspicious TFT,
Generous TFT, Joss, Extort-2, Prober and Soft Majority. Their state is
`Policy::Builtin(Stateless)`.

### Stateful strategies

- **Alternator** keeps the next move in `Policy::Alternator { next }`. `decide` only reads it, and
  `learn_step` flips it, so the state advances exactly once per step even if `decide` is called
  several times in a step.
- **Gradual** keeps the remaining retaliations and the remaining calming moves in
  `Policy::Gradual { remaining_defections, remaining_calming }` and advances them inside `decide`.
  This is correct here because each agent plays one match per step. A setting that calls `decide`
  several times per step would need to move the update to `learn_step`, as Alternator does.

## Tournament pipeline

`tournament()` in `tournament.rs` calls these functions in order:

1. `play_all` plays every pair and seed and collects the payoff cells, `matches.csv` and
   `first_moves.csv`.
2. `matrix_csv` builds the payoff matrix (mean or standard deviation across seeds).
3. `rank_strategies` computes the mean payoff, the standard deviation and the win/loss/tie counts,
   and sorts the strategies.
4. `ranking_csv` formats the ranking.
5. `warn_seed_dependent_deterministic` prints a `WARNING` if a pair of deterministic strategies
   depends on the seed.

## Tests

- `simulation/src/tests.rs` has one unit test per custom strategy, plus one for Pavlov (ten in
  total). Stateless strategies are tested by calling their functions on a constructed `Seen`;
  probabilities are estimated over 12,000 draws with a fixed seed and a tolerance of ±0.02.
  Alternator and Gradual are tested through `play` against `all_c` and `all_d`. Pavlov is defined
  only in the tests, added to the rule book, and checked against the built-in `wsls` with
  `play_with` for six opponents and seeds 0 to 9.
- `simulation/tests/cli.rs` runs the binary. It checks that the `checks` output and the tournament
  ranking are byte-identical to the files in `simulation/tests/fixtures/`, and that the tournament
  writes the five CSV files.
- `tools/tests/test_plot_tournament.py` reads `out/` and checks the CSV contract, the set of nice
  strategies, the colour mapping and layout of the figures, and that drawing twice gives identical
  files (nine tests).
