**English** | [日本語](tit_for_tat.ja.md)

# Tit for Tat (`tit_for_tat`)

Tit for Tat cooperates in the first round and then plays whatever the opponent played in the
previous round.

## Origin and properties

Tit for Tat was submitted by Anatol Rapoport to Axelrod's first computer tournament and won it
(Axelrod, 1980); it also won the second tournament (Axelrod, 1984). Axelrod explains its success by
four properties:

- **Nice**: it never defects first.
- **Retaliatory (provocable)**: it answers a defection with a defection in the next round.
- **Forgiving**: it returns to cooperation as soon as the opponent does.
- **Clear**: its behavior is easy for the opponent to recognize.

It is deterministic. It never scores more than its opponent in a match, so it does well by
eliciting cooperation rather than by beating anyone. Its known weakness is echoing: once a single
defection enters a match between two TFT-like players, the two sides alternate C and D
indefinitely. Without noise this happens here only against strategies that defect first or defect
at random, such as `suspicious_tft` and `joss`.

## How it moves

| Round | Move |
|---|---|
| 1 | C |
| t ≥ 2 | The opponent's move in round t − 1 |

**State.** `init_state` creates a `TftState { last_opponent: None }`. `learn_step` writes the
opponent's move into `last_opponent` after every round. `decide` returns `last_opponent`, or C
while it is still `None`.

**Examples** (first 16 rounds at seed 0).

Against `joss` (one sample; Joss plays a random D where TFT would play C):

| Round | Opponent | TFT | Note |
|---:|:-:|:-:|---|
| 1–9 | C | C | Mutual cooperation |
| 10 | D | C | Joss's random defection |
| 11 | C | D | TFT retaliates; Joss copies TFT's C |
| 12 | D | C | Joss copies TFT's D |
| 13–16 | C, D, C, D | D, C, D, C | The echo continues |

Against `suspicious_tft` (deterministic; it opens with D and then copies):

| Round | Opponent | TFT | Note |
|---:|:-:|:-:|---|
| 1 | D | C | |
| 2 | C | D | Each copies the other's previous move |
| 3–16 | D, C, … | C, D, … | The two sides alternate in opposite phase for the rest of the match |

## Reading the code

`tit_for_tat` is registered as `TitForTat::new(C)`
([arena.rs#L48-L50](../../simulation/src/arena.rs#L48-L50)). socsim-games also has an automaton
kind `AutomatonKind::Tft` with the same moves; this repository uses the stand-alone rule, whose
state is only the opponent's last move.

- The `TitForTat` rule:
  [rule/builtin.rs#L104-L117](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L104-L117).
- `TftState`:
  [world/state.rs#L40-L45](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/world/state.rs#L40-L45).

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
  ([rule/builtin.rs#L129-L153](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L129-L153))
  requires exactly two roles with two actions each and a valid cooperation action, then returns
  `TftState::default()`.
- **`decide`**
  ([rule/builtin.rs#L155-L163](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L155-L163))
  is one line: `state.last_opponent.unwrap_or(self.cooperate)`. It draws no random number.
- **`learn_step`**
  ([rule/builtin.rs#L165-L194](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L165-L194))
  goes through the step's encounters, finds the opponent's role (1 for role 0 and 0 for role 1) and
  stores that role's move in `last_opponent`. With several encounters in one step, the last one
  would win; here there is only one.

## References

- Axelrod, R. (1980). Effective choice in the prisoner's dilemma. *Journal of Conflict Resolution*,
  24(1), 3–25.
- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.

[← Strategies](../strategies.md)
