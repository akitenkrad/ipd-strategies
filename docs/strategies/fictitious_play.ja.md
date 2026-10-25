[English](fictitious_play.md) | **日本語**

# Fictitious Play (`fictitious_play`)

Fictitious Play は，相手がこれまでに C と D をそれぞれ何回出したかを数え，相手が決まった分布から手を引いているとみなして，その頻度への最適反応を出します．

## 由来と性質

Brown (1951) は，ゲームを解く反復法として fictitious play を提案しました．各プレイヤーは，相手の過去の手の経験分布を混合戦略とみなし，それに最適反応します．これはゲームにおける信念に基づく学習の基本モデルのひとつになりました (Fudenberg & Levine, 1998)．ここでは同点を行動番号の小さい方で決めるので，決定的な戦略です．

Fictitious play は，各ラウンドを 1 回限りのゲームとして扱います．囚人のジレンマでは，D は相手の C に対しても (5 > 3) D に対しても (1 > 0) C より利得が高いので，どんな信念に対しても D が最適反応です．そのため，相手の手を 1 回でも数えた時点で，相手が何をしても Fictitious Play は D を出します．相手が自分の手に反応することをモデルに含めないので，応報する相手に協力すれば得をすることが見えません．Axelrod の言葉では nice ではありません．挑発されていないのに 2 ラウンド目に D を出します．

## 動き

各行動の値は，相手の行動ごとの回数と利得の積の和です．

value(a) = n_C · u(a, C) + n_D · u(a, D)

n_C と n_D は，それまでに相手が C と D を出した回数です．値の大きい方の行動を出し，同点なら行動番号の小さい方，つまり C を出します．

| ラウンド | 回数 | value(C)，value(D) | 手 |
|---|---|---|:-:|
| 1 | n_C = n_D = 0 | 0，0 (同点) | C |
| t ≥ 2 | n_C + n_D = t − 1 ≥ 1 | 3 n_C，5 n_C + n_D | D |

value(D) − value(C) = 2 n_C + n_D は，1 回でも手を数えれば正になります．このゲームでは，この戦略は「1 ラウンド目は C，2 ラウンド目からは D」と同じです．

**状態**：`init_state` は `BeliefState` を作ります．その `global` は，`counts = [0.0, 0.0]` (擬似回数なし) と同点の決め方 `LowestIndex` を持つ `FictitiousBelief` です．`learn_step` は毎ラウンドのあとに相手の手の回数に 1 を足し，`decide` は回数を読みます．

**例** (seed 0 の最初の 16 ラウンド)

`all_c` が相手のとき：

| ラウンド | 相手 | Fictitious Play | 備考 |
|---:|:-:|:-:|---|
| 1 | C | C | まだ観測が無く，同点で C |
| 2 | C | D | n_C = 1：value(C) = 3，value(D) = 5 |
| 3〜16 | C | D | 以降は All-C を搾取する |

`tit_for_tat` が相手のとき：

| ラウンド | 相手 | Fictitious Play | 備考 |
|---:|:-:|:-:|---|
| 1 | C | C | |
| 2 | C | D | |
| 3 | D | D | TFT は 2 ラウンド目の D をまねる |
| 4〜16 | D | D | 互いに裏切り |

## コードの読み方

`fictitious_play` は `FictitiousPlay::new()` として登録しています ([arena.rs#L51-L53](../../simulation/src/arena.rs#L51-L53))．初期の擬似回数は 0，同点は行動番号の小さい方で決めます．

- `FictitiousPlay` 戦略とコンストラクタ：[rule/learning.rs#L379-L393](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L379-L393)
- `BeliefState`：[world/state.rs#L58-L82](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/world/state.rs#L58-L82)
- `FictitiousBelief`：[learn/fictitious.rs#L9-L47](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/fictitious.rs#L9-L47)

**エンジンが呼ぶ順番**：`init_state` は，`build_world` が対戦を組み立てるときにエージェントごとに 1 回だけ呼ばれ ([assemble.rs#L719-L727](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L719-L727))，返した状態はそのエージェントの分として `GameWorld` に保存されます．この実験は `PlayOptions::new()` で対戦を組み立てており ([arena.rs#L112-L121](../../simulation/src/arena.rs#L112-L121))，その決定の単位は `PerStep` です ([assemble.rs#L261-L271](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/assemble.rs#L261-L271))．そのためエンジンは，各ステップでエージェントごとに `decide` を 1 回呼びます ([mechanism/decide.rs#L248-L250](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L248-L250))．1 ステップの対戦は 1 つなので，1 ラウンドに 1 回です．そのステップのすべての対戦が終わったあと，`learn_step` が 1 回呼ばれます ([mechanism/decide.rs#L431-L438](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/mechanism/decide.rs#L431-L438))．どちらにもエージェントの状態が `&mut` で渡されます．

- **`init_state`** ([rule/learning.rs#L460-L472](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L460-L472))：役割がちょうど 2 つであることを確かめ，`initial` で初期の信念を作ります ([rule/learning.rs#L413-L429](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L413-L429))．初期の回数が指定されていなければ，相手の行動ごとに 0 を使います．
- **`decide`** ([rule/learning.rs#L474-L495](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L474-L495))：`best_response` を呼びます ([learn/fictitious.rs#L75-L116](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/fictitious.rs#L75-L116))．value(a) を正規化していない回数のまま計算し (最適反応は正規化しても変わりません)，最大値と厳密に等しい行動をすべて集めて `resolve_tie` に渡します ([learn/mod.rs#L161-L178](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/mod.rs#L161-L178))．`LowestIndex` では最初の候補を選び，乱数は引きません．
- **`learn_step`** ([rule/learning.rs#L497-L512](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/rule/learning.rs#L497-L512))：各対戦から相手の手を取り出して `observe` を呼びます ([learn/fictitious.rs#L164-L182](https://github.com/akitenkrad/rs-social-simulation-tools/blob/3b246d2b9f4e6f6a92d152ed367304d588249bc2/crates/socsim-games/src/learn/fictitious.rs#L164-L182))．`observe` はその行動の回数に 1 を足します．

`PerStep` では，戦略は `global` の信念を使います．`by_opponent` に相手ごとの信念を持つのは決定の単位が `PerEncounter` のときだけで，ここでは空のままです．`learn_encounter` も何もせずに戻ります．

**教科書との違い**：教科書の fictitious play は，事前の信念 (正の初期回数) から始めることが多いです．ここでは回数が 0 から始まるので，最初の手は同点の決め方で決まり，C が行動 `0` なので C になります．事前の信念があれば，このゲームではどんな信念に対しても D が最適反応なので，最初の手も D になります．

## 参考文献

- Brown, G. W. (1951). Iterative solution of games by fictitious play. In T. C. Koopmans (Ed.), *Activity analysis of production and allocation* (pp. 374–376). Wiley.
- Fudenberg, D., & Levine, D. K. (1998). *The theory of learning in games*. MIT Press.

[← 戦略の一覧](../strategies.ja.md)
