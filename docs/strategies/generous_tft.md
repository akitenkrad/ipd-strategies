**English** | [日本語](generous_tft.ja.md)

# Generous Tit for Tat (`generous_tft`)

Generous Tit for Tat plays like Tit for Tat, except that after an opponent's D it still cooperates
with probability g = 1/3. It cooperates in the first round and always after an opponent's C.

## Origin and properties

Generous Tit for Tat (GTFT) comes from Nowak and Sigmund (1992), who found it taking over in
evolutionary simulations of heterogeneous populations of stochastic reactive strategies. The
generosity is

g = min{1 − (T − R)/(R − S), (R − P)/(T − P)},

which with T = 5, R = 3, P = 1, S = 0 is min{1 − 2/3, 2/4} = 1/3.

In Axelrod's (1984) terms it is:

- **nice**: it never defects first. It only defects right after an opponent's D.
- **retaliatory, but not always**: it answers an opponent's D with D with probability 2/3.
- **forgiving**: one C from the opponent brings it back to C; it can also forgive a D on its own.
- **stochastic**: it draws a random number in every round that follows an opponent's D.

Its strength is that a single defection does not lock two reciprocators into an echo: with
probability 1/3 per round, GTFT breaks the cycle by cooperating. Its weakness is the price of that
generosity: against `all_d` it keeps being exploited in about a third of the rounds.

## How it moves

- In the first round (no opponent move yet), play C.
- If the opponent's last move was C, play C.
- If the opponent's last move was D, play C with probability g = 1/3 and D with probability 2/3.

As a memory-one strategy, its cooperation probabilities after (own move, opponent's move) of
(C, C), (C, D), (D, C), (D, D) are 1, 1/3, 1, 1/3. Its own previous move does not matter.

The strategy is **stateless**. It keeps nothing between rounds; each round it recomputes the move
from the match history, which arrives as `Seen { opp, mine }` (`opp` is the opponent's moves, oldest
first; `mine` is its own previous move, unused here). Its `init_state` returns
`Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))`, the empty state shared by all
history-based strategies in this repository.

The traces below are the first 16 rounds at seed 0 from the tournament output. Because the strategy
is stochastic, each trace is one sample; other seeds give other sequences.

Against `all_d`, it forgives in 5 of the 15 rounds that follow a D (rounds 2, 6, 8, 10 and 11):

| Round | Opponent (`all_d`) | `generous_tft` | Note |
|---:|:-:|:-:|---|
| 1 | D | C | No history: C |
| 2 | D | C | After D: forgave |
| 3 | D | D | After D: retaliated |
| 4 | D | D | Retaliated |
| 5 | D | D | Retaliated |
| 6 | D | C | Forgave |
| 7 | D | D | Retaliated |
| 8 | D | C | Forgave |
| 9 | D | D | Retaliated |
| 10 | D | C | Forgave |
| 11 | D | C | Forgave |
| 12–16 | D | D | Retaliated five times |

Against `joss`, the generosity stops an echo before it starts. Joss makes one of its random
defections in round 10; GTFT forgives it in round 11, so both are back to C:

| Round | Opponent (`joss`) | `generous_tft` | Note |
|---:|:-:|:-:|---|
| 1–9 | C | C | Mutual cooperation |
| 10 | D | C | After C: C |
| 11 | C | C | After D: forgave |
| 12–16 | C | C | Mutual cooperation |

Compare [`joss`](joss.md) against `tit_for_tat`, where the same kind of defection starts an
alternation that runs to the end of the trace.

## Reading the code

The function is
[`generous_tft` in `strategies.rs`](../../simulation/src/strategies.rs#L72-L78), and the generosity
is the constant [`GENEROSITY`](../../simulation/src/strategies.rs#L14):

```rust
pub const GENEROSITY: f64 = 1.0 / 3.0;

pub fn generous_tft(s: &Seen, rng: &mut SimRng) -> usize {
    match s.opp.last() {
        Some(&D) if !rng.gen_bool(GENEROSITY) => D,
        _ => C,
    }
}
```

- `s.opp.last()` is the opponent's previous move, or `None` in the first round.
- The first arm matches only when that move is D. Its guard draws `rng.gen_bool(GENEROSITY)`, which
  is true with probability 1/3. If the draw is false (probability 2/3), the strategy plays D.
- Every other case falls through to `_ => C`: the first round, an opponent's C, and an opponent's D
  that the draw forgave.
- The random number is drawn only after an opponent's D. Rounds after a C use no random numbers.
- `GENEROSITY` is fixed at 1/3; it is not recomputed from the payoffs at run time.

The call path:

1. [`rule_book` in `arena.rs`](../../simulation/src/arena.rs#L19-L34) lists
   `("generous_tft", generous_tft)` in the `history_rules` array
   ([L23](../../simulation/src/arena.rs#L23)) and registers each entry as
   `HistoryRule { name, choose }`.
2. Each round, [`play_with`](../../simulation/src/arena.rs#L124-L125) calls `sim.step()`. In that
   step the socsim-games engine asks each of the two players for a move by calling the rule's
   `decide`, passing a random number generator.
3. [`HistoryRule::decide`](../../simulation/src/strategies.rs#L46-L56) calls
   [`seen(ctx)`](../../simulation/src/strategies.rs#L17-L35), which takes the only opponent, reads its
   history in this match (`opp`, dropping rounds with no recorded action), and reads this player's
   last action (`mine`).
4. `decide` passes the result and the generator to `choose`, which for this strategy is
   `generous_tft`, and returns its move.

## References

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Nowak, M. A., & Sigmund, K. (1992). Tit for tat in heterogeneous populations. *Nature*, 355,
  250–253.

[← Strategies](../strategies.md)
