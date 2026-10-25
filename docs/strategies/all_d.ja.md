[English](all_d.md) | **日本語**

# All-D (`all_d`)

All-D は，相手が何をしても毎ラウンド D を出します．

## 由来と性質

All-D は無条件の裏切り者です．1 回限りの囚人のジレンマでは D が支配戦略で，毎ラウンド D を出すことは有限回の繰り返しゲームの部分ゲーム完全均衡なので，All-D は協力しない側の標準的な比較の基準になります．Axelrod の言葉では nice ではありません (1 ラウンド目に自分から D を出します)．決定的な戦略です．

1 回の対戦で相手より低い利得になることはありません．無条件の協力者は搾取し (All-C が相手なら 1 ラウンドあたり T = 5)，Tit for Tat や Grim のように応報する相手には最初の 1 ラウンドだけ差をつけ，その後は互いに P = 1 になります．弱点は R = 3 を一度も得られないことです．応報する戦略が多い集団では，それらが互いに得る利得よりずっと低くなります (Axelrod, 1984)．

## 動き

手は常に D (行動 `1`) です．相手の手は見ません．

**状態**：`init_state` は種類 `AllD` の `AutomatonState` を作り，`AutomatonPolicyState` に包みます．`learn_step` は毎ラウンド直前の手の組をそこに書き込みますが，`decide` はそれを読みません．

**例** (seed 0 の最初の 16 ラウンド)

`tit_for_tat` が相手のとき：

| ラウンド | 相手 | All-D | 備考 |
|---:|:-:|:-:|---|
| 1 | C | D | All-D は T = 5，TFT は S = 0 |
| 2 | D | D | TFT は 1 ラウンド目の D をまねる |
| 3〜16 | D | D | 互いに裏切り，それぞれ P = 1 |

`grim` が相手のとき：

| ラウンド | 相手 | All-D | 備考 |
|---:|:-:|:-:|---|
| 1 | C | D | Grim の引き金が引かれる |
| 2〜16 | D | D | Grim は対戦の終わりまで D |

## コードの読み方

`all_d` は，この実験で協力の行動を `C` とした `Automaton` として登録している 4 つの戦略のひとつです ([arena.rs#L37-L47](../../simulation/src/arena.rs#L37-L47))．戦略とその状態は socsim-games にあります．

- `Automaton` 戦略：[rule/learning.rs#L818-L833](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L818-L833)
- `AutomatonKind` と `AutomatonState`：[learn/automaton.rs#L9-L61](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L9-L61)

**エンジンが呼ぶ順番**：`init_state` は，`build_world` が対戦を組み立てるときにエージェントごとに 1 回だけ呼ばれ ([assemble.rs#L719-L727](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L719-L727))，返した状態はそのエージェントの分として `GameWorld` に保存されます．この実験は `PlayOptions::new()` で対戦を組み立てており ([arena.rs#L112-L121](../../simulation/src/arena.rs#L112-L121))，その決定の単位は `PerStep` です ([assemble.rs#L261-L271](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L261-L271))．そのためエンジンは，各ステップでエージェントごとに `decide` を 1 回呼びます ([mechanism/decide.rs#L248-L250](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L248-L250))．1 ステップの対戦は 1 つなので，1 ラウンドに 1 回です．そのステップのすべての対戦が終わったあと，`learn_step` が 1 回呼ばれます ([mechanism/decide.rs#L431-L438](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L431-L438))．どちらにもエージェントの状態が `&mut` で渡されます．

- **`init_state`** ([rule/learning.rs#L858-L864](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L858-L864))：どの役割も行動がちょうど 2 つであることを確かめ，オートマトンを作ります．
- **`decide`** ([rule/learning.rs#L866-L883](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L866-L883))：`next_action` を呼びます．`AllD` なら裏切りの行動 `1 - coop` を返します ([learn/automaton.rs#L144-L148](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L144-L148))．乱数は引きません．オートマトンの種類のうち，C から始めないのは `AllD` だけです．
- **`learn_step`** ([rule/learning.rs#L885-L900](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L885-L900))：両者の手を `observe` に渡します ([learn/automaton.rs#L175-L191](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L175-L191))．記録した値は手を変えません．

`PerStep` では，戦略は `AutomatonPolicyState` の `global` を使います．`by_opponent` を使うのは決定の単位が `PerEncounter` のときだけで，ここでは空のままです．

## 参考文献

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.

[← 戦略の一覧](../strategies.ja.md)
