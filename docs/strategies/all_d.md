**English** | [日本語](all_d.ja.md)

# All-D (`all_d`)

All-D defects in every round, whatever the opponent does.

## Origin and properties

All-D is the unconditional defector. Defection is the dominant action of the one-shot prisoner's
dilemma, and playing it in every round is a subgame-perfect equilibrium of the finitely repeated
game, so All-D is the standard non-cooperative baseline. In Axelrod's terms it is not nice (it
defects first, in round 1). It is deterministic.

It never receives less than its opponent in a match: it exploits unconditional cooperators
(T = 5 per round against All-C) and loses only the first round to reciprocators such as Tit for
Tat or Grim, after which both sides receive P = 1. Its weakness is that it never gets R = 3, so in
a population of reciprocators it earns far less than they earn among themselves (Axelrod, 1984).

## How it moves

The move is always D (action `1`). It never looks at the opponent.

**State.** `init_state` creates an `AutomatonState` with kind `AllD`, wrapped in an
`AutomatonPolicyState`. `learn_step` writes the last pair of moves into it every round, but
`decide` never reads it.

**Examples** (first 16 rounds at seed 0).

Against `tit_for_tat`:

| Round | Opponent | All-D | Note |
|---:|:-:|:-:|---|
| 1 | C | D | All-D receives T = 5, TFT receives S = 0 |
| 2 | D | D | TFT copies the D of round 1 |
| 3–16 | D | D | Mutual defection, P = 1 each |

Against `grim`:

| Round | Opponent | All-D | Note |
|---:|:-:|:-:|---|
| 1 | C | D | Grim's trigger is pulled |
| 2–16 | D | D | Grim defects for the rest of the match |

## Reading the code

`all_d` is one of four strategies registered in this repository as an `Automaton` with the
cooperation action `C` ([arena.rs#L37-L47](../../simulation/src/arena.rs#L37-L47)). The rule and
its state live in socsim-games:

- The `Automaton` rule:
  [rule/learning.rs#L818-L833](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L818-L833).
- `AutomatonKind` and `AutomatonState`:
  [learn/automaton.rs#L9-L61](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L9-L61).

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
  ([rule/learning.rs#L858-L864](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L858-L864))
  checks that every role has exactly two actions and creates the automaton.
- **`decide`**
  ([rule/learning.rs#L866-L883](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L866-L883))
  calls `next_action`, which for `AllD` returns `1 - coop`, the defection action
  ([learn/automaton.rs#L144-L148](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L144-L148)).
  It draws no random number. `AllD` is the only automaton kind that does not open with C.
- **`learn_step`**
  ([rule/learning.rs#L885-L900](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L885-L900))
  passes both moves to `observe`
  ([learn/automaton.rs#L175-L191](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L175-L191)).
  The recorded values do not change the move.

Under `PerStep` the rule uses the `global` entry of `AutomatonPolicyState`. The `by_opponent` list
is used only under the `PerEncounter` scope and stays empty here.

## References

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.

[← Strategies](../strategies.md)
