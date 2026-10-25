[English](joss.md) | **日本語**

# Joss (`joss`)

Joss は Tit for Tat と同じように動きますが，Tit for Tat なら協力する場面で，確率 0.1 で裏切ります．初回は C で，相手の D には必ず D を返します．

## 由来と性質

Joss の名前は，Axelrod の第 1 回のコンピュータ大会にこの戦略で応募した Johann Joss に由来します (Axelrod, 1980)．Axelrod (1984) はこれを，ときどきの裏切りでうまく得をしようとする Tit for Tat の変種として紹介しています．Tit for Tat と同じく相手が裏切った直後には必ず裏切りますが，相手が協力した後でも 10% の割合で裏切ります．

Axelrod (1984) の言葉で言えば，次の性質を持ちます．

- **非先制型ではない (not nice)**：自分から先に裏切ることがあります．確率的な裏切りは，相手が協力しかしていなくても，2 ラウンド目以降のどのラウンドにも起こりえます．
- **報復的 (retaliatory)**：相手の D には必ず D を返します．
- **寛容 (forgiving)**：相手の C には，10 回に 9 回は C を返します．
- **確率的**：相手が C を出した次のラウンドには，毎回乱数を引きます．

この小さな変更が，応報的な相手に対しては大きな損になります．Axelrod (1984) は，Tit for Tat に対する 1 回の確率的な裏切りが，2 人が D と C を交互に出し合う応酬を引き起こす様子を示しています．戦略の定義から，応酬の途中で Joss が相手の C に応じるラウンドに 2 回目の確率的な裏切りが起きると，そのラウンド以降は 2 人とも裏切り続けることになります．下の手順は，そうした応酬の始まりです．

## 動き

- 1 ラウンド目 (相手の手がまだ無い) は C を出す．
- 相手の前回の手が D なら D を出す．
- 相手の前回の手が C なら，確率 0.1 で D，確率 0.9 で C を出す．

記憶 1 の戦略として書けば，前回の (自分の手, 相手の手) が (C, C)，(C, D)，(D, C)，(D, D) のときの協力確率は 0.9，0，0.9，0 です．自分の前回の手は関係せず，1 ラウンド目は必ず C です．コードは上の説明どおりで，それ以上のことはしていません．

この戦略は **状態を持ちません**．ラウンドをまたいで何も保持せず，毎ラウンド，対戦の履歴 `Seen { opp, mine }` から手を計算し直します (`opp` は相手の手を古い順に並べたもの，`mine` は自分の前回の手で，この戦略では使いません)．`init_state` は `Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))` を返します．このリポジトリの，履歴だけで手を決める戦略に共通の，中身のない状態です．

以下の手順は，総当たりの出力から取った seed 0 の最初の 16 ラウンドです．確率的な戦略なので，どの手順も 1 つの標本にすぎず，seed が変われば手も変わります．

`tit_for_tat` が相手なら，1 回の確率的な裏切りが応酬を始めます．

| ラウンド | 相手 (`tit_for_tat`) | `joss` | 説明 |
|---:|:-:|:-:|---|
| 1 | C | C | 履歴が無いので C |
| 2〜9 | C | C | C の後：乱数で C |
| 10 | C | D | C の後：確率的な裏切り |
| 11 | D | C | C の後なので C．Tit for Tat が 10 ラウンド目に応じる |
| 12 | C | D | D の後なので D |
| 13 | D | C | C の後なので C |
| 14 | C | D | D の後なので D |
| 15 | D | C | C の後なので C |
| 16 | C | D | D の後なので D |

報復しない `all_c` が相手なら，確率的な裏切りはそのラウンドで T = 5 を得るだけです．Joss は 10 ラウンド目に D を出し，それ以外のラウンドはすべて C を出します．

## コードの読み方

関数は [`strategies.rs` の `joss`](../../simulation/src/strategies.rs#L80-L88)，裏切る確率は定数 [`JOSS_DEFECT`](../../simulation/src/strategies.rs#L15) です．

```rust
pub const JOSS_DEFECT: f64 = 0.1;

pub fn joss(s: &Seen, rng: &mut SimRng) -> usize {
    match s.opp.last() {
        None => C,
        Some(&D) => D,
        Some(_) if rng.gen_bool(JOSS_DEFECT) => D,
        Some(_) => C,
    }
}
```

- `s.opp.last()` は相手の前回の手です．1 ラウンド目は `None` になります．
- `None => C`：1 ラウンド目は C です．乱数は引きません．
- `Some(&D) => D`：相手の D には必ず D を返します．乱数は引きません．
- `Some(_) if rng.gen_bool(JOSS_DEFECT) => D`：残るのは相手が C を出した場合です．ガードで確率 0.1 で真になる値を引き，真なら裏切ります．
- `Some(_) => C`：そうでなければ (確率 0.9) 協力します．
- 腕は上から順に試されるので，乱数を引くのは相手の C の後だけです．

呼ばれるまでの流れは次のとおりです．

1. [`arena.rs` の `rule_book`](../../simulation/src/arena.rs#L19-L34) が，配列 `history_rules` に `("joss", joss)` を並べ ([L24](../../simulation/src/arena.rs#L24))，各要素を `HistoryRule { name, choose }` として登録します．
2. 毎ラウンド，[`play_with`](../../simulation/src/arena.rs#L124-L125) が `sim.step()` を呼びます．そのステップの中で，socsim-games のエンジンが 2 人のプレイヤーそれぞれの戦略の `decide` を，乱数生成器を渡して呼び，手を求めます．
3. [`HistoryRule::decide`](../../simulation/src/strategies.rs#L46-L56) が [`seen(ctx)`](../../simulation/src/strategies.rs#L17-L35) を呼びます．`seen` は唯一の相手を取り出し，この対戦での相手の履歴 (`opp`．行動が記録されていない回は除く) と，自分の前回の行動 (`mine`) を読みます．
4. `decide` はその結果と乱数生成器を `choose` に渡します．この戦略では `choose` が `joss` で，その返した手が `decide` の戻り値になります．

## 参考文献

- Axelrod, R. (1980). Effective choice in the prisoner's dilemma. *Journal of Conflict Resolution*, 24(1), 3–25.
- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.

[← 戦略の一覧](../strategies.ja.md)
