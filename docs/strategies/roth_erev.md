**English** | [日本語](roth_erev.ja.md)

# Roth–Erev (`roth_erev`)

Roth–Erev keeps a propensity for C and for D, chooses each action with probability proportional to
its propensity, and after each round reinforces the action it played by the payoff it received.

## Origin and properties

Roth and Erev (1995) proposed this reinforcement-learning model to explain how experimental
subjects learn in games, and Erev and Roth (1998) extended it to three parameters: the initial
strength s(1), forgetting φ and experimentation ε. The player does not model the opponent at all;
it only remembers how well its own actions have paid.

It is stochastic. Because it learns only from its own payoffs, its behavior depends on the
opponent's reaction. Against All-C, D pays more (5 against 3), so it drifts toward D; against a
reciprocator, cooperating pays R = 3 while defecting soon pays P = 1, so in principle it can learn
to cooperate. With φ = 0.1 old experience fades quickly, and with ε = 0.2 a fifth of every payoff
goes to the action not played, so neither action's probability approaches 0 or 1 quickly. In
Axelrod's terms it is not nice: it defects with probability 1/2 in round 1.

## How it moves

Let q_C and q_D be the propensities. The probability of each action is

P(C) = q_C / (q_C + q_D).

After a round in which it played action k and received payoff x, every propensity is updated:

| Action | Update |
|---|---|
| The action played, k | q ← (1 − φ) q + (1 − ε) x |
| The other action | q ← (1 − φ) q + ε x |

With two actions, the experimentation term ε x / (M − 1) of Erev and Roth (1998) is just ε x.

Parameters as registered: φ = 0.1, ε = 0.2, and an initial propensity of 10.125 for each action.
The initial value is Erev and Roth's s(1) = 9 times the mean payoff (5 + 3 + 1 + 0) / 4 = 2.25,
divided by the two actions.

**State.** `init_state` creates a `PropensityState` whose `global` entry is a `RothErevState` with
`propensities = [10.125, 10.125]`, `phi = 0.1` and `epsilon = 0.2`. `learn_step` updates the
propensities with its own move and payoff after every round; `decide` reads them.

**Payoff S.** The socsim-games implementation requires every payoff of the game to be strictly
positive and rejects a realized payoff of 0 or less. In matches that include `roth_erev`, the
payoff S given to the strategies is therefore replaced with `f64::MIN_POSITIVE`
([arena.rs#L79-L90](../../simulation/src/arena.rs#L79-L90)). A round with payoff S then only
decays the propensities, which is effectively the same update as with S = 0. The payoffs in the
output are recomputed from the recorded moves with S = 0.

**Examples** (first 16 rounds at seed 0; one sample). P(C) is computed from the update above along
the recorded moves, with S = 0.

Against `all_c`:

| Round | Opponent | Roth–Erev | P(C) | Note |
|---:|:-:|:-:|---:|---|
| 1 | C | C | 0.500 | Equal propensities |
| 2 | C | D | 0.542 | C was reinforced by R = 3 |
| 3 | C | D | 0.471 | D was reinforced by T = 5 |
| 4 | C | D | 0.421 | |
| 5 | C | C | 0.383 | |
| 6 | C | D | 0.426 | |
| 7 | C | C | 0.389 | |
| 8 | C | D | 0.429 | |
| 9 | C | C | 0.394 | |
| 10 | C | C | 0.432 | |
| 11–16 | C | D | 0.466 to 0.328 | Repeated T = 5 push P(C) down |

Against `tit_for_tat`:

| Round | Opponent | Roth–Erev | P(C) | Note |
|---:|:-:|:-:|---:|---|
| 1 | C | C | 0.500 | |
| 2 | C | D | 0.542 | |
| 3 | D | D | 0.471 | TFT copies round 2 |
| 4 | D | D | 0.459 | Mutual defection pays only P = 1 |
| 5 | D | C | 0.447 | |
| 6 | C | D | 0.447 | C against D paid S, so only decay |
| 7 | D | C | 0.392 | |
| 8 | C | D | 0.392 | |
| 9 | D | C | 0.350 | |
| 10 | C | C | 0.350 | |
| 11 | C | D | 0.413 | |
| 12–16 | D | D | 0.369 to 0.339 | |

## Reading the code

`roth_erev` is registered as `RothErev::new(0.1, 0.2, 10.125)`, in the order φ, ε, initial
propensity ([arena.rs#L60-L62](../../simulation/src/arena.rs#L60-L62)).

- The `RothErev` rule and its constructor:
  [rule/learning.rs#L690-L706](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L690-L706).
- `PropensityState`:
  [world/state.rs#L84-L108](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/world/state.rs#L84-L108).
- `RothErevState::new`:
  [learn/roth_erev.rs#L17-L38](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/roth_erev.rs#L17-L38).

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
  ([rule/learning.rs#L738-L746](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L738-L746))
  builds a `RothErevState` for each role and keeps the one for role 0 as `global`.
  `RothErevState::new` checks that the initial propensity is positive and that φ and ε lie in
  [0, 1), and calls `validate_positive_payoffs`
  ([learn/roth_erev.rs#L157-L185](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/roth_erev.rs#L157-L185)),
  which walks through every action profile and fails on any payoff of 0 or less. This is the check
  that S = 0 would fail.
- **`decide`**
  ([rule/learning.rs#L748-L767](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L748-L767))
  calls `choose_action`
  ([learn/roth_erev.rs#L67-L70](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/roth_erev.rs#L67-L70)).
  `selection_probabilities`
  ([learn/roth_erev.rs#L45-L65](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/roth_erev.rs#L45-L65))
  divides the propensities by their maximum and then by their sum, which gives q_k / Σ q.
  `sample_probabilities`
  ([learn/mod.rs#L130-L159](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/mod.rs#L130-L159))
  draws one uniform number and returns C if it is below P(C), D otherwise.
- **`learn_step`**
  ([rule/learning.rs#L769-L784](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L769-L784))
  calls `observe`
  ([rule/learning.rs#L713-L725](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L713-L725))
  for each encounter. It reads the strategy's own move and own payoff and calls `update`
  ([learn/roth_erev.rs#L97-L135](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/roth_erev.rs#L97-L135)),
  which rejects a payoff that is not finite and strictly positive and applies the table above.

Under `PerStep` the rule uses the `global` propensities. The `by_opponent` list holds one set per
opponent only under the `PerEncounter` scope; it stays empty here, and `learn_encounter` returns
without doing anything.

**Differences from the textbook version.** The reinforcement is the raw payoff x, and the
implementation requires it to be strictly positive; this is why S is replaced with
`f64::MIN_POSITIVE` in its matches. The initial propensity is given directly as one number
(10.125) rather than as s(1) and the mean payoff; the value used here is the one derived from
s(1) = 9 above.

## References

- Erev, I., & Roth, A. E. (1998). Predicting how people play games: Reinforcement learning in
  experimental games with unique, mixed strategy equilibria. *American Economic Review*, 88(4),
  848–881.
- Roth, A. E., & Erev, I. (1995). Learning in extensive-form games: Experimental data and simple
  dynamic models in the intermediate term. *Games and Economic Behavior*, 8(1), 164–212.

[← Strategies](../strategies.md)
