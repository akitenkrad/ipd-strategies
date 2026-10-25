**English** | [日本語](extort2.ja.md)

# Extort-2 (`extort2`)

Extort-2 cooperates in the first round. After that it cooperates with a probability that depends on
the previous round's pair of moves: 8/9 after (C, C), 1/2 after (C, D), 1/3 after (D, C) and 0 after
(D, D), where the first letter is its own move. Over a long match this forces its payoff above P to
be twice the opponent's.

## Origin and properties

Extort-2 is a zero-determinant (ZD) strategy of Press and Dyson (2012), the extortionate one with
χ = 2 and φ = 1/18. The name Extort-2 and these four probabilities were used by Stewart and Plotkin
(2012), who entered it in a tournament alongside classic strategies.

A memory-one strategy for player X is a vector p = (p1, p2, p3, p4) of cooperation probabilities
after the previous outcomes (C, C), (C, D), (D, C), (D, D), seen from X. Press and Dyson show that if

p̃ = (p1 − 1, p2 − 1, p3, p4) = φ [(S_X − P) − χ (S_Y − P)],

where S_X = (R, S, T, P) and S_Y = (R, T, S, P) are the two players' payoffs in those four outcomes,
then the long-run average payoffs s_X and s_Y satisfy s_X − P = χ (s_Y − P), whatever Y does. With
R = 3, S = 0, T = 5, P = 1, χ = 2 and φ = 1/18:

| Outcome (X, Y) | S_X − P | S_Y − P | p̃ = φ [(S_X − P) − 2 (S_Y − P)] | p |
|---|---:|---:|---:|---:|
| (C, C) | 2 | 2 | −1/9 | 8/9 |
| (C, D) | −1 | 4 | −1/2 | 1/2 |
| (D, C) | 4 | −1 | 1/3 | 1/3 |
| (D, D) | 0 | 0 | 0 | 0 |

So the strategy enforces s_X − P = 2 (s_Y − P).

In Axelrod's (1984) terms it is:

- **not nice**: even after mutual cooperation it defects with probability 1/9.
- **retaliatory**: after being exploited, (C, D), it defects with probability 1/2, and after mutual
  defection it always defects.
- **only partly forgiving**: after its own D against the opponent's C it returns to C only with
  probability 1/3, and mutual defection is absorbing, because p4 = 0.
- **stochastic**: it draws a random number in every round after the first.

Its strength is head-to-head: whenever the opponent earns at least P, s_X − P = 2 (s_Y − P) means
Extort-2 earns at least as much as the opponent, and an opponent who raises its own payoff raises
Extort-2's twice as much. Its weakness is that it also extorts players like itself. Two Extort-2
players each enforce the relation on the other, which only holds when both earn P; mutual
defection, once reached, never ends.

## How it moves

- In the first round, play C.
- Otherwise look at the previous round's (own move, opponent's move) and cooperate with
  probability 8/9, 1/2, 1/3 or 0 for (C, C), (C, D), (D, C), (D, D).

The opening C is a choice of this implementation. The ZD construction only fixes the four
conditional probabilities; the first move does not affect the long-run relation.

It is the only history-based strategy in this repository that reads its own previous move. It is
still **stateless**: it keeps
nothing between rounds, and each round it recomputes the move from the match history, which arrives
as `Seen { opp, mine }` (`opp` is the opponent's moves, oldest first; `mine` is its own previous
move). Its `init_state` returns `Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))`, the
empty state shared by all history-based strategies in this repository.

The traces below are the first 16 rounds at seed 0 from the tournament output. Because the strategy
is stochastic, each trace is one sample; other seeds give other sequences.

Against `all_c`, it defects now and then and is never punished:

| Round | Opponent (`all_c`) | `extort2` | Note |
|---:|:-:|:-:|---|
| 1 | C | C | First round: C |
| 2–4 | C | C | After (C, C): C with probability 8/9 |
| 5 | C | D | After (C, C): drew D |
| 6 | C | C | After (D, C): drew C (probability 1/3) |
| 7 | C | D | After (C, C): drew D |
| 8 | C | C | After (D, C): drew C |
| 9–16 | C | C | After (C, C): C |

