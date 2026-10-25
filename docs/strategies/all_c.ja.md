[English](all_c.md) | **日本語**

# All-C (`all_c`)

All-C は，相手が何をしても毎ラウンド C を出します．

## 由来と性質

All-C は無条件の協力者で，繰り返し囚人のジレンマの文献でいちばん単純な比較の基準です．Axelrod の言葉では，nice (自分から先に裏切らない) で forgiving (寛容) ですが，retaliatory (報復的) ではありません．相手が D を出しても何も起きません．決定的な戦略です．

弱点は，いくらでも搾取されることです．All-D が相手なら，毎ラウンド自分は S = 0，相手は T = 5 を得ます．相手が応報する戦略なら，互いに協力し続けて R = 3 を得ます．Nowak と Sigmund (1993) は，Win-Stay, Lose-Shift は Tit for Tat と違って無条件の協力者を搾取できると述べています．ただしこの総当たりでは，`wsls` が自分から先に D を出すことはないので，その差は現れません．

## 動き

手は常に C (行動 `0`) です．相手の手は見ません．

**状態**：`init_state` は種類 `AllC` の `AutomatonState` を作り，`AutomatonPolicyState` に包みます．`learn_step` は毎ラウンド直前の手の組をそこに書き込みますが，`decide` はそれを読みません．

**例** (seed 0 の最初の 16 ラウンド)

`all_d` が相手のとき：

| ラウンド | 相手 | All-C | 備考 |
|---:|:-:|:-:|---|
| 1 | D | C | All-C は S = 0 |
| 2〜16 | D | C | 毎ラウンド同じ |

`tit_for_tat` が相手のとき：

| ラウンド | 相手 | All-C | 備考 |
|---:|:-:|:-:|---|
| 1 | C | C | TFT は C から始める |
| 2〜16 | C | C | 互いに協力し，それぞれ R = 3 |

## コードの読み方

`all_c` は，この実験で協力の行動を `C` とした `Automaton` として登録している 4 つの戦略のひとつです ([arena.rs#L37-L47](../../simulation/src/arena.rs#L37-L47))．戦略とその状態は socsim-games にあります．

- `Automaton` 戦略：[rule/learning.rs#L818-L833](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L818-L833)
- `AutomatonKind` と `AutomatonState`：[learn/automaton.rs#L9-L61](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L9-L61)

**エンジンが呼ぶ順番**：`init_state` は，`build_world` が対戦を組み立てるときにエージェントごとに 1 回だけ呼ばれ ([assemble.rs#L719-L727](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L719-L727))，返した状態はそのエージェントの分として `GameWorld` に保存されます．この実験は `PlayOptions::new()` で対戦を組み立てており ([arena.rs#L112-L121](../../simulation/src/arena.rs#L112-L121))，その決定の単位は `PerStep` です ([assemble.rs#L261-L271](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L261-L271))．そのためエンジンは，各ステップでエージェントごとに `decide` を 1 回呼びます ([mechanism/decide.rs#L248-L250](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L248-L250))．1 ステップの対戦は 1 つなので，1 ラウンドに 1 回です．そのステップのすべての対戦が終わったあと，`learn_step` が 1 回呼ばれます ([mechanism/decide.rs#L431-L438](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L431-L438))．どちらにもエージェントの状態が `&mut` で渡されます．

- **`init_state`** ([rule/learning.rs#L858-L864](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L858-L864))：どの役割も行動がちょうど 2 つであることを確かめ，オートマトンを作ります．
- **`decide`** ([rule/learning.rs#L866-L883](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L866-L883))：`next_action` を呼びます．`AllC` なら協力の行動を返します ([learn/automaton.rs#L144-L148](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L144-L148))．乱数は引きません．
- **`learn_step`** ([rule/learning.rs#L885-L900](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L885-L900))：両者の手を `observe` に渡します ([learn/automaton.rs#L175-L191](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L175-L191))．記録した値は手を変えません．

`PerStep` では，戦略は `AutomatonPolicyState` の `global` を使います．`by_opponent` を使うのは決定の単位が `PerEncounter` のときだけで，ここでは空のままです．

## 参考文献

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Nowak, M., & Sigmund, K. (1993). A strategy of win-stay, lose-shift that outperforms tit-for-tat in the Prisoner's Dilemma game. *Nature*, 364, 56–58.

[← 戦略の一覧](../strategies.ja.md)
