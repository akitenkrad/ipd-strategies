[English](logit.md) | **日本語**

# Logit Choice (`logit`)

Logit Choice は，相手の過去の手の頻度から C と D の期待利得を見積もり，exp(λ × 期待利得) に比例する確率で手をランダムに選びます．λ = 1 です．

## 由来と性質

ロジットの式は，離散選択モデルと同じように，利得の差を選択確率に変えます．McKelvey と Palfrey (1995) はこれを使ってロジット量子応答均衡 (logit quantal response equilibrium) を定義しました．そこでは各プレイヤーが，他のプレイヤーの実際の混合戦略にロジットで反応します．ここでのように相手の過去の手の経験頻度に繰り返し当てはめると，fictitious play を滑らかにしたもの (確率的 fictitious play．Fudenberg & Levine, 1998) になります．λ は逆温度で，λ = 0 なら一様にランダム，λ → ∞ なら fictitious play の最適反応になります．

乱数を使う戦略です．fictitious play と同じく，囚人のジレンマではどんな信念に対しても D の方が利得が高いので，2 ラウンド目からは D に傾きますが，ときどき協力し続けます．相手が自分の手に反応することをモデルに含めないので，応報する相手に対して協力を増やすことはありません．Axelrod の言葉では nice (非先制型) ではありません．

## 動き

相手の過去の手に占める C の割合を p とします．見積もった利得は次のとおりです．

- E[C] = 3p + 0(1 − p) = 3p
- E[D] = 5p + 1(1 − p) = 1 + 4p

C を出す確率は

P(C) = exp(λ E[C]) / (exp(λ E[C]) + exp(λ E[D])) = 1 / (1 + exp(λ (1 + p)))

です．

| ラウンド | 情報 | λ = 1 のときの P(C) |
|---|---|---|
| 1 | 観測なし．見積もりはどちらも 0 | 1/2 |
| t ≥ 2 | ラウンド 1 から t − 1 までの相手の手から求めた p | 1/(1 + e²) ≈ 0.119 (p = 1) から 1/(1 + e) ≈ 0.269 (p = 0) のあいだ |

つまり 2 ラウンド目からは確率 12%〜27% で協力し，相手が協力するほど協力しなくなります．

**状態**：`init_state` は `EstimateState` を作ります．その `global` は，`empirical_counts = [0, 0]`，`lambda = 1.0`，見積もりの方式 `EmpiricalFrequency` を持つ `LogitState` です．`learn_step` は毎ラウンドのあとに相手の手の回数に 1 を足し，`decide` は回数を読みます．

**例** (seed 0 の最初の 16 ラウンド．乱数を含む 1 つの標本)．P(C) は，記録した手に沿って上の式から計算した値です．

`all_c` が相手のとき：

| ラウンド | 相手 | Logit | P(C) | 備考 |
|---:|:-:|:-:|---:|---|
| 1 | C | C | 0.500 | 観測なし |
| 2〜8 | C | D | 0.119 | ここから p = 1 |
| 9 | C | C | 0.119 | |
| 10 | C | C | 0.119 | |
| 11〜16 | C | D | 0.119 | |

`tit_for_tat` が相手のとき：

| ラウンド | 相手 | Logit | P(C) | 備考 |
|---:|:-:|:-:|---:|---|
| 1 | C | C | 0.500 | |
| 2 | C | D | 0.119 | |
| 3 | D | D | 0.119 | TFT は 2 ラウンド目をまねる |
| 4〜6 | D | D | 0.159〜0.198 | p が下がるので P(C) が上がる |
| 7 | D | C | 0.209 | |
| 8 | C | D | 0.217 | TFT は C をまねる |
| 9 | D | C | 0.202 | |
| 10 | C | C | 0.209 | |
| 11 | C | D | 0.198 | |
| 12〜16 | D | D | 0.189〜0.209 | |

## コードの読み方

