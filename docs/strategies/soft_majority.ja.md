[English](soft_majority.md) | **日本語**

# Soft Majority (`soft_majority`)

Soft Majority は，それまでの試合全体で相手が多く出した手を出します．相手の C の回数が D の回数以上であるあいだは C を出し，そうでなければ D を出します．同数は協力に数えるので，最初は C です．

## 由来と性質

Soft Majority は Mathieu と Delahaye (2017) の古典的な戦略の 1 つで，そこでは `soft_majo` と呼ばれています．最初に C を出し，相手が C を出した回数が D を出した回数以上であるあいだ C を出す戦略です．«soft» は同数のときに C を出すことを指し，対になる Hard Majority は同数のときに D を出します．ここでの実装はこの説明に従っています．

Axelrod (1984) の言葉で言えば，次のとおりです．

- **上品 (nice)**：自分から先に D を出しません．相手が D を出すまでは相手の D の数が 0 なので，C を出す条件が必ず成り立ちます．
- **報復的 (retaliatory) だが，応えが遅い**：相手の前回の手ではなく，それまでの通算に反応します．C を多く積み上げた相手は，何度か D を出しても応えられません．
- **寛容 (forgiving) だが，記憶が長い**：相手の C の数が D の数に追いつけば C に戻りますが，D が長く続いたあとでは，同じだけの C が要ります．
- **TFT ほど明快 (clear) ではない**：手は履歴全体の回数で決まり，直前のラウンドを見ても分かりません．

決定的な戦略です (乱数を使いません)．

強みは安定していることです．おおむね協力する相手なら 1 回の D で報復の応酬が始まることはなく，D を出し続ける相手には D に落ち着きます．弱みは，D の数を C の数の少し下に保つ相手につけ込まれることです．`alternator` は回数を同数か C が 1 つ多い状態に保つので，Soft Majority は 1 回おきに D を出されながら，毎回 C を出し続けます．

## 動き

**状態なし．** Soft Majority は状態を持ちません．毎回 `Seen { opp, mine }` (相手の過去の手を古い順に並べたものと，自分の前回の手) から手を計算し直し，`init_state` は `Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))` を返します．使うのは `opp` だけです．

**手の決め方．** `opp` にある相手の C と D を数え，C ≥ D なら C，そうでなければ D を出します．1 ラウンド目はどちらも 0 なので C です．

**例．** 総当たりのプログラムが記録した，seed 0 での最初の 16 ラウンドです．最後の列は，そのラウンドの前に Soft Majority が見ている相手の C と D の回数です．

`fictitious_play` が相手のとき．相手は 2 ラウンド目から D を出し続けます．

| ラウンド | 相手 | Soft Majority | 見ている相手の C : D |
|---:|:-:|:-:|---|
| 1 | C | C | 0 : 0 |
| 2 | D | C | 1 : 0 |
| 3 | D | C | 1 : 1．同数なので C |
| 4 | D | D | 1 : 2 |
| 5–16 | D | D | D が多いまま |

`suspicious_tft` が相手のとき．2 人は位相をずらして交互に手を出します．

| ラウンド | 相手 | Soft Majority | 見ている相手の C : D |
|---:|:-:|:-:|---|
| 1 | D | C | 0 : 0 |
| 2 | C | D | 0 : 1 |
| 3 | D | C | 1 : 1 |
| 4 | C | D | 1 : 2 |
| 5 | D | C | 2 : 2 |
| 6 | C | D | 2 : 3 |
| 7–16 | D，C，… | C，D，… | 同じ繰り返し |

`alternator` が相手のとき．回数が D の側に傾くことはありません．

| ラウンド | 相手 | Soft Majority | 見ている相手の C : D |
|---:|:-:|:-:|---|
| 1 | C | C | 0 : 0 |
| 2 | D | C | 1 : 0 |
| 3 | C | C | 1 : 1 |
| 4 | D | C | 2 : 1 |
| 5 | C | C | 2 : 2 |
| 6–16 | D，C，… | C | 常に同数か，C が 1 つ多い |

`extort2` が相手のとき．序盤の協力のおかげで，相手の D は応えられずに済みます．

| ラウンド | 相手 | Soft Majority | 見ている相手の C : D |
|---:|:-:|:-:|---|
| 1–4 | C | C | 0 : 0〜3 : 0 |
| 5 | D | C | 4 : 0 |
| 6 | C | C | 4 : 1 |
| 7 | D | C | 5 : 1 |
| 8 | C | C | 5 : 2 |
| 9–16 | C | C | |

## コードの読み方

- **登録．** Soft Majority は，[`arena.rs#L20-L28`](../../simulation/src/arena.rs#L20-L28) に並べた 7 つの状態を持たない戦略の 1 つです (`("soft_majority", soft_majority)` は [L27](../../simulation/src/arena.rs#L27))．[`arena.rs#L30-L34`](../../simulation/src/arena.rs#L30-L34) のループが，各関数を `HistoryRule { name, choose }` に包み，[`rule_book`](../../simulation/src/arena.rs#L19-L64) の作る戦略集に登録します．
- **包む型．** [`strategies.rs#L17-L56`](../../simulation/src/strategies.rs#L17-L56) に，`Seen` ([L17-L21](../../simulation/src/strategies.rs#L17-L21))，それを意思決定の文脈から読む関数 `seen` ([L23-L35](../../simulation/src/strategies.rs#L23-L35))，関数の型 `Choose` ([L37-L38](../../simulation/src/strategies.rs#L37-L38))，`HistoryRule` ([L40-L56](../../simulation/src/strategies.rs#L40-L56)) を定義しています．`init_state` ([L50-L52](../../simulation/src/strategies.rs#L50-L52)) は状態を持たない戦略の状態を返し，`decide` ([L53-L55](../../simulation/src/strategies.rs#L53-L55)) は `(self.choose)(&seen(ctx)?, rng)` を呼びます．`learn_step` は定義していないので，何もしない既定のものが使われます．
- **戦略本体．** [`strategies.rs#L113-L122`](../../simulation/src/strategies.rs#L113-L122)．`s.opp` の C を数え，残りを D の数とし，`cooperations >= defections` なら C を返します．累計を持たず，毎回履歴全体を数え直します．乱数の引数は使いません．

**エンジンが各メソッドを呼ぶとき．** 1 試合は [`play_with`](../../simulation/src/arena.rs#L73-L140) の 1 回の呼び出しです．まず `build_world` ([`arena.rs#L100-L111`](../../simulation/src/arena.rs#L100-L111)) で新しいワールドを作ります．このとき `init_state` がエージェントごとに 1 回呼ばれ，各エージェントの `Policy` が `GameWorld` に保存されます．試合はそのあとの 200 回の `sim.step()` です ([`arena.rs#L124-L138`](../../simulation/src/arena.rs#L124-L138))．各ステップで，エンジンはエージェントの対戦ごとに `decide` を 1 回呼び (ここでは 1 ステップに 1 回)，そのあとエージェントごとに `learn_step` を 1 回呼びます．Soft Majority は状態に何も持たないので，回数は `decide` が毎回読む履歴から数えます．

**テスト．** [`tests.rs#L162-L173`](../../simulation/src/tests.rs#L162-L173) は，決まった履歴を与えて関数を呼びます．空の履歴では C，(C, D) のあと (同数) は C，(C, D, D) のあとは D です．

## 参考文献

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Mathieu, P., & Delahaye, J.-P. (2017). New winning strategies for the iterated prisoner's dilemma. *Journal of Artificial Societies and Social Simulation*, 20(4), 12.

[← 戦略の一覧](../strategies.ja.md)
