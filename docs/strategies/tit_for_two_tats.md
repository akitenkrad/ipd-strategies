**English** | [日本語](tit_for_two_tats.ja.md)

# Tit for Two Tats (`tit_for_two_tats`)

Tit for Two Tats cooperates unless the opponent defected in each of the last two rounds; only then
does it defect. It is a more forgiving Tit for Tat that ignores a single defection.

## Origin and properties

Tit for Two Tats (TF2T) belongs to the family of variations on Tit for Tat that Axelrod discusses
(Axelrod, 1984). It was entered in Axelrod's second tournament, by John Maynard Smith (Axelrod,
1980).

In Axelrod's terms it is:

- **nice**: it never defects first. It needs two defections from the opponent before it defects at
  all, so it cannot be the first to defect.
- **retaliatory, but slowly**: it answers only two defections in a row. One isolated defection goes
  unpunished.
- **forgiving**: as soon as the opponent cooperates once, the last two moves are no longer D, D, and
  it returns to C.
- **deterministic**: it draws no random numbers.

Its strength is that a single defection, by mistake or by probing, does not start an echo of
retaliation as it does between two Tit for Tat players. Its weakness is the same tolerance: an
opponent that never defects twice in a row (for example one that alternates C and D) can defect
every other round without any response.

## How it moves

The rule looks only at the opponent's moves:

- If the opponent's last two moves are D, D, play D.
- Otherwise play C. This includes the first two rounds, where the opponent has played fewer than two
  moves.

The strategy is **stateless**. It keeps nothing between rounds; each round it recomputes the move
from the match history, which arrives as `Seen { opp, mine }` (`opp` is the opponent's moves, oldest
first; `mine` is its own previous move, unused here). Its `init_state` returns
`Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))`, the empty state shared by all
history-based strategies in this repository.

The traces below are the first 16 rounds at seed 0 from the tournament output. The strategy is
deterministic, so every seed gives the same moves.

Against `all_d`, it cooperates twice and then defects for good:

| Round | Opponent (`all_d`) | `tit_for_two_tats` | Note |
|---:|:-:|:-:|---|
| 1 | D | C | No history: C |
| 2 | D | C | Only one opponent move so far: C |
| 3 | D | D | Opponent's last two moves are D, D |
| 4–16 | D | D | Still D, D |

Against `prober`, it is detected as exploitable. Prober opens with D, C, C; because TF2T answered
with C in rounds 2 and 3, Prober defects from round 4 on:

| Round | Opponent (`prober`) | `tit_for_two_tats` | Note |
|---:|:-:|:-:|---|
| 1 | D | C | No history: C |
| 2 | C | C | Last two: D (only one move) |
| 3 | C | C | Last two: D, C |
| 4 | D | C | Last two: C, C |
| 5 | D | C | Last two: C, D |
| 6 | D | D | Last two: D, D |
| 7–16 | D | D | Still D, D |

Against `alternator` (C, D, C, D, …), it plays C in all 16 rounds, because the opponent never
defects twice in a row.

## Reading the code

The function is
[`tit_for_two_tats` in `strategies.rs`](../../simulation/src/strategies.rs#L58-L65):

```rust
pub fn tit_for_two_tats(s: &Seen, _: &mut SimRng) -> usize {
    if s.opp.ends_with(&[D, D]) {
        D
    } else {
        C
    }
}
```

- `s.opp` is the opponent's moves in this match, oldest first. `ends_with(&[D, D])` is true only when
  there are at least two moves and the last two are both D. With fewer than two moves it is false,
  which is what makes the first two rounds C.
- The random number generator is ignored (`_`).
- `s.mine` is not read.

The call path:

1. [`rule_book` in `arena.rs`](../../simulation/src/arena.rs#L19-L34) lists
   `("tit_for_two_tats", tit_for_two_tats)` in the `history_rules` array
   ([L21](../../simulation/src/arena.rs#L21)) and registers each entry as
   `HistoryRule { name, choose }`.
2. Each round, [`play_with`](../../simulation/src/arena.rs#L124-L125) calls `sim.step()`. In that
   step the socsim-games engine asks each of the two players for a move by calling the rule's
   `decide`.
3. [`HistoryRule::decide`](../../simulation/src/strategies.rs#L46-L56) calls
   [`seen(ctx)`](../../simulation/src/strategies.rs#L17-L35), which takes the only opponent, reads its
   history in this match (`opp`, dropping rounds with no recorded action), and reads this player's
   last action (`mine`).
4. `decide` passes the result to `choose`, which for this strategy is `tit_for_two_tats`, and returns
   its move.

## References

- Axelrod, R. (1980). More effective choice in the prisoner's dilemma. *Journal of Conflict
  Resolution*, 24(3), 379–403.
- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.

[← Strategies](../strategies.md)
