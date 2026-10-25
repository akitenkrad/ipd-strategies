**English** | [日本語](grim.ja.md)

# Grim Trigger (`grim`)

Grim cooperates until the opponent defects once, and then defects for the rest of the match.

## Origin and properties

Grim is the trigger strategy that Friedman (1971) used to show that cooperation can be sustained as
a non-cooperative equilibrium of an infinitely repeated game: any deviation is punished by
reverting to the one-shot equilibrium forever. Axelrod (1984) called the strategy "Friedman" after
this article.

In Axelrod's terms it is nice (it never defects first) and maximally retaliatory, but not
forgiving at all: one defection by the opponent ends cooperation for good. It is deterministic.

Its strength is that it cannot be exploited for more than one round. Its weakness is the same
unforgivingness: a single defection, whether a probe, a random move or an error, locks both sides
into mutual defection (P = 1) for the rest of the match, even if the opponent wants to return to
cooperation.

## How it moves

| Mode | Move | Looks at the opponent? | When it leaves |
|---|:-:|:-:|---|
| Cooperating (not triggered) | C | Yes | The opponent plays D once |
| Triggered | D | No | Never |

The trigger depends only on the opponent's moves; Grim's own moves do not pull it.

**State.** `init_state` creates an `AutomatonState` with kind `Grim` and
`grim_triggered: false`, wrapped in an `AutomatonPolicyState`. `learn_step` sets `grim_triggered`
to `true` the first time the opponent's move is not C and never sets it back.

**Examples** (first 16 rounds at seed 0).

Against `joss` (one sample; Joss plays a random D where TFT would play C):

| Round | Opponent | Grim | Note |
|---:|:-:|:-:|---|
| 1–9 | C | C | Mutual cooperation |
| 10 | D | C | Joss's random defection pulls the trigger |
| 11 | C | D | Grim defects from now on |
| 12 | D | D | Joss copies Grim's D |
| 13–16 | D | D | Mutual defection |

Against `alternator` (deterministic; it alternates C and D, starting with C):

| Round | Opponent | Grim | Note |
|---:|:-:|:-:|---|
| 1 | C | C | |
| 2 | D | C | The trigger is pulled |
| 3–16 | C, D, … | D | Grim no longer looks at the opponent |

## Reading the code

`grim` is one of four strategies registered in this repository as an `Automaton` with the
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
  checks that every role has exactly two actions and creates the automaton with
  `grim_triggered: false`.
- **`decide`**
  ([rule/learning.rs#L866-L883](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L866-L883))
  calls `next_action`, which for `Grim` returns D if `grim_triggered` is set and C otherwise
  ([learn/automaton.rs#L144-L153](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L144-L153)).
  It draws no random number.
- **`learn_step`**
  ([rule/learning.rs#L885-L900](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L885-L900))
  passes both moves to `observe`
  ([learn/automaton.rs#L175-L191](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L175-L191)),
  which sets `grim_triggered` when the opponent's move differs from `coop`.

Under `PerStep` the rule uses the `global` entry of `AutomatonPolicyState`. The `by_opponent` list
is used only under the `PerEncounter` scope and stays empty here.

## References

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Friedman, J. W. (1971). A non-cooperative equilibrium for supergames. *The Review of Economic
  Studies*, 38(1), 1–12.

[← Strategies](../strategies.md)
