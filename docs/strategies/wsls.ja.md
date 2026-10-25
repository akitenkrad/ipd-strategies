[English](wsls.md) | **日本語**

# Win-Stay, Lose-Shift (`wsls`)

Win-Stay, Lose-Shift は，よいラウンドのあとは前の手を繰り返し，悪いラウンドのあとは手を切り替えます．この利得では，前のラウンドの自分と相手の手が同じなら C，違えば D になります．

## 由来と性質

Nowak と Sigmund (1993) は，記憶長 1 の戦略に突然変異と選択をかける進化シミュレーションで，この戦略 (彼らの呼び名では Pavlov) が勝ち残ることを示しました．利得が R か T のラウンド，つまり相手が協力したラウンドを勝ちとし，勝ちのあとは同じ手を続け，負け (S か P) のあとは手を切り替えます．

Axelrod の言葉では，nice (非先制型．自分から先に裏切らない) で報復的です．決定的な戦略です．Nowak と Sigmund は Tit for Tat に対する利点を 2 つ挙げています．WSLS どうしなら，ときおりの誤りを反響させずに 2 ラウンドで立て直せること，そして誤りのあとに無条件の協力者を搾取できることです．弱点は All-D です．D に負けると C に切り替えるので，All-D が相手だと 1 ラウンドおきに協力してしまいます．

戦略の全体は次の表です (「前」は前のラウンドの手の組)．

| 前 (自分, 相手) | 勝ち負け | 次の手 |
|---|---|:-:|
| (C, C) | 勝ち (R) | C |
| (C, D) | 負け (S) | D |
| (D, C) | 勝ち (T) | D |
| (D, D) | 負け (P) | C |

## 動き

| ラウンド | 手 |
|---|---|
| 1 | C |
| t ≥ 2 | ラウンド t − 1 に相手が C なら，ラウンド t − 1 の自分の手を繰り返す．そうでなければ切り替える |

**状態**：`init_state` は種類 `Wsls` の `AutomatonState` を作り，`AutomatonPolicyState` に包みます．`learn_step` はそのラウンドの手の組を `own_last` と `opponent_last` に書き込み，`decide` は両方を読みます．

**例** (seed 0 の最初の 16 ラウンド)

`all_d` が相手のとき：

| ラウンド | 相手 | WSLS | 備考 |
|---:|:-:|:-:|---|
| 1 | D | C | 負け (S) |
| 2 | D | D | D に切り替える．負け (P) |
| 3 | D | C | C に切り替え直す |
| 4〜16 | D | D，C，… | 対戦の終わりまで C と D を交互に出す |

`suspicious_tft` が相手のとき (初回は D，以降は相手の前の手をまねる)：

| ラウンド | 相手 | WSLS | 備考 |
|---:|:-:|:-:|---|
| 1 | D | C | 負け (S) |
| 2 | C | D | WSLS は切り替え，STFT は C をまねる．勝ち (T) |
| 3 | D | D | WSLS は続け，STFT は D をまねる．負け (P) |
| 4 | D | C | WSLS は切り替え，STFT は D をまねる．負け (S) |
| 5〜16 | C，D，D，… | D，D，C，… | 3 ラウンドの周期が繰り返す |

## コードの読み方

`wsls` は，この実験で協力の行動を `C` とした `Automaton` として登録している 4 つの戦略のひとつです ([arena.rs#L37-L47](../../simulation/src/arena.rs#L37-L47))．戦略とその状態は socsim-games にあります．

- `Automaton` 戦略：[rule/learning.rs#L818-L833](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L818-L833)
- `AutomatonKind` と `AutomatonState`：[learn/automaton.rs#L9-L61](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L9-L61)

**エンジンが呼ぶ順番**：`init_state` は，`build_world` が対戦を組み立てるときにエージェントごとに 1 回だけ呼ばれ ([assemble.rs#L719-L727](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L719-L727))，返した状態はそのエージェントの分として `GameWorld` に保存されます．この実験は `PlayOptions::new()` で対戦を組み立てており ([arena.rs#L112-L121](../../simulation/src/arena.rs#L112-L121))，その決定の単位は `PerStep` です ([assemble.rs#L261-L271](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L261-L271))．そのためエンジンは，各ステップでエージェントごとに `decide` を 1 回呼びます ([mechanism/decide.rs#L248-L250](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L248-L250))．1 ステップの対戦は 1 つなので，1 ラウンドに 1 回です．そのステップのすべての対戦が終わったあと，`learn_step` が 1 回呼ばれます ([mechanism/decide.rs#L431-L438](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L431-L438))．どちらにもエージェントの状態が `&mut` で渡されます．

- **`init_state`** ([rule/learning.rs#L858-L864](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L858-L864))：どの役割も行動がちょうど 2 つであることを確かめ，`own_last` と `opponent_last` がどちらも `None` のオートマトンを作ります．
- **`decide`** ([rule/learning.rs#L866-L883](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L866-L883))：`next_action` を呼びます ([learn/automaton.rs#L129-L158](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L129-L158))．`Wsls` なら，`opponent_last` が C のとき `own_last` を，D のとき他方の行動を返し，前のラウンドが無いうちは C を返します．その上のドキュメントコメントにも同じ 4 行の表があります．乱数は引きません．
- **`learn_step`** ([rule/learning.rs#L885-L900](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L885-L900))：両者の手を `observe` に渡します ([learn/automaton.rs#L175-L191](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/automaton.rs#L175-L191))．`observe` はそれを `own_last` と `opponent_last` に入れます．

実装は，勝ちを利得と要求水準の比較ではなく，相手の手で決めています．R = 3，T = 5，S = 0，P = 1 では 2 つの定義は一致します．R と T は利得の上位 2 つで，ちょうど相手が協力したラウンドにあたるからです．

`PerStep` では，戦略は `AutomatonPolicyState` の `global` を使います．`by_opponent` を使うのは決定の単位が `PerEncounter` のときだけで，ここでは空のままです．

## 参考文献

- Nowak, M., & Sigmund, K. (1993). A strategy of win-stay, lose-shift that outperforms tit-for-tat in the Prisoner's Dilemma game. *Nature*, 364, 56–58.

[← 戦略の一覧](../strategies.ja.md)
