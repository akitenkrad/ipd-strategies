**English** | [日本語](gradual.ja.md)

# Gradual (`gradual`)

Gradual cooperates until the opponent defects. When it answers a defection, it plays D as many
times as the opponent has defected in total so far, then plays C twice to calm things down, and
then goes back to watching the opponent's last move. Each punishment is longer than the one
before, but every punishment ends with an offer to cooperate again.

## Origin and properties

Gradual was proposed by Beaufils, Delahaye and Mathieu (1997) as a refinement of Tit for Tat (TFT):
like TFT it answers a defection, but the answer grows with the opponent's record, and it always
ends with two cooperations. Mathieu and Delahaye (2017) keep it in their set of classical
strategies.

In the terms Axelrod (1984) used to describe successful strategies:

- **Nice**: it is never the first to defect. Before the opponent's first D it only plays C.
- **Retaliatory (provocable)**: it answers a D, and more severely each time.
- **Forgiving**: every punishment is followed by two C and a return to normal mode, so a single D
  is never punished forever, as it is by `grim`.
- **Less clear than TFT**: the length of a punishment depends on the whole history, and during a
  punishment and the calming that follows it does not react to the opponent at all.

It is deterministic.

Its strength is that the two calming C can break an echo of defections. TFT and Suspicious TFT copy
each other's D forever; Gradual punishes once, offers C twice, and both sides return to mutual
cooperation (see the trace against `suspicious_tft` below). Against a persistent defector the
punishments grow (1, 4, 10, 22, … against `all_d`), so it defects almost all the time.

Its weaknesses follow from the same design. Punishments never shrink, so against an opponent that
defects now and then, such as `joss`, they get longer as the match goes on. And because the
strategy ignores the opponent during punishment and calming, a D played in that period is not
answered when it happens (it is only counted for later).

## How it moves

**State.** `Policy::Gradual { remaining_defections, remaining_calming }`, the number of D still to
play in the current punishment and the number of calming C still to play. Both start at 0. The
state decides the mode:

| Mode | Condition | Move | Looks at the opponent? | When it leaves |
|---|---|---|---|---|
| Normal | both counters are 0 | C, or D if the opponent's previous move was D | Yes, only the opponent's previous move | On seeing a D: the D it plays now is the first of a punishment |
| Punish | `remaining_defections > 0` | D | No | When the n D of the punishment have been played |
| Calm | `remaining_defections == 0`, `remaining_calming > 0` | C | No | After the second C |

**The rule in normal mode.** In the first round there is no history and it plays C. Afterwards, if
the opponent's previous move was C, it plays C. If it was D, it counts n, the number of D in the
opponent's entire history, plays D now, and sets `remaining_defections = n − 1` and
`remaining_calming = 2`. The punishment is therefore n D in a row (the one played from normal mode
and n − 1 more), followed by two C.

Three consequences are worth spelling out:

- **A D during punishment or calming is not answered by itself.** Normal mode looks only at the
  opponent's previous move, so the only D in that period that it sees is one played in the last
  calming round, because the round after it is decided in normal mode.
- **Every D is counted.** n is recomputed from the full history, so a D played during a punishment
  or a calming phase lengthens the next punishment.
- **A punishment in progress is not changed.** Its length is fixed when it starts.

Descriptions of Gradual in the literature do not settle how a D played during a punishment is
handled. The three points above are the interpretation this implementation uses.

**Traces.** These are the first 16 rounds at seed 0 of the tournament, as recorded by the program.

Against `all_d`, the punishments are 1, 4 and 10 long:

| Round | Opponent | Gradual | Mode / note |
|---:|:-:|:-:|---|
| 1 | D | C | normal, no history |
| 2 | D | D | normal sees D, n = 1: punish 1/1 |
| 3 | D | C | calm 1/2 |
| 4 | D | C | calm 2/2 |
| 5 | D | D | normal sees D, n = 4: punish 1/4 |
| 6 | D | D | punish 2/4 |
| 7 | D | D | punish 3/4 |
| 8 | D | D | punish 4/4 |
| 9 | D | C | calm 1/2 |
| 10 | D | C | calm 2/2 |
| 11 | D | D | normal sees D, n = 10: punish 1/10 |
| 12–16 | D | D | punish 2/10 to 6/10 |

Against `suspicious_tft`, one punishment, and the calming breaks the echo:

| Round | Opponent | Gradual | Mode / note |
|---:|:-:|:-:|---|
| 1 | D | C | normal, no history |
| 2 | C | D | normal sees D, n = 1: punish 1/1 |
| 3 | D | C | calm 1/2; the opponent echoes round 2, and this D is not answered |
| 4 | C | C | calm 2/2 |
| 5 | C | C | normal sees C |
| 6–16 | C | C | normal |

The opponent's D in round 3 is counted: if it defected again, the next punishment would have n = 3.

Against `joss`, which defects at random where it would cooperate:

