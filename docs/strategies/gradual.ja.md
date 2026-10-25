[English](gradual.md) | **日本語**

# Gradual (`gradual`)

Gradual は，相手が D を出すまで C を出し続けます．相手の D に応えるときは，それまでに相手が出した D の総数と同じ回数だけ D を出し，続けて C を 2 回出して場を鎮め，また相手の前回の手を見る状態に戻ります．報復は回を追うごとに長くなりますが，報復のあとには必ず協力を申し出ます．

## 由来と性質

Gradual は Beaufils，Delahaye と Mathieu (1997) が Tit for Tat (TFT) を改良する戦略として提案しました．TFT と同じく D に応えますが，応え方は相手のそれまでの振る舞いに応じて強くなり，最後に必ず C を 2 回出します．Mathieu と Delahaye (2017) も，古典的な戦略の 1 つとしてこれを扱っています．

Axelrod (1984) が成功した戦略を説明するのに使った言葉で言えば，次のとおりです．

- **非先制型 (nice)**：自分から先に D を出しません．相手が最初の D を出すまでは C だけを出します．
- **報復的 (retaliatory, provocable)**：D には D で応え，しかも回を追うごとに強く応えます．
- **寛容 (forgiving)**：報復のあとには必ず C を 2 回出して通常モードに戻るので，`grim` のように 1 回の D をいつまでも罰し続けることはありません．
- **TFT ほど明快 (clear) ではない**：報復の長さは履歴全体で決まり，報復とそれに続く鎮静のあいだは相手の手にまったく反応しません．

決定的な戦略です (乱数を使いません)．

強みは，鎮静の C 2 回が D の応酬を断ち切れることです．TFT と Suspicious TFT は互いの D をいつまでもまね合いますが，Gradual は 1 回報復し，C を 2 回出して，双方が相互協力に戻ります (下の `suspicious_tft` との例)．D を出し続ける相手には報復が伸びていき (`all_d` が相手なら 1，4，10，22，…)，ほとんどの回で D を出すことになります．

弱みも同じ作りから生じます．報復は短くならないので，`joss` のようにときどき D を出す相手には，対戦が進むほど報復が長くなります．また報復と鎮静のあいだは相手を見ないので，その間に出された D にはその場では応えません (後のために数えるだけです)．

## 動き

**状態．** `Policy::Gradual { remaining_defections, remaining_calming }` で，いまの報復で残っている D の回数と，残っている鎮静の C の回数です．どちらも 0 から始まります．この状態がモードを決めます．

| モード | 条件 | 出す手 | 相手を見るか | 抜けるとき |
|---|---|---|---|---|
| 通常 | 2 つの数がともに 0 | C．相手の前回の手が D なら D | 見る．相手の前回の手だけ | D を見たとき．このとき出す D が報復の 1 回目になる |
| 報復 | `remaining_defections > 0` | D | 見ない | 報復の n 回の D を出し終えたとき |
| 鎮静 | `remaining_defections == 0` かつ `remaining_calming > 0` | C | 見ない | 2 回目の C を出したとき |

**通常モードでの手の決め方．** 1 回目は履歴が無いので C を出します．以降は，相手の前回の手が C なら C を出します．D なら，相手の履歴全体にある D の数 n を数え，いま D を出して，`remaining_defections = n − 1`，`remaining_calming = 2` とします．つまり報復は D の n 回の連続 (通常モードで出す 1 回と，残りの n − 1 回) で，そのあとに C が 2 回続きます．

ここから次の 3 点が導かれます．

- **報復中・鎮静中の D には，それ自体には応えません．** 通常モードは相手の前回の手しか見ないので，その期間の D のうち見えるのは，鎮静の最後の回に出された D だけです．その次の回は通常モードで決まるからです．
- **D はすべて数えます．** n は毎回履歴全体から数え直すので，報復中や鎮静中に出された D も次の報復を長くします．
- **始めた報復は途中で変えません．** 報復の長さは始めた時点で決まります．

