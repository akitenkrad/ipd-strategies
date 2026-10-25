[English](uniform_random.md) | **日本語**

# Uniform Random (`uniform_random`)

Uniform Random は，履歴と関係なく，毎ラウンド C と D を確率 1/2 ずつで出します．

## 由来と性質

等しい確率で協力と裏切りをランダムに選ぶ戦略は，Axelrod の第 1 回コンピュータ選手権に RANDOM として加えられ，比較の基準になりました．順位は最下位でした (Axelrod, 1980)．乱数を使い，記憶を持たない戦略です．

Axelrod の言葉では，nice (非先制型) でも (1 ラウンド目に確率 1/2 で自分から D を出します)，報復的でも寛容でもありません．相手にまったく反応しないからです．1 ラウンドあたりの期待利得は相手が協力する頻度だけで決まり，相手が確率 p で協力するなら平均 (3p + 0(1 − p) + 5p + 1(1 − p)) / 2 = (1 + 7p) / 2 です．応報する相手はこの戦略の D に D で応えるので，相手の手はたいてい C と D の混ざったものになります．

## 動き

毎ラウンド，C と D を確率 1/2 ずつで出します．相手の手は見ません．

**状態**：持ちません．`init_state` は `Stateless` を返し，戦略は `learn_step` を実装していません (トレイトの既定の実装は何もしません)．

**例** (seed 0 の最初の 16 ラウンド．乱数を含む 1 つの標本)

`all_c` が相手のとき：

| ラウンド | 相手 | Uniform Random | 備考 |
|---:|:-:|:-:|---|
| 1 | C | D | |
| 2 | C | D | |
| 3 | C | C | |
| 4〜16 | C | D C C D D D C C D C D D C | この 13 ラウンドで D が 7 回，C が 6 回 |

`tit_for_tat` が相手のとき：

| ラウンド | 相手 | Uniform Random | 備考 |
|---:|:-:|:-:|---|
| 1 | C | D | |
| 2 | D | D | TFT は 1 ラウンド目をまねる |
| 3 | D | C | |
| 4 | C | D | |
| 5〜16 | D C C D D D C C D C D D | C C D D D C C D C D D C | TFT の列は Uniform Random の列を 1 ラウンドずらしたもの |

seed 0 では，どちらの相手でも乱数の列 `DDCDCCDDDCCDCDDC` が同じです．どちらの相手も乱数を引かないので，乱数の列を使うのが Uniform Random だけだからです．

## コードの読み方

`uniform_random` は，ユニット構造体 `UniformRandom` として登録しています ([arena.rs#L54-L56](../../simulation/src/arena.rs#L54-L56))．パラメータはありません．

- `UniformRandom` 戦略：[rule/builtin.rs#L54-L102](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L54-L102)
- `Stateless`：[world/state.rs#L36-L38](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/world/state.rs#L36-L38)

**エンジンが呼ぶ順番**：`init_state` は，`build_world` が対戦を組み立てるときにエージェントごとに 1 回だけ呼ばれ ([assemble.rs#L719-L727](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L719-L727))，返した状態はそのエージェントの分として `GameWorld` に保存されます．この実験は `PlayOptions::new()` で対戦を組み立てており ([arena.rs#L112-L121](../../simulation/src/arena.rs#L112-L121))，その決定の単位は `PerStep` です ([assemble.rs#L261-L271](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L261-L271))．そのためエンジンは，各ステップでエージェントごとに `decide` を 1 回呼びます ([mechanism/decide.rs#L248-L250](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L248-L250))．1 ステップの対戦は 1 つなので，1 ラウンドに 1 回です．そのステップのすべての対戦が終わったあと，`learn_step` が 1 回呼ばれます ([mechanism/decide.rs#L431-L438](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L431-L438))．どちらにもエージェントの状態が `&mut` で渡されます．

- **`init_state`** ([rule/builtin.rs#L68-L70](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L68-L70))：`Stateless` を返します．
- **`decide`** ([rule/builtin.rs#L72-L89](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L72-L89))：自分の役割の行動の数を調べ，`rng.gen_range(0..n_actions)`，ここでは `gen_range(0..2)` を返します．乱数生成器は，エンジンがそのステップの `decide` に渡すものです．
- **`learn_step`**：実装していないので，`DecisionRule` トレイトの既定の実装 (何もしない) が使われます ([rule/mod.rs#L118-L124](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/mod.rs#L118-L124))．

## 参考文献

- Axelrod, R. (1980). Effective choice in the prisoner's dilemma. *Journal of Conflict Resolution*, 24(1), 3–25.

[← 戦略の一覧](../strategies.ja.md)
