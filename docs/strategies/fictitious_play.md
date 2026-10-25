**English** | [日本語](fictitious_play.ja.md)

# Fictitious Play (`fictitious_play`)

Fictitious Play counts how often the opponent has played C and D and plays a best response to those
frequencies, as if the opponent were drawing its moves from a fixed distribution.

## Origin and properties

Brown (1951) proposed fictitious play as an iterative method for solving games: each player treats
the empirical distribution of the opponent's past moves as a mixed strategy and best-responds to
it. It became one of the basic models of belief-based learning in games (Fudenberg & Levine, 1998).
It is deterministic here, because ties are broken by the lowest action index.

Fictitious play treats each round as a one-shot game. In the prisoner's dilemma D is the best
response to every belief, because it pays more than C against both C (5 > 3) and D (1 > 0). As
soon as one opponent move has been counted, Fictitious Play therefore defects, whatever the
opponent does. It does not model the fact that the opponent reacts to its own moves, so it cannot
see that cooperating could pay off against a reciprocator. In Axelrod's terms it is not nice: it
defects in round 2 without having been provoked.

## How it moves

The value of each action is the sum, over the opponent's actions, of the count times the payoff:

value(a) = n_C · u(a, C) + n_D · u(a, D)

where n_C and n_D are the numbers of times the opponent has played C and D so far. Fictitious Play
plays the action with the higher value; on a tie it plays the lower action index, which is C.

| Round | Counts | value(C), value(D) | Move |
|---|---|---|:-:|
| 1 | n_C = n_D = 0 | 0, 0 (tie) | C |
| t ≥ 2 | n_C + n_D = t − 1 ≥ 1 | 3 n_C, 5 n_C + n_D | D |

value(D) − value(C) = 2 n_C + n_D is positive once any move has been counted, so in this game the
strategy reduces to "C in round 1, D from round 2 on".

**State.** `init_state` creates a `BeliefState` whose `global` entry is a `FictitiousBelief` with
`counts = [0.0, 0.0]` (no pseudo-counts) and the tie rule `LowestIndex`. `learn_step` adds 1 to the
count of the opponent's move after every round; `decide` reads the counts.

**Examples** (first 16 rounds at seed 0).

Against `all_c`:

| Round | Opponent | Fictitious Play | Note |
|---:|:-:|:-:|---|
| 1 | C | C | No observations yet; the tie goes to C |
| 2 | C | D | n_C = 1: value(C) = 3, value(D) = 5 |
| 3–16 | C | D | All-C is exploited from now on |

Against `tit_for_tat`:

| Round | Opponent | Fictitious Play | Note |
|---:|:-:|:-:|---|
| 1 | C | C | |
| 2 | C | D | |
| 3 | D | D | TFT copies the D of round 2 |
| 4–16 | D | D | Mutual defection |

## Reading the code

`fictitious_play` is registered as `FictitiousPlay::new()`
([arena.rs#L51-L53](../../simulation/src/arena.rs#L51-L53)), which means zero initial pseudo-counts
and lowest-index tie breaking.

- The `FictitiousPlay` rule and its constructor:
  [rule/learning.rs#L379-L393](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L379-L393).
- `BeliefState`:
  [world/state.rs#L58-L82](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/world/state.rs#L58-L82).
- `FictitiousBelief`:
  [learn/fictitious.rs#L9-L47](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/fictitious.rs#L9-L47).

**When the engine calls what.** `init_state` runs once per agent while `build_world` sets up the
match
([assemble.rs#L719-L727](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L719-L727)),
and the state it returns is stored for that agent in `GameWorld`. Matches here are built with
`PlayOptions::new()` ([arena.rs#L112-L121](../../simulation/src/arena.rs#L112-L121)), whose
decision scope is `PerStep`
([assemble.rs#L261-L271](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L261-L271)).
The engine therefore calls `decide` once per agent per step
([mechanism/decide.rs#L248-L250](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L248-L250));
with one encounter per step, that is once per round. After all encounters of the step it calls
`learn_step` once
([mechanism/decide.rs#L431-L438](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L431-L438)).
Both receive the agent's state as `&mut`.

- **`init_state`**
  ([rule/learning.rs#L460-L472](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L460-L472))
  requires exactly two roles and builds the initial belief with `initial`
  ([rule/learning.rs#L413-L429](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L413-L429)),
  which uses a zero count for each opponent action when no initial counts are given.
- **`decide`**
  ([rule/learning.rs#L474-L495](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L474-L495))
  calls `best_response`
  ([learn/fictitious.rs#L75-L116](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/fictitious.rs#L75-L116)).
  It computes value(a) from the raw counts (not normalized, which gives the same best response),
  collects every action whose value equals the maximum exactly, and passes them to `resolve_tie`
  ([learn/mod.rs#L161-L178](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/mod.rs#L161-L178)).
  With `LowestIndex` the first candidate is taken and no random number is drawn.
- **`learn_step`**
  ([rule/learning.rs#L497-L512](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L497-L512))
  takes the opponent's move from each encounter and calls `observe`
  ([learn/fictitious.rs#L164-L182](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/fictitious.rs#L164-L182)),
  which adds 1 to that action's count.

Under `PerStep` the rule uses the `global` belief. The `by_opponent` list holds one belief per
opponent only under the `PerEncounter` scope; it stays empty here, and `learn_encounter` returns
without doing anything.

**Differences from the textbook version.** Textbook fictitious play often starts from a prior
(positive initial counts). Here the counts start at zero, so the first move is decided by the tie
rule, and the tie rule picks C because C is action `0`. With a prior, the first move would also be
D, since D is the best response to every belief in this game.

## References

- Brown, G. W. (1951). Iterative solution of games by fictitious play. In T. C. Koopmans (Ed.),
  *Activity analysis of production and allocation* (pp. 374–376). Wiley.
- Fudenberg, D., & Levine, D. K. (1998). *The theory of learning in games*. MIT Press.

[← Strategies](../strategies.md)
