**English** | [日本語](logit.ja.md)

# Logit Choice (`logit`)

Logit Choice estimates the expected payoff of C and D from the frequencies of the opponent's past
moves, and chooses at random with probabilities proportional to exp(λ × expected payoff), with
λ = 1.

## Origin and properties

The logit rule turns payoff differences into choice probabilities, as in discrete-choice models.
McKelvey and Palfrey (1995) used it to define the logit quantal response equilibrium, in which
every player logit-responds to the others' actual mixed strategies. Applied repeatedly to the
empirical frequencies of the opponent's past moves, as here, it is a smoothed version of fictitious
play (stochastic fictitious play; Fudenberg & Levine, 1998). λ is the inverse temperature: λ = 0
gives uniform random choice, and λ → ∞ gives the best response of fictitious play.

It is stochastic. As with fictitious play, D pays more than C against any belief in the prisoner's
dilemma, so Logit Choice leans toward D from round 2 on, but it keeps cooperating now and then. It
does not model the opponent's reaction to its own moves, so it does not cooperate more against
reciprocators. In Axelrod's terms it is not nice.

## How it moves

Let p be the share of C among the opponent's past moves. The estimated payoffs are

- E[C] = 3p + 0(1 − p) = 3p
- E[D] = 5p + 1(1 − p) = 1 + 4p

and the probability of C is

P(C) = exp(λ E[C]) / (exp(λ E[C]) + exp(λ E[D])) = 1 / (1 + exp(λ (1 + p))).

| Round | Information | P(C) with λ = 1 |
|---|---|---|
| 1 | No observation: both estimates are 0 | 1/2 |
| t ≥ 2 | p from the opponent's moves in rounds 1 to t − 1 | between 1/(1 + e²) ≈ 0.119 (p = 1) and 1/(1 + e) ≈ 0.269 (p = 0) |

So after round 1 it cooperates with probability 12% to 27%, and less often the more the opponent
cooperates.

**State.** `init_state` creates an `EstimateState` whose `global` entry is a `LogitState` with
`empirical_counts = [0, 0]`, `lambda = 1.0` and the estimate mode `EmpiricalFrequency`.
`learn_step` adds 1 to the count of the opponent's move after every round; `decide` reads the
counts.

**Examples** (first 16 rounds at seed 0; one sample). P(C) is computed from the formula above
along the recorded moves.

Against `all_c`:

| Round | Opponent | Logit | P(C) | Note |
|---:|:-:|:-:|---:|---|
| 1 | C | C | 0.500 | No observation |
| 2–8 | C | D | 0.119 | p = 1 from now on |
| 9 | C | C | 0.119 | |
| 10 | C | C | 0.119 | |
| 11–16 | C | D | 0.119 | |

Against `tit_for_tat`:

| Round | Opponent | Logit | P(C) | Note |
|---:|:-:|:-:|---:|---|
| 1 | C | C | 0.500 | |
| 2 | C | D | 0.119 | |
| 3 | D | D | 0.119 | TFT copies round 2 |
| 4–6 | D | D | 0.159 to 0.198 | p falls, so P(C) rises |
| 7 | D | C | 0.209 | |
| 8 | C | D | 0.217 | TFT copies the C |
| 9 | D | C | 0.202 | |
| 10 | C | C | 0.209 | |
| 11 | C | D | 0.198 | |
| 12–16 | D | D | 0.189 to 0.209 | |

## Reading the code

`logit` is registered as `LogitChoice::new(1.0)`
([arena.rs#L57-L59](../../simulation/src/arena.rs#L57-L59)), that is λ = 1 with the empirical
frequency estimate. The payoffs enter on their raw scale (R = 3, T = 5, S = 0, P = 1); λ is not
rescaled to them.

- The `LogitChoice` rule and its constructor:
  [rule/learning.rs#L557-L572](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L557-L572).
- `EstimateState`:
  [world/state.rs#L110-L134](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/world/state.rs#L110-L134).
- `LogitState`:
  [learn/logit.rs#L20-L50](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/logit.rs#L20-L50).

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
  ([rule/learning.rs#L607-L615](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L607-L615))
  builds a `LogitState` for each role to validate λ and the game, and keeps the one for role 0 as
  `global`.
- **`decide`**
  ([rule/learning.rs#L617-L638](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L617-L638))
  calls `choose_action`
  ([learn/logit.rs#L169-L177](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/logit.rs#L169-L177)).
  `expected_payoffs`
  ([learn/logit.rs#L87-L141](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/logit.rs#L87-L141))
  averages the payoffs over the observed frequencies and returns zeros while nothing has been
  observed. `selection_probabilities`
  ([learn/logit.rs#L143-L167](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/logit.rs#L143-L167))
  subtracts the maximum estimate before exponentiating, for numerical stability, and normalizes.
  `sample_probabilities`
  ([learn/mod.rs#L130-L159](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/mod.rs#L130-L159))
  draws one uniform number and returns C if it is below P(C), D otherwise.
- **`learn_step`**
  ([rule/learning.rs#L640-L655](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L640-L655))
  takes the opponent's move from each encounter and calls `observe`
  ([learn/logit.rs#L71-L85](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/logit.rs#L71-L85)),
  which adds 1 to that action's count and also records it as `last_opponent` (used only by the
  `LastOpponent` mode).

Under `PerStep` the rule uses the `global` estimate. The `by_opponent` list holds one estimate per
opponent only under the `PerEncounter` scope; it stays empty here, and `learn_encounter` returns
without doing anything.

**Differences from the textbook version.** There is no prior: the first round is uniform because
all estimates are zero, not because of a prior belief. In matches against `roth_erev` the payoff S
is replaced with `f64::MIN_POSITIVE` (see [Strategies](../strategies.md)); this changes E[C] by a
negligible amount.

## References

- Fudenberg, D., & Levine, D. K. (1998). *The theory of learning in games*. MIT Press.
- McKelvey, R. D., & Palfrey, T. R. (1995). Quantal response equilibria for normal form games.
  *Games and Economic Behavior*, 10(1), 6–38.

[← Strategies](../strategies.md)
