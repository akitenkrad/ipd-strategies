[English](grim.md) | **日本語**

# Grim Trigger (`grim`)

Grim は，相手が一度でも D を出すまでは C を出し，出したあとは対戦の終わりまで D を出します．

## 由来と性質

Grim は，無限回の繰り返しゲームで協力が非協力均衡として維持できることを示すために Friedman (1971) が用いた引き金戦略 (trigger strategy) です．逸脱が一度でもあれば，以降はずっと 1 回限りのゲームの均衡に戻ることで罰します．Axelrod (1984) はこの論文にちなんで，この戦略を «Friedman» と呼びました．

Axelrod の言葉では，nice (非先制型．自分から先に裏切らない) で，報復の強さは最大ですが，まったく forgiving (寛容) ではありません．相手の D が一度あれば，協力は二度と戻りません．決定的な戦略です．

強みは，1 ラウンドを超えて搾取されないことです．弱点は同じ容赦の無さです．探りの D，ランダムな D，誤りの D のどれであっても，一度の D で両者は対戦の終わりまで互いに裏切る状態 (P = 1) に閉じ込められます．相手が協力に戻ろうとしても戻れません．

## 動き

| モード | 手 | 相手の手を見るか | 抜ける条件 |
|---|:-:|:-:|---|
| 協力中 (引き金が引かれていない) | C | 見る | 相手が一度 D を出す |
| 引き金が引かれた | D | 見ない | 抜けない |

引き金は相手の手だけで決まり，Grim 自身の手では引かれません．

**状態**：`init_state` は種類 `Grim`，`grim_triggered: false` の `AutomatonState` を作り，`AutomatonPolicyState` に包みます．`learn_step` は，相手の手が初めて C でなかったときに `grim_triggered` を `true` にし，二度と戻しません．

**例** (seed 0 の最初の 16 ラウンド)

`joss` が相手のとき (乱数を含む 1 つの標本．Joss は TFT なら C を出す場面でランダムに D を出します)：

| ラウンド | 相手 | Grim | 備考 |
|---:|:-:|:-:|---|
| 1〜9 | C | C | 互いに協力 |
| 10 | D | C | Joss のランダムな裏切りで引き金が引かれる |
| 11 | C | D | Grim はここから D |
| 12 | D | D | Joss は Grim の D をまねる |
| 13〜16 | D | D | 互いに裏切り |

`alternator` が相手のとき (決定的．C から始めて C と D を交互に出す)：

| ラウンド | 相手 | Grim | 備考 |
|---:|:-:|:-:|---|
| 1 | C | C | |
| 2 | D | C | 引き金が引かれる |
| 3〜16 | C，D，… | D | Grim はもう相手の手を見ない |

## コードの読み方

`grim` は，この実験で協力の行動を `C` とした `Automaton` として登録している 4 つの戦略のひとつです ([arena.rs#L37-L47](../../simulation/src/arena.rs#L37-L47))．戦略とその状態は socsim-games にあります．

- `Automaton` 戦略：[rule/learning.rs#L818-L833](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L818-L833)
- `AutomatonKind` と `AutomatonState`：[learn/automaton.rs#L9-L61](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L9-L61)

**エンジンが呼ぶ順番**：`init_state` は，`build_world` が対戦を組み立てるときにエージェントごとに 1 回だけ呼ばれ ([assemble.rs#L719-L727](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L719-L727))，返した状態はそのエージェントの分として `GameWorld` に保存されます．この実験は `PlayOptions::new()` で対戦を組み立てており ([arena.rs#L112-L121](../../simulation/src/arena.rs#L112-L121))，その決定の単位は `PerStep` です ([assemble.rs#L261-L271](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L261-L271))．そのためエンジンは，各ステップでエージェントごとに `decide` を 1 回呼びます ([mechanism/decide.rs#L248-L250](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L248-L250))．1 ステップの対戦は 1 つなので，1 ラウンドに 1 回です．そのステップのすべての対戦が終わったあと，`learn_step` が 1 回呼ばれます ([mechanism/decide.rs#L431-L438](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L431-L438))．どちらにもエージェントの状態が `&mut` で渡されます．

- **`init_state`** ([rule/learning.rs#L858-L864](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L858-L864))：どの役割も行動がちょうど 2 つであることを確かめ，`grim_triggered: false` のオートマトンを作ります．
- **`decide`** ([rule/learning.rs#L866-L883](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L866-L883))：`next_action` を呼びます．`Grim` なら，`grim_triggered` が立っていれば D，そうでなければ C を返します ([learn/automaton.rs#L144-L153](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L144-L153))．乱数は引きません．
- **`learn_step`** ([rule/learning.rs#L885-L900](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L885-L900))：両者の手を `observe` に渡します ([learn/automaton.rs#L175-L191](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L175-L191))．`observe` は，相手の手が `coop` と違えば `grim_triggered` を立てます．

`PerStep` では，戦略は `AutomatonPolicyState` の `global` を使います．`by_opponent` を使うのは決定の単位が `PerEncounter` のときだけで，ここでは空のままです．

## 参考文献

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Friedman, J. W. (1971). A non-cooperative equilibrium for supergames. *The Review of Economic Studies*, 38(1), 1–12.

[← 戦略の一覧](../strategies.ja.md)