文献での Gradual の説明は，報復中に相手が出した D の扱いまでは決めていません．上の 3 点は，この実装が採った解釈です．

**例．** 総当たりのプログラムが記録した，seed 0 での最初の 16 ラウンドです．

`all_d` が相手のとき．報復が 1 回，4 回，10 回と伸びます．

| ラウンド | 相手 | Gradual | モード・備考 |
|---:|:-:|:-:|---|
| 1 | D | C | 通常．履歴なし |
| 2 | D | D | 通常で D を見る．n = 1：報復 1/1 |
| 3 | D | C | 鎮静 1/2 |
| 4 | D | C | 鎮静 2/2 |
| 5 | D | D | 通常で D を見る．n = 4：報復 1/4 |
| 6 | D | D | 報復 2/4 |
| 7 | D | D | 報復 3/4 |
| 8 | D | D | 報復 4/4 |
| 9 | D | C | 鎮静 1/2 |
| 10 | D | C | 鎮静 2/2 |
| 11 | D | D | 通常で D を見る．n = 10：報復 1/10 |
| 12–16 | D | D | 報復 2/10〜6/10 |

`suspicious_tft` が相手のとき．報復は 1 回で，鎮静が応酬を断ち切ります．

| ラウンド | 相手 | Gradual | モード・備考 |
|---:|:-:|:-:|---|
| 1 | D | C | 通常．履歴なし |
| 2 | C | D | 通常で D を見る．n = 1：報復 1/1 |
| 3 | D | C | 鎮静 1/2．相手は 2 ラウンド目をまねた．この D には応えない |
| 4 | C | C | 鎮静 2/2 |
| 5 | C | C | 通常で C を見る |
| 6–16 | C | C | 通常 |

3 ラウンド目の相手の D は数えられています．相手がもう一度 D を出せば，次の報復は n = 3 になります．

`joss` が相手のとき．Joss は C を出す場面で確率的に D を出します．

| ラウンド | 相手 | Gradual | モード・備考 |
|---:|:-:|:-:|---|
| 1–9 | C | C | 通常 |
| 10 | D | C | 通常で C を見る．Joss の確率的な D |
| 11 | C | D | 通常で D を見る．n = 1：報復 1/1 |
| 12 | D | C | 鎮静 1/2．Joss は 11 ラウンド目をまねた．応えない |
| 13 | C | C | 鎮静 2/2 |
| 14 | C | C | 通常で C を見る |
| 15–16 | C | C | 通常 |

`alternator` が相手のとき．報復中・鎮静中の相手の D には応えませんが，次の報復を長くします．

| ラウンド | 相手 | Gradual | モード・備考 |
|---:|:-:|:-:|---|
| 1 | C | C | 通常．履歴なし |
| 2 | D | C | 通常で C を見る |
| 3 | C | D | 通常で D を見る．n = 1：報復 1/1 |
| 4 | D | C | 鎮静 1/2．この D には応えない |
| 5 | C | C | 鎮静 2/2 |
| 6 | D | C | 通常で C を見る |
| 7 | C | D | 通常で D を見る．n = 3 (2，4，6 ラウンド目)：報復 1/3 |
| 8 | D | D | 報復 2/3 |
| 9 | C | D | 報復 3/3 |
| 10 | D | C | 鎮静 1/2 |
| 11 | C | C | 鎮静 2/2 |
| 12 | D | C | 通常で C を見る |
| 13 | C | D | 通常で D を見る．n = 6：報復 1/6 |
| 14–16 | D，C，D | D | 報復 2/6〜4/6 |

## コードの読み方

