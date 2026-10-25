**English** | [日本語](joss.ja.md)

# Joss (`joss`)

Joss plays like Tit for Tat, but where Tit for Tat would cooperate it defects with probability 0.1.
It cooperates in the first round and always answers an opponent's D with D.

## Origin and properties

Joss is named after Johann Joss, who entered it in Axelrod's first computer tournament (Axelrod,
1980). Axelrod (1984) describes it as a variation of Tit for Tat that tries to get away with an
occasional defection: like Tit for Tat it always defects right after the other player defects, but
after the other player cooperates it defects 10 percent of the time.

In Axelrod's (1984) terms it is:

- **not nice**: it can defect first. A random defection can come in any round after the first, even
  when the opponent has only cooperated.
- **retaliatory**: an opponent's D is always answered with D.
- **forgiving**: an opponent's C is answered with C nine times out of ten.
- **stochastic**: it draws a random number in every round that follows an opponent's C.

The small variation costs it a lot against reciprocators. Axelrod (1984) shows how a single random
defection against Tit for Tat sets off an echo in which the two players alternate D and C. It follows
from the rules that a second random defection during the echo, in a round where Joss answers the
opponent's C, makes both players defect in that round and in every round after it. The trace below
shows the start of such an echo.

## How it moves

- In the first round (no opponent move yet), play C.
- If the opponent's last move was D, play D.
- If the opponent's last move was C, play D with probability 0.1 and C with probability 0.9.

As a memory-one strategy, its cooperation probabilities after (own move, opponent's move) of
(C, C), (C, D), (D, C), (D, D) are 0.9, 0, 0.9, 0. Its own previous move does not matter, and the
first round is always C. The code follows the description above; it adds nothing beyond it.

The strategy is **stateless**. It keeps nothing between rounds; each round it recomputes the move
from the match history, which arrives as `Seen { opp, mine }` (`opp` is the opponent's moves, oldest
first; `mine` is its own previous move, unused here). Its `init_state` returns
`Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))`, the empty state shared by all
history-based strategies in this repository.

The traces below are the first 16 rounds at seed 0 from the tournament output. Because the strategy
is stochastic, each trace is one sample; other seeds give other sequences.

Against `tit_for_tat`, one random defection starts an echo:

| Round | Opponent (`tit_for_tat`) | `joss` | Note |
|---:|:-:|:-:|---|
| 1 | C | C | No history: C |
| 2–9 | C | C | After C: the draw gave C |
| 10 | C | D | After C: random defection |
| 11 | D | C | After C: C. Tit for Tat answers round 10 |
| 12 | C | D | After D: D |
| 13 | D | C | After C: C |
| 14 | C | D | After D: D |
| 15 | D | C | After C: C |
| 16 | C | D | After D: D |

Against `all_c`, which never retaliates, the random defection is just a gain of T = 5 in that round:
Joss plays C in every round except round 10, where it plays D.

## Reading the code

The function is [`joss` in `strategies.rs`](../../simulation/src/strategies.rs#L80-L88), and the
defection probability is the constant
[`JOSS_DEFECT`](../../simulation/src/strategies.rs#L15):

```rust
pub const JOSS_DEFECT: f64 = 0.1;

pub fn joss(s: &Seen, rng: &mut SimRng) -> usize {
    match s.opp.last() {
        None => C,
        Some(&D) => D,
        Some(_) if rng.gen_bool(JOSS_DEFECT) => D,
        Some(_) => C,
    }
}
```

- `s.opp.last()` is the opponent's previous move, or `None` in the first round.
- `None => C`: the first round is C, with no random draw.
- `Some(&D) => D`: an opponent's D is always answered with D, with no random draw.
- `Some(_) if rng.gen_bool(JOSS_DEFECT) => D`: the remaining case is an opponent's C. The guard draws
  a value that is true with probability 0.1, and then the strategy defects.
- `Some(_) => C`: otherwise (probability 0.9) it cooperates.
- The arms are tried in order, so the random number is drawn only after an opponent's C.

The call path:

1. [`rule_book` in `arena.rs`](../../simulation/src/arena.rs#L19-L34) lists `("joss", joss)` in the
   `history_rules` array ([L24](../../simulation/src/arena.rs#L24)) and registers each entry as
   `HistoryRule { name, choose }`.
2. Each round, [`play_with`](../../simulation/src/arena.rs#L124-L125) calls `sim.step()`. In that
   step the socsim-games engine asks each of the two players for a move by calling the rule's
   `decide`, passing a random number generator.
3. [`HistoryRule::decide`](../../simulation/src/strategies.rs#L46-L56) calls
   [`seen(ctx)`](../../simulation/src/strategies.rs#L17-L35), which takes the only opponent, reads its
   history in this match (`opp`, dropping rounds with no recorded action), and reads this player's
   last action (`mine`).
4. `decide` passes the result and the generator to `choose`, which for this strategy is `joss`, and
   returns its move.

## References

- Axelrod, R. (1980). Effective choice in the prisoner's dilemma. *Journal of Conflict Resolution*,
  24(1), 3–25.
- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.

[← Strategies](../strategies.md)
