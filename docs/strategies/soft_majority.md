**English** | [日本語](soft_majority.ja.md)

# Soft Majority (`soft_majority`)

Soft Majority plays the opponent's majority move over the whole match so far. It cooperates as
long as the opponent has cooperated at least as often as it has defected, and defects otherwise.
A tie counts as cooperation, so it starts with C.

## Origin and properties

Soft Majority is one of the classical strategies of Mathieu and Delahaye (2017), where it is called
`soft_majo`: it begins by cooperating and cooperates as long as the number of times the opponent
has cooperated is greater than or equal to the number of times it has defected. The "soft" refers
to the tie going to C; its counterpart Hard Majority defects on a tie. The implementation here
follows that description.

In the terms of Axelrod (1984):

- **Nice**: it is never the first to defect. Until the opponent defects, the opponent's D count is
  0, so the condition for C always holds.
- **Retaliatory, but slowly**: it reacts to the opponent's overall record, not to the last move.
  An opponent that has built up a lead of C can defect several times without being answered.
- **Forgiving, but with a long memory**: it returns to C once the opponent's C have caught up with
  its D, which after a long run of D takes as many C.
- **Less clear than TFT**: its move depends on the counts over the whole history, not on anything
  visible in the last round.

It is deterministic.

Its strength is stability: against mostly cooperative opponents a single D does not set off an
echo of retaliations, and against persistent defectors it settles into D. Its weakness is that it
can be exploited by an opponent that keeps its D count just below its C count. `alternator` keeps
the counts tied or one C ahead, so Soft Majority cooperates in every round while being defected on
every other round.

## How it moves

**No state.** Soft Majority is stateless: each round it recomputes its move from
`Seen { opp, mine }` (the opponent's past moves, oldest first, and its own previous move), and its
`init_state` returns `Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))`. It only uses
`opp`.

**The rule.** Count the opponent's C and D in `opp`. Play C if C ≥ D, and D otherwise. In round 1
both counts are 0, so it plays C.

**Traces.** These are the first 16 rounds at seed 0 of the tournament, as recorded by the program.
The last column shows the opponent's C : D counts that Soft Majority sees before the round.

Against `fictitious_play`, which defects from round 2 on:

| Round | Opponent | Soft Majority | Opponent C : D seen |
|---:|:-:|:-:|---|
| 1 | C | C | 0 : 0 |
| 2 | D | C | 1 : 0 |
| 3 | D | C | 1 : 1, a tie goes to C |
| 4 | D | D | 1 : 2 |
| 5–16 | D | D | D stays in the majority |

Against `suspicious_tft`, the two alternate out of phase:

| Round | Opponent | Soft Majority | Opponent C : D seen |
|---:|:-:|:-:|---|
| 1 | D | C | 0 : 0 |
| 2 | C | D | 0 : 1 |
| 3 | D | C | 1 : 1 |
| 4 | C | D | 1 : 2 |
| 5 | D | C | 2 : 2 |
| 6 | C | D | 2 : 3 |
| 7–16 | D, C, … | C, D, … | the pattern repeats |

Against `alternator`, the counts never favor D:

| Round | Opponent | Soft Majority | Opponent C : D seen |
|---:|:-:|:-:|---|
| 1 | C | C | 0 : 0 |
| 2 | D | C | 1 : 0 |
| 3 | C | C | 1 : 1 |
| 4 | D | C | 2 : 1 |
| 5 | C | C | 2 : 2 |
| 6–16 | D, C, … | C | always a tie or one C ahead |

Against `extort2`, early cooperation buys D that go unanswered:

| Round | Opponent | Soft Majority | Opponent C : D seen |
|---:|:-:|:-:|---|
| 1–4 | C | C | 0 : 0 to 3 : 0 |
| 5 | D | C | 4 : 0 |
| 6 | C | C | 4 : 1 |
| 7 | D | C | 5 : 1 |
| 8 | C | C | 5 : 2 |
| 9–16 | C | C | |

## Reading the code

- **Registration.** Soft Majority is one of the seven stateless strategies listed in
  [`arena.rs#L20-L28`](../../simulation/src/arena.rs#L20-L28) (the entry
  `("soft_majority", soft_majority)` is on [L27](../../simulation/src/arena.rs#L27)). The loop at
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
- **The strategy.** [`strategies.rs#L113-L122`](../../simulation/src/strategies.rs#L113-L122). It
  counts the C in `s.opp`, takes the D as the rest, and returns C when `cooperations >= defections`.
  It recounts the whole history each round rather than keeping running totals. It does not use its
  random-number argument.

**When the engine calls each method.** Each match is one call of
[`play_with`](../../simulation/src/arena.rs#L73-L140). It builds a fresh world with `build_world`
([`arena.rs#L100-L111`](../../simulation/src/arena.rs#L100-L111)), which calls `init_state` once
per agent and stores each agent's `Policy` in the `GameWorld`. The match is then 200 calls of
`sim.step()` ([`arena.rs#L124-L138`](../../simulation/src/arena.rs#L124-L138)). In each step the
engine calls `decide` once for every encounter of the agent (one per step here), and then
`learn_step` once per agent. Because Soft Majority keeps nothing in its state, the counts come from
the history that `decide` reads each round.

**Test.** [`tests.rs#L162-L173`](../../simulation/src/tests.rs#L162-L173) calls the function on
fixed histories: C on an empty history, C after (C, D) (a tie), and D after (C, D, D).

## References

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Mathieu, P., & Delahaye, J.-P. (2017). New winning strategies for the iterated prisoner's dilemma.
  *Journal of Artificial Societies and Social Simulation*, 20(4), 12.

[← Strategies](../strategies.md)
