[English](roth_erev.md) | **日本語**

# Roth–Erev (`roth_erev`)

Roth–Erev は C と D それぞれの強さ (propensity) を持ち，強さに比例する確率で手を選び，各ラウンドのあとに出した手を受け取った利得で強めます．

## 由来と性質

Roth と Erev (1995) は，実験の参加者がゲームの中でどう学ぶかを説明するためにこの強化学習のモデルを提案し，Erev と Roth (1998) はそれを 3 つのパラメータを持つモデルに広げました．初期の強さ s(1)，忘却 φ，試行 ε です．プレイヤーは相手をまったくモデル化せず，自分の行動がどれだけ利得を生んだかだけを覚えています．

乱数を使う戦略です．自分の利得だけから学ぶので，振る舞いは相手の反応しだいです．All-C が相手なら D の方が利得が高い (5 対 3) ので D に傾きます．応報する相手なら，協力すれば R = 3，裏切ればすぐに P = 1 になるので，原理的には協力を学べます．φ = 0.1 では古い経験はすぐに薄れ，ε = 0.2 では毎回の利得の 5 分の 1 が出さなかった方の行動に回るので，どちらの行動の確率もすぐには 0 や 1 に近づきません．Axelrod の言葉では nice ではありません．1 ラウンド目に確率 1/2 で D を出します．

## 動き

強さを q_C，q_D とします．各行動の確率は

P(C) = q_C / (q_C + q_D)

です．行動 k を出して利得 x を受け取ったラウンドのあと，すべての強さを次のように更新します．

| 行動 | 更新 |
|---|---|
| 出した行動 k | q ← (1 − φ) q + (1 − ε) x |
| もう一方の行動 | q ← (1 − φ) q + ε x |

行動が 2 つなので，Erev と Roth (1998) の試行の項 ε x / (M − 1) は ε x になります．

登録したパラメータは φ = 0.1，ε = 0.2，1 行動あたりの初期の強さ 10.125 です．初期の強さは，Erev と Roth の s(1) = 9 に利得の平均 (5 + 3 + 1 + 0) / 4 = 2.25 を掛け，行動の数 2 で割った値です．

**状態**：`init_state` は `PropensityState` を作ります．その `global` は，`propensities = [10.125, 10.125]`，`phi = 0.1`，`epsilon = 0.2` を持つ `RothErevState` です．`learn_step` は毎ラウンドのあとに自分の手と利得で強さを更新し，`decide` は強さを読みます．