`logit` は `LogitChoice::new(1.0)` として登録しています ([arena.rs#L57-L59](../../simulation/src/arena.rs#L57-L59))．λ = 1 で，経験頻度から見積もります．利得はそのままの尺度 (R = 3，T = 5，S = 0，P = 1) で入り，λ をそれに合わせて尺度を変えることはしていません．

- `LogitChoice` 戦略とコンストラクタ：[rule/learning.rs#L557-L572](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L557-L572)
- `EstimateState`：[world/state.rs#L110-L134](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/world/state.rs#L110-L134)
- `LogitState`：[learn/logit.rs#L20-L50](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/logit.rs#L20-L50)

**エンジンが呼ぶ順番**：`init_state` は，`build_world` が対戦を組み立てるときにエージェントごとに 1 回だけ呼ばれ ([assemble.rs#L719-L727](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L719-L727))，返した状態はそのエージェントの分として `GameWorld` に保存されます．この実験は `PlayOptions::new()` で対戦を組み立てており ([arena.rs#L112-L121](../../simulation/src/arena.rs#L112-L121))，その決定の単位は `PerStep` です ([assemble.rs#L261-L271](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L261-L271))．そのためエンジンは，各ステップでエージェントごとに `decide` を 1 回呼びます ([mechanism/decide.rs#L248-L250](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L248-L250))．1 ステップの対戦は 1 つなので，1 ラウンドに 1 回です．そのステップのすべての対戦が終わったあと，`learn_step` が 1 回呼ばれます ([mechanism/decide.rs#L431-L438](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L431-L438))．どちらにもエージェントの状態が `&mut` で渡されます．

- **`init_state`** ([rule/learning.rs#L607-L615](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L607-L615))：λ とゲームを確かめるために役割ごとに `LogitState` を作り，役割 0 のものを `global` として持ちます．
- **`decide`** ([rule/learning.rs#L617-L638](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L617-L638))：`choose_action` を呼びます ([learn/logit.rs#L169-L177](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/logit.rs#L169-L177))．`expected_payoffs` ([learn/logit.rs#L87-L141](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/logit.rs#L87-L141)) は観測した頻度で利得を平均し，まだ何も観測していなければ 0 を返します．`selection_probabilities` ([learn/logit.rs#L143-L167](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/logit.rs#L143-L167)) は，数値的に安定させるために見積もりの最大値を引いてから指数をとり，正規化します．`sample_probabilities` ([learn/mod.rs#L130-L159](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/mod.rs#L130-L159)) は一様乱数を 1 つ引き，それが P(C) 未満なら C，そうでなければ D を返します．
- **`learn_step`** ([rule/learning.rs#L640-L655](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L640-L655))：各対戦から相手の手を取り出して `observe` を呼びます ([learn/logit.rs#L71-L85](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/logit.rs#L71-L85))．`observe` はその行動の回数に 1 を足し，`last_opponent` にも記録します (これを使うのは `LastOpponent` 方式だけです)．

`PerStep` では，戦略は `global` の見積もりを使います．`by_opponent` に相手ごとの見積もりを持つのは決定の単位が `PerEncounter` のときだけで，ここでは空のままです．`learn_encounter` も何もせずに戻ります．

**教科書との違い**：事前の信念はありません．1 ラウンド目が一様になるのは，事前の信念からではなく，見積もりがすべて 0 だからです．`roth_erev` との対戦では利得 S を `f64::MIN_POSITIVE` に置き換えています ([戦略の一覧](../strategies.ja.md) を参照)．これによる E[C] の変化は無視できるほど小さいものです．

## 参考文献

- Fudenberg, D., & Levine, D. K. (1998). *The theory of learning in games*. MIT Press.
- McKelvey, R. D., & Palfrey, T. R. (1995). Quantal response equilibria for normal form games. *Games and Economic Behavior*, 10(1), 6–38.

[← 戦略の一覧](../strategies.ja.md)
