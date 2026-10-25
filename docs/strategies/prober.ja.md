[English](prober.md) | **日本語**

# Prober (`prober`)

Prober は，最初の 3 回に D，C，C を出して相手を試します．相手が 2 回目と 3 回目の両方で C を出した，つまり最初の D に応えなかった場合は，つけ込める相手とみなして試合の最後まで D を出し続けます．そうでなければ，4 回目以降は Tit for Tat (TFT) を使います．

## 由来と性質

Prober は Mathieu と Delahaye (2017) の古典的な戦略の 1 つです．彼らは，D，C，C と出したあと，相手が 2 手目と 3 手目に C を出していれば以降ずっと D を出し，そうでなければ TFT を使う戦略として説明しています．ここでの実装はこの説明に従っています．

Axelrod (1984) の言葉で言えば，次のとおりです．

- **非先制型ではない (not nice)**：1 ラウンド目に D を出します．
- **報復的 (retaliatory)**：TFT に移ったあとは，相手の D にすべて応えます．
- **寛容 (forgiving) なのは TFT に移ったときだけ**：そのときは相手が C に戻ればすぐ C に戻ります．試しの結果つけ込めると判断した相手には，二度と C を出しません．
- **かなり明快 (clear)**：最初の 3 手は決まっていて，そのあとは TFT かずっと D かのどちらかです．

決定的な戦略です (乱数を使いません)．

強みは，1 回の D を罰さない相手につけ込めることです．`all_c` や，2 回続いた D にしか応えない Tit for Two Tats がそうです．報復する相手に対しては，試しの代償は小さく，TFT は最初の D に 1 回応え，そのあとは双方が協力します．弱みは最初の D そのものです．`grim` のように決して許さない相手は 1 ラウンド目のあとずっと D を出し，TFT に移った Prober はその D をまねるので，試合は相互裏切りのまま終わります．

## 動き

**状態なし．** Prober は状態を持ちません．毎回 `Seen { opp, mine }` (相手の過去の手を古い順に並べたものと，自分の前回の手) から手を計算し直し，`init_state` は `Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))` を返します．使うのは `opp` だけです．

| ラウンド | 出す手 |
|---|---|
| 1 | D |
| 2，3 | 相手の手にかかわらず C |
| 4 以降 | 相手が 2 ラウンド目と 3 ラウンド目の両方で C を出していれば D．そうでなければ相手の前回の手 (TFT) |

相手の 2 ラウンド目と 3 ラウンド目の手は，Prober の 1 ラウンド目の D と 2 ラウンド目の C への応えです．この判定は 4 ラウンド目以降の毎回やり直しますが，2 ラウンド目と 3 ラウンド目の手は変わらないので結果は毎回同じで，覚えておくための状態は要りません．相手が 2 ラウンド目か 3 ラウンド目 (またはその両方) で D を出せば，Prober は TFT に移ります．

**例．** 総当たりのプログラムが記録した，seed 0 での最初の 16 ラウンドです．

`all_c` が相手のとき．試しの結果，つけ込める相手だと分かります．

| ラウンド | 相手 | Prober | 備考 |
|---:|:-:|:-:|---|
| 1 | C | D | 最初の 3 手 |
| 2 | C | C | 最初の 3 手 |
| 3 | C | C | 最初の 3 手 |
| 4 | C | D | 相手は 2，3 ラウンド目に C：以降ずっと D |
| 5–16 | C | D | |

`tit_for_tat` が相手のとき．相手が最初の D に応えるので，Prober は TFT に移ります．

| ラウンド | 相手 | Prober | 備考 |
|---:|:-:|:-:|---|
| 1 | C | D | 最初の 3 手 |
| 2 | D | C | TFT が 1 ラウンド目に応える |
| 3 | C | C | 最初の 3 手 |
| 4 | C | C | 相手は 2 ラウンド目に D：TFT．3 ラウンド目をまねる |
| 5–16 | C | C | 相互協力 |

`tit_for_two_tats` が相手のとき．最初の D 1 回には応えません．