Against itself, the two players drift into mutual defection and stay there. "Player 0" and
"Player 1" are the two copies; the note gives each one's previous (own, opponent's) pair:

| Round | Player 0 | Player 1 | Note |
|---:|:-:|:-:|---|
| 1 | C | C | First round |
| 2 | C | C | Both after (C, C) |
| 3 | C | D | Both after (C, C); player 1 drew D |
| 4 | C | D | Player 0 after (C, D), player 1 after (D, C) |
| 5 | C | D | Same pairs |
| 6 | C | C | Same pairs; player 1 drew C |
| 7 | C | C | Both after (C, C) |
| 8 | C | C | Both after (C, C) |
| 9 | C | D | Both after (C, C); player 1 drew D |
| 10 | D | D | Player 0 after (C, D) drew D; player 1 after (D, C) drew D |
| 11–16 | D | D | Both after (D, D): probability 0 |

Against `all_d`, it plays C, C and then D from round 3 on: after (C, D) it drew C in round 2 and D in
round 3, and (D, D) never cooperates again.

## Reading the code

The function is [`extort2` in `strategies.rs`](../../simulation/src/strategies.rs#L90-L101):

```rust
pub fn extort2(s: &Seen, rng: &mut SimRng) -> usize {
    let cooperate = match (s.mine, s.opp.last().copied()) {
        (None, _) => return C,
        (Some(C), Some(C)) => 8.0 / 9.0,
        (Some(C), Some(D)) => 1.0 / 2.0,
        (Some(D), Some(C)) => 1.0 / 3.0,
        (Some(D), Some(D)) => 0.0,
        _ => 0.0,
    };
    usize::from(!rng.gen_bool(cooperate))
}
```

- The `match` looks at the pair (own previous move, opponent's previous move). `s.mine` comes from
  this player's own history in [`seen`](../../simulation/src/strategies.rs#L33).
- `(None, _) => return C`: no previous move of its own means the first round. It returns C at once,
  without drawing a random number.
- The next four arms give the cooperation probability for (C, C), (C, D), (D, C), (D, D). `C` and `D`
  are the constants 0 and 1, used here as patterns.
- `_ => 0.0` covers what is left: an own previous move with no opponent move, which does not happen
  in a match where both players move every round. It sets the probability to 0, that is, D.
- `rng.gen_bool(cooperate)` is true with probability `cooperate`. `!` turns "cooperate" into
  "defect", and `usize::from` turns `false` into 0 = C and `true` into 1 = D.

The call path:

1. [`rule_book` in `arena.rs`](../../simulation/src/arena.rs#L19-L34) lists `("extort2", extort2)` in
   the `history_rules` array ([L25](../../simulation/src/arena.rs#L25)) and registers each entry as
   `HistoryRule { name, choose }`.
2. Each round, [`play_with`](../../simulation/src/arena.rs#L124-L125) calls `sim.step()`. In that
   step the socsim-games engine asks each of the two players for a move by calling the rule's
   `decide`, passing a random number generator.
3. [`HistoryRule::decide`](../../simulation/src/strategies.rs#L46-L56) calls
   [`seen(ctx)`](../../simulation/src/strategies.rs#L17-L35), which takes the only opponent, reads its
   history in this match (`opp`, dropping rounds with no recorded action), and reads this player's
   last action (`mine`).
4. `decide` passes the result and the generator to `choose`, which for this strategy is `extort2`,
   and returns its move.

## References

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Press, W. H., & Dyson, F. J. (2012). Iterated Prisoner's Dilemma contains strategies that dominate
  any evolutionary opponent. *Proceedings of the National Academy of Sciences*, 109(26),
  10409–10413.
- Stewart, A. J., & Plotkin, J. B. (2012). Extortion and cooperation in the Prisoner's Dilemma.
  *Proceedings of the National Academy of Sciences*, 109(26), 10134–10135.

[← Strategies](../strategies.md)
