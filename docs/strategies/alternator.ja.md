[English](alternator.md) | **日本語**

# Alternator (`alternator`)

Alternator は，C から始めて C，D，C，D，… を試合の最後まで出し続けます．相手の手はまったく見ず，次の手はラウンドだけで決まります．

## 由来と性質

Alternator は決まった周期で手を出す戦略で，総当たりでは本気の候補というより比較の相手として使われます．Mathieu と Delahaye (2017) は，同じ振る舞いを `per_cd` (C と D を周期的に出す) という名前で古典的な戦略の 1 つに入れています．

Axelrod (1984) の言葉で言えば，次のとおりです．

- **上品ではない (not nice)**：相手の手にかかわらず，2 ラウンド目に D を出します．
- **報復的でも寛容でもない**：相手にまったく反応しないので，これらの性質は当てはまりません．
- **明快 (clear)**：相手が何をしても手が変わらないので，読むのはたやすい戦略です．

決定的な戦略です (乱数を使いません)．

Alternator の使いみちは，主にほかの戦略を試すことです．無条件に協力する相手からは 1 回おきに T を得ますし，エスカレートせずに周期的に D を出す相手に，ほかの戦略がどう反応するかを見られます．TFT とは位相がずれて (D, C) と (C, D) が交互に続き，`grim` には最初の D のあとずっと罰せられます．

## 動き

**状態．** `Policy::Alternator { next }` で，そのラウンドに出す手です．C から始まります．`decide` は `next` を返し，各ステップの終わりに `learn_step` がそれを入れ替えます (C → D，D → C)．

**例．** 総当たりのプログラムが記録した，seed 0 での最初の 16 ラウンドのうち，最初の 6 ラウンドです．どの試合でも，Alternator は奇数ラウンドに C，偶数ラウンドに D を出します．

`all_c` が相手のとき．

| ラウンド | 相手 | Alternator | 備考 |
|---:|:-:|:-:|---|
| 1 | C | C | |
| 2 | C | D | 協力する相手につけ込む |
| 3 | C | C | |
| 4 | C | D | |
| 5 | C | C | |
| 6 | C | D | 同じ繰り返し |

`tit_for_tat` が相手のとき．TFT は Alternator を 1 ラウンド遅れでまねます．

| ラウンド | 相手 | Alternator | 備考 |
|---:|:-:|:-:|---|
| 1 | C | C | 相互協力 |
| 2 | C | D | |
| 3 | D | C | TFT が 2 ラウンド目をまねる |
| 4 | C | D | TFT が 3 ラウンド目をまねる |
| 5 | D | C | |
| 6 | C | D | 以降は (D, C) と (C, D) が交互に続く |

`grim` が相手のとき．

| ラウンド | 相手 | Alternator | 備考 |
|---:|:-:|:-:|---|
| 1 | C | C | |
| 2 | C | D | |
| 3 | D | C | 2 ラウンド目の D で Grim が切り替わる |
| 4 | D | D | |
| 5 | D | C | |
| 6 | D | D | Grim は以降ずっと D |

自分自身が相手のときは，2 人とも同じ位相で C，D，C，D，… を出すので，相互協力と相互裏切りが交互に続きます．

## コードの読み方

- **登録．** [`arena.rs#L35`](../../simulation/src/arena.rs#L35) が，[`rule_book`](../../simulation/src/arena.rs#L19-L64) の作る戦略集に `Alternator` を `alternator` という名前で登録します．
- **状態の型．** `Policy::Alternator { next }` は [`state.rs#L17-L19`](../../simulation/src/state.rs#L17-L19) で，列挙型 `Policy` ([`state.rs#L13-L24`](../../simulation/src/state.rs#L13-L24)) の中に定義しています．`next` には `C = 0` か `D = 1` が入ります ([`state.rs#L10-L11`](../../simulation/src/state.rs#L10-L11))．
- **戦略本体．** [`strategies.rs#L126-L155`](../../simulation/src/strategies.rs#L126-L155)．
  - `init_state` ([`L133-L135`](../../simulation/src/strategies.rs#L133-L135)) は `Policy::Alternator { next: C }` を返します．
  - `decide` ([`L136-L141`](../../simulation/src/strategies.rs#L136-L141)) は `next` を読んで返すだけです．履歴はまったく読みません．
  - `learn_step` ([`L142-L154`](../../simulation/src/strategies.rs#L142-L154)) が `next` を入れ替えます．

**エンジンが各メソッドを呼ぶとき．** 1 試合は [`play_with`](../../simulation/src/arena.rs#L73-L140) の 1 回の呼び出しです．まず `build_world` ([`arena.rs#L100-L111`](../../simulation/src/arena.rs#L100-L111)) で新しいワールドを作ります．このとき `init_state` がエージェントごとに 1 回呼ばれ，各エージェントの `Policy` が `GameWorld` に保存されます．試合はそのあとの 200 回の `sim.step()` です ([`arena.rs#L124-L138`](../../simulation/src/arena.rs#L124-L138))．各ステップで，エンジンはエージェントの対戦ごとに `decide` を 1 回呼び，全対戦が終わったあとにエージェントごとに `learn_step` を 1 回呼びます．Alternator どうしの試合では，2 人のエージェントは戦略のオブジェクトを共有しますが，`Policy` はそれぞれが別に持つので，それぞれが自分の `next` を入れ替えます．

**入れ替えを `decide` ではなく `learn_step` で行う理由．** `decide` は対戦ごとに 1 回，`learn_step` は全対戦のあとにステップごとに 1 回呼ばれます．`decide` で `next` を入れ替えると，1 ステップに 3 人と対戦するエージェントは 3 回入れ替え，1 ステップの中で C，D，C と出してしまいます．`learn_step` で入れ替えれば，対戦が何回あっても状態は 1 ステップにちょうど 1 つ進みます．このリポジトリの試合には 2 人の組が 1 つしかないので ([`FixedPlayers::pairs(&[(0, 1)])`](../../simulation/src/arena.rs#L114))，1 ステップの対戦は 1 回で，どちらに置いても出る手は同じです．`learn_step` に置くのは，ほかの設定でも正しく動くようにするためです．これに対して [Gradual](gradual.ja.md) は `decide` の中で状態を進めており，1 ステップに 1 回だけ対戦するという設定に頼っています．

**テスト．** [`tests.rs#L176-L184`](../../simulation/src/tests.rs#L176-L184) は `alternator` を `all_c` と対戦させ，200 手すべてが奇数ラウンドで C，偶数ラウンドで D であることを確かめます．

## 参考文献

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Mathieu, P., & Delahaye, J.-P. (2017). New winning strategies for the iterated prisoner's dilemma. *Journal of Artificial Societies and Social Simulation*, 20(4), 12.

[← 戦略の一覧](../strategies.ja.md)