| Round | Opponent | Gradual | Mode / note |
|---:|:-:|:-:|---|
| 1–9 | C | C | normal |
| 10 | D | C | normal sees C; Joss's random D |
| 11 | C | D | normal sees D, n = 1: punish 1/1 |
| 12 | D | C | calm 1/2; Joss copies round 11, not answered |
| 13 | C | C | calm 2/2 |
| 14 | C | C | normal sees C |
| 15–16 | C | C | normal |

Against `alternator`, the opponent's D during punishment and calming are not answered but
lengthen the next punishment:

| Round | Opponent | Gradual | Mode / note |
|---:|:-:|:-:|---|
| 1 | C | C | normal, no history |
| 2 | D | C | normal sees C |
| 3 | C | D | normal sees D, n = 1: punish 1/1 |
| 4 | D | C | calm 1/2; this D is not answered |
| 5 | C | C | calm 2/2 |
| 6 | D | C | normal sees C |
| 7 | C | D | normal sees D, n = 3 (rounds 2, 4, 6): punish 1/3 |
| 8 | D | D | punish 2/3 |
| 9 | C | D | punish 3/3 |
| 10 | D | C | calm 1/2 |
| 11 | C | C | calm 2/2 |
| 12 | D | C | normal sees C |
| 13 | C | D | normal sees D, n = 6: punish 1/6 |
| 14–16 | D, C, D | D | punish 2/6 to 4/6 |

## Reading the code

- **Registration.** [`arena.rs#L36`](../../simulation/src/arena.rs#L36) registers `Gradual` under
  the name `gradual` in the rule book built by
  [`rule_book`](../../simulation/src/arena.rs#L19-L64).
- **State type.** The variant `Policy::Gradual` is defined in
  [`state.rs#L20-L23`](../../simulation/src/state.rs#L20-L23), inside the enum `Policy`
  ([`state.rs#L13-L24`](../../simulation/src/state.rs#L13-L24)). The actions are the constants
  `C = 0` and `D = 1` ([`state.rs#L10-L11`](../../simulation/src/state.rs#L10-L11)).
- **The strategy.** [`strategies.rs#L157-L202`](../../simulation/src/strategies.rs#L157-L202).
  - `init_state` ([`L164-L169`](../../simulation/src/strategies.rs#L164-L169)) returns both counters
    at 0, which is normal mode.
  - `decide` ([`L170-L201`](../../simulation/src/strategies.rs#L170-L201)) checks the modes in
    order. Punish ([`L183-L186`](../../simulation/src/strategies.rs#L183-L186)) decrements
    `remaining_defections` and returns D without reading the history. Calm
    ([`L187-L190`](../../simulation/src/strategies.rs#L187-L190)) decrements `remaining_calming` and
    returns C, also without reading it. Normal mode
    ([`L192-L200`](../../simulation/src/strategies.rs#L192-L200)) reads the opponent's history
    through `seen` ([`L23-L35`](../../simulation/src/strategies.rs#L23-L35)), and on a D counts all
    the opponent's D and starts a punishment.
  - There is no `learn_step`; the default one, which does nothing, is used.

**When the engine calls each method.** Each match is one call of
[`play_with`](../../simulation/src/arena.rs#L73-L140). It builds a fresh world with `build_world`
([`arena.rs#L100-L111`](../../simulation/src/arena.rs#L100-L111)), which calls `init_state` once
per agent and stores each agent's `Policy` in the `GameWorld`. The match is then 200 calls of
`sim.step()` ([`arena.rs#L124-L138`](../../simulation/src/arena.rs#L124-L138)). In each step the
engine calls `decide` once for every encounter of the agent, and then, after all encounters,
`learn_step` once per agent. In a match of Gradual against itself, the two agents share the rule
object but each has its own `Policy`.

**Why updating inside `decide` is correct here.** Gradual advances its counters in `decide`, so one
call of `decide` must mean one round. That holds because the match has a single pair of players
([`FixedPlayers::pairs(&[(0, 1)])`](../../simulation/src/arena.rs#L114)), so each agent has exactly
one encounter per step. In a setting where an agent meets several opponents in one step, `decide`
would be called several times per step, the counters would run down several times per round, and
the update would have to move to `learn_step`, as [Alternator](alternator.md) does.

**Test.** [`tests.rs#L186-L200`](../../simulation/src/tests.rs#L186-L200) plays `gradual` against
`all_d` and checks the first 22 moves: C, D, C, C, D × 4, C, C, D × 10, C, C.

## References

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Beaufils, B., Delahaye, J.-P., & Mathieu, P. (1997). Our meeting with gradual: A good strategy for
  the iterated prisoner's dilemma. In C. Langton & K. Shimohara (Eds.), *Artificial Life V:
  Proceedings of the Fifth International Workshop on the Synthesis and Simulation of Living
  Systems*. MIT Press.
- Mathieu, P., & Delahaye, J.-P. (2017). New winning strategies for the iterated prisoner's dilemma.
  *Journal of Artificial Societies and Social Simulation*, 20(4), 12.

[← Strategies](../strategies.md)
