**English** | [日本語](uniform_random.ja.md)

# Uniform Random (`uniform_random`)

Uniform Random plays C or D with probability 1/2 each in every round, independently of the history.

## Origin and properties

A rule that cooperates and defects at random with equal probability was included as RANDOM in
Axelrod's first computer tournament, where it served as a baseline and finished last
(Axelrod, 1980). It is stochastic and has no memory.

In Axelrod's terms it is neither nice (it defects first with probability 1/2 in round 1) nor
retaliatory nor forgiving, because it does not react to the opponent at all. Its expected payoff
per round depends only on how often the opponent cooperates: against an opponent that cooperates
with probability p it receives on average (3p + 0(1 − p) + 5p + 1(1 − p)) / 2 = (1 + 7p) / 2.
Reciprocators answer its defections, so it mostly faces a mix of C and D.

## How it moves

Every round, C and D each with probability 1/2. It never looks at the opponent.

**State.** None. `init_state` returns `Stateless`, and the rule has no `learn_step` (the trait's
default does nothing).

**Examples** (first 16 rounds at seed 0; one sample).

Against `all_c`:

| Round | Opponent | Uniform Random | Note |
|---:|:-:|:-:|---|
| 1 | C | D | |
| 2 | C | D | |
| 3 | C | C | |
| 4–16 | C | D C C D D D C C D C D D C | 7 D and 6 C in these 13 rounds |

Against `tit_for_tat`:

| Round | Opponent | Uniform Random | Note |
|---:|:-:|:-:|---|
| 1 | C | D | |
| 2 | D | D | TFT copies round 1 |
| 3 | D | C | |
| 4 | C | D | |
| 5–16 | D C C D D D C C D C D D | C C D D D C C D C D D C | TFT's row is Uniform Random's row shifted by one round |

At seed 0 the random sequence `DDCDCCDDDCCDCDDC` is the same against both opponents, because
neither opponent draws random numbers and so the random stream is consumed only by Uniform Random.

## Reading the code

`uniform_random` is registered as the unit struct `UniformRandom`
([arena.rs#L54-L56](../../simulation/src/arena.rs#L54-L56)); it has no parameters.

- The `UniformRandom` rule:
  [rule/builtin.rs#L54-L102](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L54-L102).
- `Stateless`:
  [world/state.rs#L36-L38](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/world/state.rs#L36-L38).

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
  ([rule/builtin.rs#L68-L70](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L68-L70))
  returns `Stateless`.
- **`decide`**
  ([rule/builtin.rs#L72-L89](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L72-L89))
  looks up the number of actions of its role and returns `rng.gen_range(0..n_actions)`, here
  `gen_range(0..2)`. The random number generator is the one the engine passes to `decide` for the
  step.
- **`learn_step`** is not implemented, so the default of the `DecisionRule` trait, which does
  nothing, is used
  ([rule/mod.rs#L118-L124](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/mod.rs#L118-L124)).

## References

- Axelrod, R. (1980). Effective choice in the prisoner's dilemma. *Journal of Conflict Resolution*,
  24(1), 3–25.

[← Strategies](../strategies.md)
