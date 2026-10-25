[English](generous_tft.md) | **日本語**

# Generous Tit for Tat (`generous_tft`)

Generous Tit for Tat は Tit for Tat と同じように動きますが，相手が D を出した次のラウンドでも，確率 g = 1/3 で C を出します．初回と，相手が C を出した次のラウンドは必ず C です．

## 由来と性質

Generous Tit for Tat (GTFT) は Nowak と Sigmund (1992) によるものです．確率的な反応戦略が混在する集団の進化シミュレーションで，この戦略が広がることを示しました．寛容さは

g = min{1 − (T − R)/(R − S), (R − P)/(T − P)}

で，T = 5，R = 3，P = 1，S = 0 では min{1 − 2/3, 2/4} = 1/3 です．

Axelrod (1984) の言葉で言えば，次の性質を持ちます．

- **上品 (nice)**：自分から先に裏切りません．D を出すのは，相手が D を出した直後だけです．
- **報復的 (retaliatory)．ただし毎回ではない**：相手の D に D を返す確率は 2/3 です．
- **寛容 (forgiving)**：相手が一度 C を出せば C に戻ります．相手の D を自分から許すこともあります．
- **確率的**：相手が D を出した次のラウンドには，毎回乱数を引きます．

強みは，1 回の裏切りで応報的な 2 人が報復の応酬に閉じ込められないことです．GTFT は 1 ラウンドごとに確率 1/3 で C を出し，応酬を断ち切ります．弱みは，その寛容さの代償です．`all_d` が相手だと，およそ 3 ラウンドに 1 回は搾取され続けます．

## 動き

- 1 ラウンド目 (相手の手がまだ無い) は C を出す．
- 相手の前回の手が C なら C を出す．
- 相手の前回の手が D なら，確率 g = 1/3 で C，確率 2/3 で D を出す．

記憶 1 の戦略として書けば，前回の (自分の手, 相手の手) が (C, C)，(C, D)，(D, C)，(D, D) のときの協力確率は 1，1/3，1，1/3 です．自分の前回の手は関係しません．

この戦略は **状態を持ちません**．ラウンドをまたいで何も保持せず，毎ラウンド，対戦の履歴 `Seen { opp, mine }` から手を計算し直します (`opp` は相手の手を古い順に並べたもの，`mine` は自分の前回の手で，この戦略では使いません)．`init_state` は `Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))` を返します．このリポジトリの，履歴だけで手を決める戦略に共通の，中身のない状態です．

以下の手順は，総当たりの出力から取った seed 0 の最初の 16 ラウンドです．確率的な戦略なので，どの手順も 1 つの標本にすぎず，seed が変われば手も変わります．

`all_d` が相手なら，D の後の 15 ラウンドのうち 5 ラウンド (2，6，8，10，11 ラウンド目) で許しています．

| ラウンド | 相手 (`all_d`) | `generous_tft` | 説明 |
|---:|:-:|:-:|---|
| 1 | D | C | 履歴が無いので C |
| 2 | D | C | D の後：許した |
| 3 | D | D | D の後：報復した |
| 4 | D | D | 報復 |
| 5 | D | D | 報復 |
| 6 | D | C | 許した |
| 7 | D | D | 報復 |
| 8 | D | C | 許した |
| 9 | D | D | 報復 |
| 10 | D | C | 許した |
| 11 | D | C | 許した |
| 12〜16 | D | D | 5 回続けて報復 |

`joss` が相手なら，寛容さが応酬を始まる前に止めます．Joss は 10 ラウンド目に確率的な裏切りを 1 回出しますが，GTFT が 11 ラウンド目にそれを許すので，2 人とも C に戻ります．

| ラウンド | 相手 (`joss`) | `generous_tft` | 説明 |
|---:|:-:|:-:|---|
| 1〜9 | C | C | 相互協力 |
| 10 | D | C | C の後なので C |
| 11 | C | C | D の後：許した |
| 12〜16 | C | C | 相互協力 |

[`joss`](joss.ja.md) と `tit_for_tat` の対戦と比べてください．そちらでは同じような裏切りが交互の手を始め，手順の最後まで続きます．

## コードの読み方

関数は [`strategies.rs` の `generous_tft`](../../simulation/src/strategies.rs#L72-L78)，寛容さは定数 [`GENEROSITY`](../../simulation/src/strategies.rs#L14) です．

```rust
pub const GENEROSITY: f64 = 1.0 / 3.0;

pub fn generous_tft(s: &Seen, rng: &mut SimRng) -> usize {
    match s.opp.last() {
        Some(&D) if !rng.gen_bool(GENEROSITY) => D,
        _ => C,
    }
}
```

- `s.opp.last()` は相手の前回の手です．1 ラウンド目は `None` になります．
- 1 つ目の腕に当たるのは，その手が D のときだけです．ガードで `rng.gen_bool(GENEROSITY)` を引き，これは確率 1/3 で真になります．偽 (確率 2/3) なら D を出します．
- それ以外はすべて `_ => C` に落ちます．1 ラウンド目，相手の C，乱数で許された相手の D です．
- 乱数を引くのは相手の D の後だけです．C の後のラウンドでは乱数を使いません．
- `GENEROSITY` は 1/3 に固定してあり，実行時に利得から計算し直してはいません．

呼ばれるまでの流れは次のとおりです．

1. [`arena.rs` の `rule_book`](../../simulation/src/arena.rs#L19-L34) が，配列 `history_rules` に `("generous_tft", generous_tft)` を並べ ([L23](../../simulation/src/arena.rs#L23))，各要素を `HistoryRule { name, choose }` として登録します．
2. 毎ラウンド，[`play_with`](../../simulation/src/arena.rs#L124-L125) が `sim.step()` を呼びます．そのステップの中で，socsim-games のエンジンが 2 人のプレイヤーそれぞれの戦略の `decide` を，乱数生成器を渡して呼び，手を求めます．
3. [`HistoryRule::decide`](../../simulation/src/strategies.rs#L46-L56) が [`seen(ctx)`](../../simulation/src/strategies.rs#L17-L35) を呼びます．`seen` は唯一の相手を取り出し，この対戦での相手の履歴 (`opp`．行動が記録されていない回は除く) と，自分の前回の行動 (`mine`) を読みます．
4. `decide` はその結果と乱数生成器を `choose` に渡します．この戦略では `choose` が `generous_tft` で，その返した手が `decide` の戻り値になります．

## 参考文献

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Nowak, M. A., & Sigmund, K. (1992). Tit for tat in heterogeneous populations. *Nature*, 355, 250–253.

[← 戦略の一覧](../strategies.ja.md)
