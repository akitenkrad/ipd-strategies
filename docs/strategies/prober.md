**English** | [日本語](prober.ja.md)

# Prober (`prober`)

Prober opens with D, C, C to test the opponent. If the opponent cooperated in both rounds 2 and 3,
that is, did not answer the opening D, Prober treats it as exploitable and defects for the rest of
the match. Otherwise it plays Tit for Tat (TFT) from round 4 on.

## Origin and properties

Prober is one of the classical strategies of Mathieu and Delahaye (2017), who describe it as playing
D, C, C and then always defecting if the opponent cooperated in moves 2 and 3, and playing TFT
otherwise. The implementation here follows that description.

In the terms of Axelrod (1984):

- **Not nice**: it defects in round 1.
- **Retaliatory** once it has switched to TFT: it answers every D.
- **Forgiving only in the TFT branch**: there it returns to C as soon as the opponent does. Against
  an opponent that passed the test as exploitable, it never returns to C.
- **Fairly clear**: three fixed opening moves, then either TFT or permanent D.

It is deterministic.

Its strength is that it exploits opponents that do not punish a single D, such as `all_c` or
Tit for Two Tats, which only answers two D in a row. Against a retaliating opponent the test costs
little: TFT answers the opening D once, and both then cooperate. Its weakness is the opening D
itself. An opponent that never forgives, such as `grim`, defects forever after round 1, and Prober,
now in the TFT branch, copies those D, so the match ends in mutual defection.

## How it moves

**No state.** Prober is stateless: each round it recomputes its move from `Seen { opp, mine }`
(the opponent's past moves, oldest first, and its own previous move), and its `init_state` returns
`Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))`. It only uses `opp`.

| Round | Move |
|---|---|
| 1 | D |
| 2, 3 | C, whatever the opponent does |
| 4 and later | D if the opponent played C in both rounds 2 and 3; otherwise the opponent's previous move (TFT) |

The opponent's moves in rounds 2 and 3 are its answers to Prober's D in round 1 and C in round 2.
The test is re-evaluated in every round from round 4 on, but since rounds 2 and 3 never change, the
outcome is the same every time, so no state is needed to remember it. An opponent that plays D in
round 2 or in round 3 (or both) sends Prober into the TFT branch.

**Traces.** These are the first 16 rounds at seed 0 of the tournament, as recorded by the program.

Against `all_c`, the test finds an exploitable opponent:

| Round | Opponent | Prober | Note |
|---:|:-:|:-:|---|
| 1 | C | D | opening |
| 2 | C | C | opening |
| 3 | C | C | opening |
| 4 | C | D | opponent played C in rounds 2 and 3: D forever |
| 5–16 | C | D | |

Against `tit_for_tat`, the opponent answers the opening D, so Prober plays TFT:

| Round | Opponent | Prober | Note |
|---:|:-:|:-:|---|
| 1 | C | D | opening |
| 2 | D | C | TFT answers round 1 |
| 3 | C | C | opening |
| 4 | C | C | opponent played D in round 2: TFT, copies round 3 |
| 5–16 | C | C | mutual cooperation |

Against `tit_for_two_tats`, the single opening D is not answered:

| Round | Opponent | Prober | Note |
|---:|:-:|:-:|---|
| 1 | C | D | opening |
| 2 | C | C | TF2T ignores a single D |
| 3 | C | C | opening |
| 4 | C | D | opponent played C in rounds 2 and 3: D forever |
| 5 | C | D | |
| 6 | D | D | TF2T answers the D of rounds 4 and 5, too late |
| 7–16 | D | D | mutual defection |

Against `grim`, the opening D is never forgiven:

| Round | Opponent | Prober | Note |
|---:|:-:|:-:|---|
| 1 | C | D | opening |
| 2 | D | C | Grim is triggered |
| 3 | D | C | opening |
| 4 | D | D | opponent played D in rounds 2 and 3: TFT, copies round 3 |
| 5–16 | D | D | mutual defection |

## Reading the code

- **Registration.** Prober is one of the seven stateless strategies listed in
  [`arena.rs#L20-L28`](../../simulation/src/arena.rs#L20-L28) (the entry `("prober", prober)` is on
  [L26](../../simulation/src/arena.rs#L26)). The loop at
  [`arena.rs#L30-L34`](../../simulation/src/arena.rs#L30-L34) wraps each function in a
  `HistoryRule { name, choose }` and registers it in the rule book built by
  [`rule_book`](../../simulation/src/arena.rs#L19-L64).
- **The wrapper.** [`strategies.rs#L17-L56`](../../simulation/src/strategies.rs#L17-L56) defines
  `Seen` ([L17-L21](../../simulation/src/strategies.rs#L17-L21)), the function `seen` that reads it
  from the decision context ([L23-L35](../../simulation/src/strategies.rs#L23-L35)), the function
  type `Choose` ([L37-L38](../../simulation/src/strategies.rs#L37-L38)) and `HistoryRule`
  ([L40-L56](../../simulation/src/strategies.rs#L40-L56)). Its `init_state`
  ([L50-L52](../../simulation/src/strategies.rs#L50-L52)) returns the stateless policy, and its
  `decide` ([L53-L55](../../simulation/src/strategies.rs#L53-L55)) calls
  `(self.choose)(&seen(ctx)?, rng)`. It defines no `learn_step`, so the default one, which does
  nothing, is used.
- **The strategy.** [`strategies.rs#L103-L111`](../../simulation/src/strategies.rs#L103-L111). It
  branches on `s.opp.len()`, the number of rounds already played: 0 → D, 1 or 2 → C, and otherwise
  it checks `s.opp[1]` and `s.opp[2]`, the opponent's moves in rounds 2 and 3 (indices start at 0).
  It does not use its random-number argument.

**When the engine calls each method.** Each match is one call of
[`play_with`](../../simulation/src/arena.rs#L73-L140). It builds a fresh world with `build_world`
([`arena.rs#L100-L111`](../../simulation/src/arena.rs#L100-L111)), which calls `init_state` once
per agent and stores each agent's `Policy` in the `GameWorld`. The match is then 200 calls of
`sim.step()` ([`arena.rs#L124-L138`](../../simulation/src/arena.rs#L124-L138)). In each step the
engine calls `decide` once for every encounter of the agent (one per step here), and then
`learn_step` once per agent. Because Prober keeps nothing in its state, everything it needs comes
from the history that `decide` reads each round.

**Test.** [`tests.rs#L151-L159`](../../simulation/src/tests.rs#L151-L159) calls the function on
fixed histories: D on an empty history, C after one and two rounds, D after (D, C, C), and TFT after
(D, D, C) and (D, C, D).

## References

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Mathieu, P., & Delahaye, J.-P. (2017). New winning strategies for the iterated prisoner's dilemma.
  *Journal of Artificial Societies and Social Simulation*, 20(4), 12.

[← Strategies](../strategies.md)