| ラウンド | 相手 | Prober | 備考 |
|---:|:-:|:-:|---|
| 1 | C | D | 最初の 3 手 |
| 2 | C | C | TF2T は 1 回の D を見逃す |
| 3 | C | C | 最初の 3 手 |
| 4 | C | D | 相手は 2，3 ラウンド目に C：以降ずっと D |
| 5 | C | D | |
| 6 | D | D | TF2T が 4，5 ラウンド目の D に応えるが，遅い |
| 7–16 | D | D | 相互裏切り |

`grim` が相手のとき．最初の D は決して許されません．

| ラウンド | 相手 | Prober | 備考 |
|---:|:-:|:-:|---|
| 1 | C | D | 最初の 3 手 |
| 2 | D | C | Grim が切り替わる |
| 3 | D | C | 最初の 3 手 |
| 4 | D | D | 相手は 2，3 ラウンド目に D：TFT．3 ラウンド目をまねる |
| 5–16 | D | D | 相互裏切り |

## コードの読み方

- **登録．** Prober は，[`arena.rs#L20-L28`](../../simulation/src/arena.rs#L20-L28) に並べた 7 つの状態を持たない戦略の 1 つです (`("prober", prober)` は [L26](../../simulation/src/arena.rs#L26))．[`arena.rs#L30-L34`](../../simulation/src/arena.rs#L30-L34) のループが，各関数を `HistoryRule { name, choose }` に包み，[`rule_book`](../../simulation/src/arena.rs#L19-L64) の作る戦略集に登録します．
- **包む型．** [`strategies.rs#L17-L56`](../../simulation/src/strategies.rs#L17-L56) に，`Seen` ([L17-L21](../../simulation/src/strategies.rs#L17-L21))，それを意思決定の文脈から読む関数 `seen` ([L23-L35](../../simulation/src/strategies.rs#L23-L35))，関数の型 `Choose` ([L37-L38](../../simulation/src/strategies.rs#L37-L38))，`HistoryRule` ([L40-L56](../../simulation/src/strategies.rs#L40-L56)) を定義しています．`init_state` ([L50-L52](../../simulation/src/strategies.rs#L50-L52)) は状態を持たない戦略の状態を返し，`decide` ([L53-L55](../../simulation/src/strategies.rs#L53-L55)) は `(self.choose)(&seen(ctx)?, rng)` を呼びます．`learn_step` は定義していないので，何もしない既定のものが使われます．
- **戦略本体．** [`strategies.rs#L103-L111`](../../simulation/src/strategies.rs#L103-L111)．`s.opp.len()`，つまりすでに終わったラウンド数で分岐します．0 なら D，1 か 2 なら C，それ以外では `s.opp[1]` と `s.opp[2]`，すなわち相手の 2 ラウンド目と 3 ラウンド目の手を調べます (添字は 0 から始まります)．乱数の引数は使いません．

**エンジンが各メソッドを呼ぶとき．** 1 試合は [`play_with`](../../simulation/src/arena.rs#L73-L140) の 1 回の呼び出しです．まず `build_world` ([`arena.rs#L100-L111`](../../simulation/src/arena.rs#L100-L111)) で新しいワールドを作ります．このとき `init_state` がエージェントごとに 1 回呼ばれ，各エージェントの `Policy` が `GameWorld` に保存されます．試合はそのあとの 200 回の `sim.step()` です ([`arena.rs#L124-L138`](../../simulation/src/arena.rs#L124-L138))．各ステップで，エンジンはエージェントの対戦ごとに `decide` を 1 回呼び (ここでは 1 ステップに 1 回)，そのあとエージェントごとに `learn_step` を 1 回呼びます．Prober は状態に何も持たないので，必要なものはすべて，`decide` が毎回読む履歴から得ます．

**テスト．** [`tests.rs#L151-L159`](../../simulation/src/tests.rs#L151-L159) は，決まった履歴を与えて関数を呼びます．空の履歴では D，1 ラウンド後と 2 ラウンド後は C，(D, C, C) のあとは D，(D, D, C) と (D, C, D) のあとは TFT です．

## 参考文献

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Mathieu, P., & Delahaye, J.-P. (2017). New winning strategies for the iterated prisoner's dilemma. *Journal of Artificial Societies and Social Simulation*, 20(4), 12.

[← 戦略の一覧](../strategies.ja.md)
