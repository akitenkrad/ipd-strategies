**English** | [日本語](suspicious_tft.ja.md)

# Suspicious Tit for Tat (`suspicious_tft`)

Suspicious Tit for Tat defects in the first round and then copies the opponent's previous move. It
is Tit for Tat with the opening move changed from C to D.

## Origin and properties

Suspicious Tit for Tat (STFT) is a standard variant of Tit for Tat. Boyd and Lorberbaum (1987) used
it, together with Tit for Two Tats, in their argument that no pure strategy is evolutionarily stable
in the repeated prisoner's dilemma.

In Axelrod's (1984) terms it is:

- **not nice**: it defects first, in round 1.
- **retaliatory**: a D from the opponent is answered with D in the next round.
- **forgiving**: a C from the opponent is answered with C in the next round, whatever happened
  before.
- **clear**: after the first round it is exactly Tit for Tat.
- **deterministic**: it draws no random numbers.

Its opening D gains T = 5 against an unconditional cooperator, but costs it with reciprocators.
Against Tit for Tat the opening D is copied back and the two players alternate D and C for the whole
match. Against itself both players defect in every round.

## How it moves

- In the first round, when the opponent has no moves yet, play D.
- Otherwise play the opponent's last move.

The strategy is **stateless**. It keeps nothing between rounds; each round it recomputes the move
from the match history, which arrives as `Seen { opp, mine }` (`opp` is the opponent's moves, oldest
first; `mine` is its own previous move, unused here). Its `init_state` returns
`Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))`, the empty state shared by all
history-based strategies in this repository.

The traces below are the first 16 rounds at seed 0 from the tournament output. The strategy is
deterministic, so every seed gives the same moves.

Against `tit_for_tat`, the opening D echoes back and forth:

| Round | Opponent (`tit_for_tat`) | `suspicious_tft` | Note |
|---:|:-:|:-:|---|
| 1 | C | D | No history: D |
| 2 | D | C | Copies the opponent's C from round 1 |
| 3 | C | D | Copies the opponent's D from round 2 |
| 4 | D | C | Copies C |
| 5–16 | C, D, … | D, C, … | The alternation continues |

Against `all_c`, it defects once and then cooperates:

| Round | Opponent (`all_c`) | `suspicious_tft` | Note |
|---:|:-:|:-:|---|
| 1 | C | D | No history: D |
| 2–16 | C | C | Copies C |

Against itself (`suspicious_tft` vs `suspicious_tft`), both players play D in all 16 rounds.

## Reading the code

The function is
[`suspicious_tft` in `strategies.rs`](../../simulation/src/strategies.rs#L67-L70):

```rust
pub fn suspicious_tft(s: &Seen, _: &mut SimRng) -> usize {
    s.opp.last().copied().unwrap_or(D)
}
```

- `s.opp.last()` is the opponent's previous move, or `None` in the first round.
- `.copied()` turns `Option<&usize>` into `Option<usize>`.
- `.unwrap_or(D)` returns that move, or D when there is none. This one argument is the only
  difference from Tit for Tat, which would use C here.
- The random number generator is ignored (`_`), and `s.mine` is not read.

The call path:

1. [`rule_book` in `arena.rs`](../../simulation/src/arena.rs#L19-L34) lists
   `("suspicious_tft", suspicious_tft)` in the `history_rules` array
   ([L22](../../simulation/src/arena.rs#L22)) and registers each entry as
   `HistoryRule { name, choose }`.
2. Each round, [`play_with`](../../simulation/src/arena.rs#L124-L125) calls `sim.step()`. In that
   step the socsim-games engine asks each of the two players for a move by calling the rule's
   `decide`.
3. [`HistoryRule::decide`](../../simulation/src/strategies.rs#L46-L56) calls
   [`seen(ctx)`](../../simulation/src/strategies.rs#L17-L35), which takes the only opponent, reads its
   history in this match (`opp`, dropping rounds with no recorded action), and reads this player's
   last action (`mine`).
4. `decide` passes the result to `choose`, which for this strategy is `suspicious_tft`, and returns
   its move.

## References

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Boyd, R., & Lorberbaum, J. P. (1987). No pure strategy is evolutionarily stable in the repeated
  Prisoner's Dilemma game. *Nature*, 327, 58–59.

[← Strategies](../strategies.md)
