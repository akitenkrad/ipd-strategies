**English** | [日本語](wsls.ja.md)

# Win-Stay, Lose-Shift (`wsls`)

Win-Stay, Lose-Shift repeats its previous move after a good round and switches after a bad one.
With these payoffs that means: C if both players made the same move in the previous round, D
otherwise.

## Origin and properties

Nowak and Sigmund (1993) found this strategy, which they called Pavlov, to be the winner of
evolutionary simulations of memory-one strategies with mutation and selection. A round counts as a
win when the payoff is R or T, that is, when the opponent cooperated; after a win the strategy
stays with its move, after a loss (S or P) it shifts.

In Axelrod's terms it is nice (it never defects first) and retaliatory. It is deterministic. Nowak and Sigmund point to two advantages over Tit for Tat: two
WSLS players recover from an occasional mistake within two rounds instead of echoing it, and WSLS
exploits unconditional cooperators after a mistake. Its weakness is All-D: after a loss against D
it shifts to C, so against All-D it cooperates in every other round.

The table below is the whole rule ("previous" is the previous round's pair of moves):

| Previous (own, opponent) | Win or loss | Next move |
|---|---|:-:|
| (C, C) | Win (R) | C |
| (C, D) | Loss (S) | D |
| (D, C) | Win (T) | D |
| (D, D) | Loss (P) | C |

## How it moves

| Round | Move |
|---|---|
| 1 | C |
| t ≥ 2 | If the opponent played C in round t − 1, repeat own move of round t − 1; otherwise switch it |

**State.** `init_state` creates an `AutomatonState` with kind `Wsls`, wrapped in an
`AutomatonPolicyState`. `learn_step` writes the pair of moves of the round into `own_last` and
`opponent_last`; `decide` reads both.

**Examples** (first 16 rounds at seed 0).

Against `all_d`:

| Round | Opponent | WSLS | Note |
|---:|:-:|:-:|---|
| 1 | D | C | Loss (S) |
| 2 | D | D | Shift to D; loss (P) |
| 3 | D | C | Shift back to C |
| 4–16 | D | D, C, … | C and D alternate for the rest of the match |

Against `suspicious_tft` (it opens with D and then copies the opponent's previous move):

| Round | Opponent | WSLS | Note |
|---:|:-:|:-:|---|
| 1 | D | C | Loss (S) |
| 2 | C | D | WSLS shifts; STFT copies C; win (T) |
| 3 | D | D | WSLS stays; STFT copies D; loss (P) |
| 4 | D | C | WSLS shifts; STFT copies D; loss (S) |
| 5–16 | C, D, D, … | D, D, C, … | The three-round cycle repeats |

## Reading the code

`wsls` is one of four strategies registered in this repository as an `Automaton` with the
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
  checks that every role has exactly two actions and creates the automaton with `own_last` and
  `opponent_last` both `None`.
- **`decide`**
  ([rule/learning.rs#L866-L883](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L866-L883))
  calls `next_action`
  ([learn/automaton.rs#L129-L158](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L129-L158)).
  For `Wsls` it returns `own_last` when `opponent_last` is C, the other action when it is D, and C
  while there is no previous round. The doc comment above it contains the same four-row table. It
  draws no random number.
- **`learn_step`**
  ([rule/learning.rs#L885-L900](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L885-L900))
  passes both moves to `observe`
  ([learn/automaton.rs#L175-L191](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L175-L191)),
  which stores them as `own_last` and `opponent_last`.

The implementation defines a win by the opponent's move, not by comparing the payoff with an
aspiration level. With R = 3, T = 5, S = 0 and P = 1 the two definitions agree: R and T are the
two highest payoffs, and they are exactly the rounds where the opponent cooperated.

Under `PerStep` the rule uses the `global` entry of `AutomatonPolicyState`. The `by_opponent` list
is used only under the `PerEncounter` scope and stays empty here.

## References

- Nowak, M., & Sigmund, K. (1993). A strategy of win-stay, lose-shift that outperforms tit-for-tat in
  the Prisoner's Dilemma game. *Nature*, 364, 56–58.

[← Strategies](../strategies.md)
