[English](strategies.md) | **日本語**

# 戦略

## ゲーム

各ラウンドで，2 人はそれぞれ C (協力，行動 `0`) か D (裏切り，行動 `1`) を選びます．(自分の手, 相手の手) ごとの自分の利得は次のとおりです．

| | 相手が C | 相手が D |
|---|---:|---:|
| **C** | R = 3 | S = 0 |
| **D** | T = 5 | P = 1 |

1 回の対戦は 200 ラウンドです．各プレイヤーが知ることができるのは，自分の対戦の履歴だけです．

## 組み込みの戦略 (9)

socsim-games の規則を，次の名前で登録しています．

| 名前 | 振る舞い |
|---|---|
| `all_c` | 常に C |
| `all_d` | 常に D |
| `tit_for_tat` | 初回は C，以降は相手の前回の手をまねる |
| `grim` | 相手が一度でも D を出すまで C，以降はずっと D |
| `wsls` | Win-Stay, Lose-Shift．前回の自分と相手の手が同じなら C，違えば D |
| `fictitious_play` | 相手の過去の手の頻度に最適反応する |
| `uniform_random` | C と D を確率 1/2 ずつで出す |
| `logit` | 同じ頻度から計算した期待利得に応じて，確率的に手を選ぶ．λ = 1 |
| `roth_erev` | Roth–Erev の強化学習．φ = 0.1，ε = 0.2，1 行動あたりの初期の強さ 10.125 |

初期の強さ 10.125 は，Erev と Roth の s(1) = 9 に利得の平均 (5 + 3 + 1 + 0) / 4 = 2.25 を掛け，行動の数 2 で割った値です．

socsim-games の Roth–Erev の実装は，利得が 0 以下の回を受け付けません．そこで `roth_erev` を含む組だけ，規則に渡す利得 S を `f64::MIN_POSITIVE` に置き換えています．
利得 0 の回は強さが減衰するだけなので，更新の上では S = 0 と実質同じです．出力する利得は，記録した手から S = 0 で計算し直しています．

## 自作の戦略 (9)

`simulation/src/strategies.rs` に実装しています．

| 名前 | 振る舞い | 乱数 | 状態 |
|---|---|:-:|:-:|
| `tit_for_two_tats` | 相手が 2 回続けて D を出したときだけ D | | |
| `suspicious_tft` | 初回は D，以降は相手の前回の手をまねる | | |
| `generous_tft` | TFT と同じだが，相手の D を確率 g = 1/3 で許して C を返す | ✓ | |
| `joss` | TFT と同じだが，C を返す場面で確率 0.1 で D を出す | ✓ | |
| `alternator` | C から始めて C と D を交互に出す | | ✓ |
| `extort2` | 初回は C．以降は前回の手の組が (C, C)，(C, D)，(D, C)，(D, D) のとき，それぞれ確率 8/9，1/2，1/3，0 で C | ✓ | |
| `gradual` | 相手の D を見たら，相手の D の総数を n として D を n 回，続けて C を 2 回出す | | ✓ |
| `prober` | 最初の 3 回は D，C，C．相手が 2 回目と 3 回目に C を出していれば以降ずっと D，そうでなければ TFT | | |
| `soft_majority` | 相手の C の回数が D の回数以上なら C，そうでなければ D．初回は C | | |

- **Generous TFT**：g = min{1 − (T − R)/(R − S), (R − P)/(T − P)} で，この利得では 1/3 です (Nowak and Sigmund, 1992)．
- **Extort-2**：Press と Dyson (2012) のゼロ行列式戦略のうち，χ = 2，φ = 1/18 のものです．長い目で見た平均利得 s_X，s_Y のあいだに s_X − P = 2 (s_Y − P) を成り立たせます．
- **Gradual**：報復の列を始める時点までに観測した相手の D をすべて n に数えます．前の列の途中に相手が出した D も数えますが，始めた列は途中で変えません．`all_d` が相手なら，C，D，C，C，D，D，D，D，C，C，… と，報復が 1 回，4 回，10 回と伸びます (Beaufils, Delahaye and Mathieu, 1997)．
- **Prober** と **Soft Majority** は Mathieu と Delahaye (2017) に従っています．

Pavlov (前回の自分と相手の手が同じなら C，違えば D) は，組み込みの `wsls` と同じ規則なので，ここでは実装していません．単体テストの中でだけ定義し，`wsls` と同じ手を出すことを確かめています．

## 順位の決め方

総当たりでは，171 組 (18 × 19 / 2，自分自身との組を含む) を seed 0〜29 で対戦させます．

- 自分自身との対戦では，同じ規則の 2 人の利得の平均を，その戦略の利得とします．
- seed ごとに，1 ラウンドあたりの利得を 18 の相手 (自分を含む) について平均します．この 30 個の値の平均が `mean_payoff`，母標準偏差が `std_over_seeds` です．順位は `mean_payoff` で決めます．
- `wins`，`losses`，`ties` は，自分以外の 17 の相手それぞれについて，seed 平均の累積利得を相手と比べた数です．
- `stochastic` は乱数を引く戦略の印です．`uniform_random`，`logit`，`roth_erev`，`generous_tft`，`joss`，`extort2` が該当します．

## 参考文献

- Beaufils, B., Delahaye, J.-P., & Mathieu, P. (1997). Our meeting with gradual: A good strategy for the iterated prisoner's dilemma. In C. Langton & K. Shimohara (Eds.), *Artificial Life V: Proceedings of the Fifth International Workshop on the Synthesis and Simulation of Living Systems*. MIT Press.
- Mathieu, P., & Delahaye, J.-P. (2017). New winning strategies for the iterated prisoner's dilemma. *Journal of Artificial Societies and Social Simulation*, 20(4), 12.
- Nowak, M. A., & Sigmund, K. (1992). Tit for tat in heterogeneous populations. *Nature*, 355, 250–253.
- Press, W. H., & Dyson, F. J. (2012). Iterated Prisoner's Dilemma contains strategies that dominate any evolutionary opponent. *Proceedings of the National Academy of Sciences*, 109(26), 10409–10413.
- Erev, I., & Roth, A. E. (1998). Predicting how people play games: Reinforcement learning in experimental games with unique, mixed strategy equilibria. *American Economic Review*, 88(4), 848–881.
