**English** | [日本語](strategies.ja.md)

# Strategies

## Game

Each round, both players choose C (cooperate, action `0`) or D (defect, action `1`). The payoffs
for (own move, opponent's move) are:

| | Opponent C | Opponent D |
|---|---:|---:|
| **C** | R = 3 | S = 0 |
| **D** | T = 5 | P = 1 |

A match is 200 rounds. The players know the history of their own match only.

## Built-in strategies (9)

These come from socsim-games and are registered under the names below.

| Name | Behavior |
|---|---|
| `all_c` | Always C |
| `all_d` | Always D |
| `tit_for_tat` | C first, then copies the opponent's previous move |
| `grim` | C until the opponent defects once, then D forever |
| `wsls` | Win-stay, lose-shift: C if both players made the same move last round, D otherwise |
| `fictitious_play` | Best response to the frequency of the opponent's past moves |
| `uniform_random` | C or D with probability 1/2 each |
| `logit` | Chooses stochastically by the expected payoff from the same frequencies, with λ = 1 |
| `roth_erev` | Roth–Erev reinforcement learning with φ = 0.1, ε = 0.2 and an initial propensity of 10.125 per action |

The initial propensity 10.125 is Erev and Roth's s(1) = 9 times the mean payoff
(5 + 3 + 1 + 0) / 4 = 2.25, divided by the two actions.

The socsim-games Roth–Erev implementation rejects payoffs of 0 or less. In matches that include
`roth_erev`, the payoff S given to the rules is therefore replaced with `f64::MIN_POSITIVE`. A round
with payoff 0 only decays the propensities, so the update is effectively the same as with S = 0.
The payoffs in the output are recomputed from the recorded moves with S = 0.

## Custom strategies (9)

These are implemented in `simulation/src/strategies.rs`.

| Name | Behavior | Random | State |
|---|---|:-:|:-:|
| `tit_for_two_tats` | D only when the opponent defected twice in a row | | |
| `suspicious_tft` | D first, then copies the opponent's previous move | | |
| `generous_tft` | Like TFT, but forgives a D with probability g = 1/3 | ✓ | |
| `joss` | Like TFT, but plays D with probability 0.1 where it would play C | ✓ | |
| `alternator` | Starts with C and alternates C and D | | ✓ |
| `extort2` | C first; then C with probability 8/9, 1/2, 1/3, 0 after (C, C), (C, D), (D, C), (D, D) | ✓ | |
| `gradual` | After an opponent's D, plays D n times and then C twice, where n is the total number of D the opponent has played | | ✓ |
| `prober` | Opens with D, C, C; if the opponent played C in rounds 2 and 3, D forever, otherwise TFT | | |
| `soft_majority` | C while the opponent's C count is at least its D count, D otherwise; C first | | |

- **Generous TFT**: g = min{1 − (T − R)/(R − S), (R − P)/(T − P)}, which is 1/3 with these payoffs
  (Nowak and Sigmund, 1992).
- **Extort-2**: a zero-determinant strategy of Press and Dyson (2012) with χ = 2 and φ = 1/18. It
  enforces s_X − P = 2 (s_Y − P) on the long-run payoffs s_X and s_Y.
- **Gradual**: n counts every D the opponent has played up to the start of the retaliation,
  including D played during an earlier retaliation; a sequence, once started, is not changed. Against
  `all_d`, Gradual plays C, D, C, C, D, D, D, D, C, C, … with retaliations of 1, 4 and 10
  (Beaufils, Delahaye and Mathieu, 1997).
- **Prober** and **Soft Majority** follow Mathieu and Delahaye (2017).

`pavlov` (C if both players made the same move last round, D otherwise) is also implemented. It is
the same rule as the built-in `wsls`, so it is used only for the check in the `checks` mode and is
not in the tournament.

## Ranking

The tournament plays 171 pairs (18 × 19 / 2, including each strategy against itself) for seeds 0 to
29.

- In a match against itself, the payoff of the strategy is the mean of the two players.
- For each seed, the per-round payoffs against the 18 opponents (including itself) are averaged.
  The mean of these 30 values is `mean_payoff`, and their population standard deviation is
  `std_over_seeds`. The ranking is ordered by `mean_payoff`.
- `wins`, `losses` and `ties` compare, against each of the 17 other strategies, the seed-mean
  cumulative payoffs of the two sides.
- `stochastic` marks the strategies that draw random numbers: `uniform_random`, `logit`,
  `roth_erev`, `generous_tft`, `joss` and `extort2`.

## References

- Beaufils, B., Delahaye, J.-P., & Mathieu, P. (1997). Our meeting with gradual: A good strategy for
  the iterated prisoner's dilemma. In C. Langton & K. Shimohara (Eds.), *Artificial Life V:
  Proceedings of the Fifth International Workshop on the Synthesis and Simulation of Living
  Systems*. MIT Press.
- Mathieu, P., & Delahaye, J.-P. (2017). New winning strategies for the iterated prisoner's dilemma.
  *Journal of Artificial Societies and Social Simulation*, 20(4), 12.
- Nowak, M. A., & Sigmund, K. (1992). Tit for tat in heterogeneous populations. *Nature*, 355,
  250–253.
- Press, W. H., & Dyson, F. J. (2012). Iterated Prisoner's Dilemma contains strategies that dominate
  any evolutionary opponent. *Proceedings of the National Academy of Sciences*, 109(26),
  10409–10413.
- Erev, I., & Roth, A. E. (1998). Predicting how people play games: Reinforcement learning in
  experimental games with unique, mixed strategy equilibria. *American Economic Review*, 88(4),
  848–881.
