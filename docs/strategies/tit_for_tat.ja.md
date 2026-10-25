[English](tit_for_tat.md) | **日本語**

# Tit for Tat (`tit_for_tat`)

Tit for Tat は，1 ラウンド目に C を出し，以降は相手が前のラウンドに出した手をそのまま出します．

## 由来と性質

Tit for Tat は Anatol Rapoport が Axelrod の第 1 回コンピュータ選手権に投稿し，優勝した戦略です (Axelrod, 1980)．第 2 回の選手権でも優勝しました (Axelrod, 1984)．Axelrod は，その成功を次の 4 つの性質で説明しています．

- **Nice (非先制型)**：自分から先に裏切らない．
- **Retaliatory (provocable，報復的)**：相手の D には次のラウンドに D で応える．
- **Forgiving (寛容)**：相手が C に戻れば，すぐに C に戻る．
- **Clear (明快)**：振る舞いが相手から見て分かりやすい．

決定的な戦略です．1 回の対戦で相手より高い利得になることはなく，相手に勝つのではなく相手から協力を引き出すことで利得を得ます．知られている弱点は反響 (echo) です．TFT に似た 2 人の対戦に一度 D が入ると，両者は C と D を交互に出し続けます．ノイズの無いこの実験では，自分から先に D を出す戦略や，ランダムに D を出す戦略 (`suspicious_tft` や `joss`) が相手のときにだけ起きます．

## 動き

| ラウンド | 手 |
|---|---|
| 1 | C |
| t ≥ 2 | ラウンド t − 1 の相手の手 |

**状態**：`init_state` は `TftState { last_opponent: None }` を作ります．`learn_step` は毎ラウンドのあとに相手の手を `last_opponent` に書き込みます．`decide` は `last_opponent` を返し，それがまだ `None` なら C を返します．

**例** (seed 0 の最初の 16 ラウンド)

`joss` が相手のとき (乱数を含む 1 つの標本．Joss は TFT なら C を出す場面でランダムに D を出します)：

| ラウンド | 相手 | TFT | 備考 |
|---:|:-:|:-:|---|
| 1〜9 | C | C | 互いに協力 |
| 10 | D | C | Joss のランダムな裏切り |
| 11 | C | D | TFT が報復し，Joss は TFT の C をまねる |
| 12 | D | C | Joss は TFT の D をまねる |
| 13〜16 | C，D，C，D | D，C，D，C | 反響が続く |

`suspicious_tft` が相手のとき (決定的．初回は D，以降は相手の手をまねる)：

| ラウンド | 相手 | TFT | 備考 |
|---:|:-:|:-:|---|
| 1 | D | C | |
| 2 | C | D | 互いに相手の前の手をまねる |
| 3〜16 | D，C，… | C，D，… | 対戦の終わりまで，逆の位相で交互に出し続ける |

## コードの読み方

`tit_for_tat` は `TitForTat::new(C)` として登録しています ([arena.rs#L48-L50](../../simulation/src/arena.rs#L48-L50))．socsim-games には同じ手を出すオートマトンの種類 `AutomatonKind::Tft` もありますが，この実験では独立した戦略の方を使っています．その状態は相手の直前の手だけです．

- `TitForTat` 戦略：[rule/builtin.rs#L104-L117](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L104-L117)
- `TftState`：[world/state.rs#L40-L45](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/world/state.rs#L40-L45)

**エンジンが呼ぶ順番**：`init_state` は，`build_world` が対戦を組み立てるときにエージェントごとに 1 回だけ呼ばれ ([assemble.rs#L719-L727](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L719-L727))，返した状態はそのエージェントの分として `GameWorld` に保存されます．この実験は `PlayOptions::new()` で対戦を組み立てており ([arena.rs#L112-L121](../../simulation/src/arena.rs#L112-L121))，その決定の単位は `PerStep` です ([assemble.rs#L261-L271](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L261-L271))．そのためエンジンは，各ステップでエージェントごとに `decide` を 1 回呼びます ([mechanism/decide.rs#L248-L250](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L248-L250))．1 ステップの対戦は 1 つなので，1 ラウンドに 1 回です．そのステップのすべての対戦が終わったあと，`learn_step` が 1 回呼ばれます ([mechanism/decide.rs#L431-L438](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L431-L438))．どちらにもエージェントの状態が `&mut` で渡されます．

- **`init_state`** ([rule/builtin.rs#L129-L153](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L129-L153))：役割がちょうど 2 つで，それぞれ行動が 2 つあり，協力の行動が有効であることを確かめてから，`TftState::default()` を返します．
- **`decide`** ([rule/builtin.rs#L155-L163](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L155-L163))：`state.last_opponent.unwrap_or(self.cooperate)` の 1 行です．乱数は引きません．
- **`learn_step`** ([rule/builtin.rs#L165-L194](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/builtin.rs#L165-L194))：そのステップの対戦を順に見て，相手の役割 (自分が役割 0 なら 1，1 なら 0) を求め，その手を `last_opponent` に入れます．1 ステップに対戦が複数あれば最後のものが残りますが，ここでは 1 つだけです．

## 参考文献

- Axelrod, R. (1980). Effective choice in the prisoner's dilemma. *Journal of Conflict Resolution*, 24(1), 3–25.
- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.

[← 戦略の一覧](../strategies.ja.md)