- **登録．** [`arena.rs#L36`](../../simulation/src/arena.rs#L36) が，[`rule_book`](../../simulation/src/arena.rs#L19-L64) の作る戦略集に `Gradual` を `gradual` という名前で登録します．
- **状態の型．** `Policy::Gradual` は [`state.rs#L20-L23`](../../simulation/src/state.rs#L20-L23) で，列挙型 `Policy` ([`state.rs#L13-L24`](../../simulation/src/state.rs#L13-L24)) の中に定義しています．手は定数 `C = 0` と `D = 1` です ([`state.rs#L10-L11`](../../simulation/src/state.rs#L10-L11))．
- **戦略本体．** [`strategies.rs#L157-L202`](../../simulation/src/strategies.rs#L157-L202)．
  - `init_state` ([`L164-L169`](../../simulation/src/strategies.rs#L164-L169)) は 2 つの数をともに 0 にして返します．これが通常モードです．
  - `decide` ([`L170-L201`](../../simulation/src/strategies.rs#L170-L201)) はモードを順に調べます．報復 ([`L183-L186`](../../simulation/src/strategies.rs#L183-L186)) では `remaining_defections` を 1 減らし，履歴を読まずに D を返します．鎮静 ([`L187-L190`](../../simulation/src/strategies.rs#L187-L190)) では `remaining_calming` を 1 減らし，やはり履歴を読まずに C を返します．通常モード ([`L192-L200`](../../simulation/src/strategies.rs#L192-L200)) では `seen` ([`L23-L35`](../../simulation/src/strategies.rs#L23-L35)) で相手の履歴を読み，前回が D なら相手の D をすべて数えて報復を始めます．
  - `learn_step` は定義していません．何もしない既定のものが使われます．

**エンジンが各メソッドを呼ぶとき．** 1 試合は [`play_with`](../../simulation/src/arena.rs#L73-L140) の 1 回の呼び出しです．まず `build_world` ([`arena.rs#L100-L111`](../../simulation/src/arena.rs#L100-L111)) で新しいワールドを作ります．このとき `init_state` がエージェントごとに 1 回呼ばれ，各エージェントの `Policy` が `GameWorld` に保存されます．試合はそのあとの 200 回の `sim.step()` です ([`arena.rs#L124-L138`](../../simulation/src/arena.rs#L124-L138))．各ステップで，エンジンはエージェントの対戦ごとに `decide` を 1 回呼び，全対戦が終わったあとにエージェントごとに `learn_step` を 1 回呼びます．Gradual どうしの試合では，2 人のエージェントは戦略のオブジェクトを共有しますが，`Policy` はそれぞれが別に持ちます．

**`decide` の中で状態を進めてよい理由．** Gradual は数を `decide` の中で進めるので，`decide` の 1 回が 1 ラウンドに対応していなければなりません．この試合には 2 人の組が 1 つしかなく ([`FixedPlayers::pairs(&[(0, 1)])`](../../simulation/src/arena.rs#L114))，各エージェントは 1 ステップにちょうど 1 回対戦するので，これが成り立ちます．1 ステップに複数の相手と対戦する設定では，`decide` が 1 ステップに何度も呼ばれて 1 ラウンドに数が何度も減るので，[Alternator](alternator.ja.md) のように更新を `learn_step` に移す必要があります．

**テスト．** [`tests.rs#L186-L200`](../../simulation/src/tests.rs#L186-L200) は `gradual` を `all_d` と対戦させ，最初の 22 手が C，D，C，C，D × 4，C，C，D × 10，C，C であることを確かめます．

## 参考文献

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Beaufils, B., Delahaye, J.-P., & Mathieu, P. (1997). Our meeting with gradual: A good strategy for the iterated prisoner's dilemma. In C. Langton & K. Shimohara (Eds.), *Artificial Life V: Proceedings of the Fifth International Workshop on the Synthesis and Simulation of Living Systems*. MIT Press.
- Mathieu, P., & Delahaye, J.-P. (2017). New winning strategies for the iterated prisoner's dilemma. *Journal of Artificial Societies and Social Simulation*, 20(4), 12.

[← 戦略の一覧](../strategies.ja.md)
