**English** | [日本語](alternator.ja.md)

# Alternator (`alternator`)

Alternator plays C, D, C, D, … for the whole match, starting with C. It never looks at the
opponent: the next move depends only on the round.

## Origin and properties

Alternator is a fixed periodic strategy that serves as a foil in tournaments rather than as a
serious contender. Mathieu and Delahaye (2017) include the same behavior in their set of classical
strategies under the name `per_cd` (plays C, D periodically).

In the terms of Axelrod (1984):

- **Not nice**: it defects in round 2 whatever the opponent did.
- **Not retaliatory and not forgiving**: it does not react to the opponent at all, so these
  properties do not apply.
- **Clear** in the sense of being trivially predictable: its moves do not depend on anything the
  opponent can do.

It is deterministic.

Alternator is useful mainly as a test for other strategies: it gains T against unconditional
cooperators every other round, and it shows how a strategy reacts to an opponent that defects
regularly without escalating. Against TFT the two fall out of phase and alternate (D, C) and
(C, D); against `grim` it is punished forever after its first D.

## How it moves

**State.** `Policy::Alternator { next }`, the move to play in the current round. It starts at C.
`decide` returns `next`, and at the end of each step `learn_step` flips it (C → D, D → C).

**Traces.** These are the first 16 rounds at seed 0 of the tournament, as recorded by the program;
the tables show the first 6. In every match Alternator plays C in odd rounds and D in even rounds.

Against `all_c`:

| Round | Opponent | Alternator | Note |
|---:|:-:|:-:|---|
| 1 | C | C | |
| 2 | C | D | exploits the cooperator |
| 3 | C | C | |
| 4 | C | D | |
| 5 | C | C | |
| 6 | C | D | the pattern repeats |

Against `tit_for_tat`, TFT copies Alternator one round late:

| Round | Opponent | Alternator | Note |
|---:|:-:|:-:|---|
| 1 | C | C | mutual C |
| 2 | C | D | |
| 3 | D | C | TFT copies round 2 |
| 4 | C | D | TFT copies round 3 |
| 5 | D | C | |
| 6 | C | D | (D, C) and (C, D) alternate from here on |

Against `grim`:

| Round | Opponent | Alternator | Note |
|---:|:-:|:-:|---|
| 1 | C | C | |
| 2 | C | D | |
| 3 | D | C | Grim is triggered by round 2 |
| 4 | D | D | |
| 5 | D | C | |
| 6 | D | D | Grim defects forever |

Against itself, both players play C, D, C, D, … in phase, so the rounds alternate between
mutual C and mutual D.

## Reading the code

- **Registration.** [`arena.rs#L35`](../../simulation/src/arena.rs#L35) registers `Alternator` under
  the name `alternator` in the rule book built by
  [`rule_book`](../../simulation/src/arena.rs#L19-L64).
- **State type.** The variant `Policy::Alternator { next }` is defined in
  [`state.rs#L17-L19`](../../simulation/src/state.rs#L17-L19), inside the enum `Policy`
  ([`state.rs#L13-L24`](../../simulation/src/state.rs#L13-L24)). `next` holds `C = 0` or `D = 1`
  ([`state.rs#L10-L11`](../../simulation/src/state.rs#L10-L11)).
- **The strategy.** [`strategies.rs#L126-L155`](../../simulation/src/strategies.rs#L126-L155).
  - `init_state` ([`L133-L135`](../../simulation/src/strategies.rs#L133-L135)) returns
    `Policy::Alternator { next: C }`.
  - `decide` ([`L136-L141`](../../simulation/src/strategies.rs#L136-L141)) only reads `next` and
    returns it. It does not read the history at all.
  - `learn_step` ([`L142-L154`](../../simulation/src/strategies.rs#L142-L154)) flips `next`.

**When the engine calls each method.** Each match is one call of
[`play_with`](../../simulation/src/arena.rs#L73-L140). It builds a fresh world with `build_world`
([`arena.rs#L100-L111`](../../simulation/src/arena.rs#L100-L111)), which calls `init_state` once
per agent and stores each agent's `Policy` in the `GameWorld`. The match is then 200 calls of
`sim.step()` ([`arena.rs#L124-L138`](../../simulation/src/arena.rs#L124-L138)). In each step the
engine calls `decide` once for every encounter of the agent, and then, after all encounters,
`learn_step` once per agent. In a match of Alternator against itself, the two agents share the rule
object but each has its own `Policy`, so each flips its own `next`.

**Why the flip is in `learn_step` and not in `decide`.** `decide` is called once per encounter,
while `learn_step` is called once per step after all encounters. If `decide` flipped `next`, an
agent that met three opponents in one step would flip three times and play C, D, C within a single
step. Flipping in `learn_step` advances the state exactly once per step, however many encounters
there are. In this repository each match has a single pair of players
([`FixedPlayers::pairs(&[(0, 1)])`](../../simulation/src/arena.rs#L114)), so there is one encounter
per step and both placements would give the same moves; `learn_step` is the placement that stays
correct in other settings. [Gradual](gradual.md), by contrast, updates inside `decide` and relies on
the one-encounter-per-step setting.

**Test.** [`tests.rs#L176-L184`](../../simulation/src/tests.rs#L176-L184) plays `alternator`
against `all_c` and checks that all 200 moves are C in odd rounds and D in even rounds.

## References

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Mathieu, P., & Delahaye, J.-P. (2017). New winning strategies for the iterated prisoner's dilemma.
  *Journal of Artificial Societies and Social Simulation*, 20(4), 12.

[← Strategies](../strategies.md)
