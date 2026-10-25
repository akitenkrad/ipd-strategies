**English** | [日本語](all_c.ja.md)

# All-C (`all_c`)

All-C cooperates in every round, whatever the opponent does.

## Origin and properties

All-C is the unconditional cooperator, the simplest baseline in the iterated prisoner's dilemma
literature. In Axelrod's terms it is nice (it never defects first) and forgiving, but not
retaliatory: a defection by the opponent has no consequence. It is deterministic.

Its weakness is that it can be exploited without limit. Against All-D it receives S = 0 in every
round while the opponent receives T = 5. Against any strategy that reciprocates, it settles into
mutual cooperation and receives R = 3. Nowak and Sigmund (1993) note that Win-Stay, Lose-Shift
exploits unconditional cooperators where Tit for Tat does not; in this tournament that does not
show, because `wsls` never defects first.

## How it moves

The move is always C (action `0`). It never looks at the opponent.

**State.** `init_state` creates an `AutomatonState` with kind `AllC`, wrapped in an
`AutomatonPolicyState`. `learn_step` writes the last pair of moves into it every round, but
`decide` never reads it.

**Examples** (first 16 rounds at seed 0).

Against `all_d`:

| Round | Opponent | All-C | Note |
|---:|:-:|:-:|---|
| 1 | D | C | All-C receives S = 0 |
| 2–16 | D | C | Same in every round |

Against `tit_for_tat`:

| Round | Opponent | All-C | Note |
|---:|:-:|:-:|---|
| 1 | C | C | TFT opens with C |
| 2–16 | C | C | Mutual cooperation, R = 3 each |

## Reading the code

`all_c` is one of four strategies registered in this repository as an `Automaton` with the
cooperation action `C`
([arena.rs#L37-L47](../../simulation/src/arena.rs#L37-L47)). The rule and its state live in
socsim-games:

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
  calls `next_action`, which for `AllC` returns the cooperation action
  ([learn/automaton.rs#L144-L148](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L144-L148)).
  It draws no random number.
- **`learn_step`**
  ([rule/learning.rs#L885-L900](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L885-L900))
  passes both moves to `observe`
  ([learn/automaton.rs#L175-L191](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L175-L191)).
  The recorded values do not change the move.

Under `PerStep` the rule uses the `global` entry of `AutomatonPolicyState`. The `by_opponent` list
is used only under the `PerEncounter` scope and stays empty here.

## References

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Nowak, M., & Sigmund, K. (1993). A strategy of win-stay, lose-shift that outperforms tit-for-tat in
  the Prisoner's Dilemma game. *Nature*, 364, 56–58.

[← Strategies](../strategies.md)