**利得 S**：socsim-games の実装は，ゲームのすべての利得が正であることを求め，実際に受け取った利得が 0 以下だと受け付けません．そこで `roth_erev` を含む組だけ，戦略に渡す利得 S を `f64::MIN_POSITIVE` に置き換えています ([arena.rs#L79-L90](../../simulation/src/arena.rs#L79-L90))．利得 S の回は強さが減衰するだけなので，更新の上では S = 0 と実質同じです．出力する利得は，記録した手から S = 0 で計算し直しています．

**例** (seed 0 の最初の 16 ラウンド．乱数を含む 1 つの標本)．P(C) は，記録した手に沿って上の更新を S = 0 で計算した値です．

`all_c` が相手のとき：

| ラウンド | 相手 | Roth–Erev | P(C) | 備考 |
|---:|:-:|:-:|---:|---|
| 1 | C | C | 0.500 | 強さが等しい |
| 2 | C | D | 0.542 | C が R = 3 で強まった |
| 3 | C | D | 0.471 | D が T = 5 で強まった |
| 4 | C | D | 0.421 | |
| 5 | C | C | 0.383 | |
| 6 | C | D | 0.426 | |
| 7 | C | C | 0.389 | |
| 8 | C | D | 0.429 | |
| 9 | C | C | 0.394 | |
| 10 | C | C | 0.432 | |
| 11〜16 | C | D | 0.466〜0.328 | T = 5 が続いて P(C) が下がる |

`tit_for_tat` が相手のとき：

| ラウンド | 相手 | Roth–Erev | P(C) | 備考 |
|---:|:-:|:-:|---:|---|
| 1 | C | C | 0.500 | |
| 2 | C | D | 0.542 | |
| 3 | D | D | 0.471 | TFT は 2 ラウンド目をまねる |
| 4 | D | D | 0.459 | 互いに裏切ると P = 1 しか得られない |
| 5 | D | C | 0.447 | |
| 6 | C | D | 0.447 | D に対する C は S なので減衰だけ |
| 7 | D | C | 0.392 | |
| 8 | C | D | 0.392 | |
| 9 | D | C | 0.350 | |
| 10 | C | C | 0.350 | |
| 11 | C | D | 0.413 | |
| 12〜16 | D | D | 0.369〜0.339 | |

## コードの読み方

`roth_erev` は `RothErev::new(0.1, 0.2, 10.125)` として登録しています．引数の順は φ，ε，初期の強さです ([arena.rs#L60-L62](../../simulation/src/arena.rs#L60-L62))．

- `RothErev` 戦略とコンストラクタ：[rule/learning.rs#L690-L706](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L690-L706)
- `PropensityState`：[world/state.rs#L84-L108](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/world/state.rs#L84-L108)
- `RothErevState::new`：[learn/roth_erev.rs#L17-L38](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/roth_erev.rs#L17-L38)

**エンジンが呼ぶ順番**：`init_state` は，`build_world` が対戦を組み立てるときにエージェントごとに 1 回だけ呼ばれ ([assemble.rs#L719-L727](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L719-L727))，返した状態はそのエージェントの分として `GameWorld` に保存されます．この実験は `PlayOptions::new()` で対戦を組み立てており ([arena.rs#L112-L121](../../simulation/src/arena.rs#L112-L121))，その決定の単位は `PerStep` です ([assemble.rs#L261-L271](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L261-L271))．そのためエンジンは，各ステップでエージェントごとに `decide` を 1 回呼びます ([mechanism/decide.rs#L248-L250](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L248-L250))．1 ステップの対戦は 1 つなので，1 ラウンドに 1 回です．そのステップのすべての対戦が終わったあと，`learn_step` が 1 回呼ばれます ([mechanism/decide.rs#L431-L438](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L431-L438))．どちらにもエージェントの状態が `&mut` で渡されます．

- **`init_state`** ([rule/learning.rs#L738-L746](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L738-L746))：役割ごとに `RothErevState` を作り，役割 0 のものを `global` として持ちます．`RothErevState::new` は，初期の強さが正であること，φ と ε が [0, 1) にあることを確かめ，`validate_positive_payoffs` を呼びます ([learn/roth_erev.rs#L157-L185](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/roth_erev.rs#L157-L185))．これはすべての手の組をたどり，0 以下の利得が 1 つでもあれば失敗します．S = 0 のままだと引っかかるのはこの確認です．
- **`decide`** ([rule/learning.rs#L748-L767](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L748-L767))：`choose_action` を呼びます ([learn/roth_erev.rs#L67-L70](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/roth_erev.rs#L67-L70))．`selection_probabilities` ([learn/roth_erev.rs#L45-L65](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/roth_erev.rs#L45-L65)) は強さを最大値で割り，さらに和で割るので，結果は q_k / Σ q です．`sample_probabilities` ([learn/mod.rs#L130-L159](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/mod.rs#L130-L159)) は一様乱数を 1 つ引き，それが P(C) 未満なら C，そうでなければ D を返します．
- **`learn_step`** ([rule/learning.rs#L769-L784](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L769-L784))：各対戦について `observe` を呼びます ([rule/learning.rs#L713-L725](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L713-L725))．`observe` は自分の手と自分の利得を読み，`update` を呼びます ([learn/roth_erev.rs#L97-L135](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/roth_erev.rs#L97-L135))．`update` は有限で正でない利得を受け付けず，上の表のとおりに更新します．

`PerStep` では，戦略は `global` の強さを使います．`by_opponent` に相手ごとの強さを持つのは決定の単位が `PerEncounter` のときだけで，ここでは空のままです．`learn_encounter` も何もせずに戻ります．

**教科書との違い**：強化に使うのは利得 x そのもので，実装はそれが正であることを求めます．このため，この戦略を含む対戦では S を `f64::MIN_POSITIVE` に置き換えています．初期の強さは s(1) と利得の平均からではなく 1 つの数 (10.125) として直接渡しており，その値は上で述べた s(1) = 9 から求めたものです．

## 参考文献

- Erev, I., & Roth, A. E. (1998). Predicting how people play games: Reinforcement learning in experimental games with unique, mixed strategy equilibria. *American Economic Review*, 88(4), 848–881.
- Roth, A. E., & Erev, I. (1995). Learning in extensive-form games: Experimental data and simple dynamic models in the intermediate term. *Games and Economic Behavior*, 8(1), 164–212.

[← 戦略の一覧](../strategies.ja.md)
