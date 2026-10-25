[English](tit_for_two_tats.md) | **日本語**

# Tit for Two Tats (`tit_for_two_tats`)

Tit for Two Tats は，相手が直前の 2 ラウンド続けて D を出したときだけ D を出し，それ以外は C を出します．1 回だけの裏切りは見逃す，Tit for Tat より寛容な戦略です．

## 由来と性質

Tit for Two Tats (TF2T) は，Axelrod が論じた Tit for Tat の変種の 1 つです (Axelrod, 1984)．Axelrod の第 2 回の大会には，John Maynard Smith が応募しています (Axelrod, 1980)．

Axelrod の言葉で言えば，次の性質を持ちます．

- **非先制型 (nice)**：自分から先に裏切りません．D を出すには相手の D が 2 回必要なので，先に裏切ることがありえません．
- **報復的 (retaliatory)．ただし遅い**：応じるのは 2 回続いた D だけです．単発の D には報復しません．
- **寛容 (forgiving)**：相手が一度 C を出せば，直前の 2 手が D，D ではなくなり，C に戻ります．
- **決定的**：乱数を引きません．

強みは，誤りや探りによる 1 回の D が，Tit for Tat どうしのような報復の応酬を起こさないことです．弱みも同じ寛容さにあります．2 回続けて D を出さない相手 (たとえば C と D を交互に出す相手) は，1 ラウンドおきに D を出しても報復されません．

## 動き

戦略が見るのは相手の手だけです．

- 相手の直前の 2 手が D，D なら D を出す．
- それ以外は C を出す．相手の手がまだ 2 手に満たない最初の 2 ラウンドもここに含まれます．

この戦略は **状態を持ちません**．ラウンドをまたいで何も保持せず，毎ラウンド，対戦の履歴 `Seen { opp, mine }` から手を計算し直します (`opp` は相手の手を古い順に並べたもの，`mine` は自分の前回の手で，この戦略では使いません)．`init_state` は `Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))` を返します．このリポジトリの，履歴だけで手を決める戦略に共通の，中身のない状態です．

以下の手順は，総当たりの出力から取った seed 0 の最初の 16 ラウンドです．決定的な戦略なので，どの seed でも同じ手になります．

`all_d` が相手なら，2 回 C を出したあと，ずっと D を出します．

| ラウンド | 相手 (`all_d`) | `tit_for_two_tats` | 説明 |
|---:|:-:|:-:|---|
| 1 | D | C | 履歴が無いので C |
| 2 | D | C | 相手の手はまだ 1 手だけなので C |
| 3 | D | D | 相手の直前の 2 手が D，D |
| 4〜16 | D | D | D，D が続く |

`prober` が相手なら，付け込める相手だと見抜かれます．Prober は D，C，C と始め，TF2T が 2 回目と 3 回目に C を返したので，4 回目からずっと D を出します．

| ラウンド | 相手 (`prober`) | `tit_for_two_tats` | 説明 |
|---:|:-:|:-:|---|
| 1 | D | C | 履歴が無いので C |
| 2 | C | C | 直前の手は D の 1 手だけ |
| 3 | C | C | 直前の 2 手は D，C |
| 4 | D | C | 直前の 2 手は C，C |
| 5 | D | C | 直前の 2 手は C，D |
| 6 | D | D | 直前の 2 手が D，D |
| 7〜16 | D | D | D，D が続く |

`alternator` (C，D，C，D，…) が相手なら，16 ラウンドすべてで C を出します．相手が 2 回続けて D を出すことが無いからです．

## コードの読み方

関数は [`strategies.rs` の `tit_for_two_tats`](../../simulation/src/strategies.rs#L58-L65) です．

```rust
pub fn tit_for_two_tats(s: &Seen, _: &mut SimRng) -> usize {
    if s.opp.ends_with(&[D, D]) {
        D
    } else {
        C
    }
}
```

- `s.opp` は，この対戦での相手の手を古い順に並べたものです．`ends_with(&[D, D])` が真になるのは，手が 2 手以上あり，最後の 2 手がどちらも D のときだけです．2 手に満たなければ偽になるので，最初の 2 ラウンドは C になります．
- 乱数生成器は使いません (`_`)．
- `s.mine` は読みません．

呼ばれるまでの流れは次のとおりです．

1. [`arena.rs` の `rule_book`](../../simulation/src/arena.rs#L19-L34) が，配列 `history_rules` に `("tit_for_two_tats", tit_for_two_tats)` を並べ ([L21](../../simulation/src/arena.rs#L21))，各要素を `HistoryRule { name, choose }` として登録します．
2. 毎ラウンド，[`play_with`](../../simulation/src/arena.rs#L124-L125) が `sim.step()` を呼びます．そのステップの中で，socsim-games のエンジンが 2 人のプレイヤーそれぞれの戦略の `decide` を呼んで手を求めます．
3. [`HistoryRule::decide`](../../simulation/src/strategies.rs#L46-L56) が [`seen(ctx)`](../../simulation/src/strategies.rs#L17-L35) を呼びます．`seen` は唯一の相手を取り出し，この対戦での相手の履歴 (`opp`．行動が記録されていない回は除く) と，自分の前回の行動 (`mine`) を読みます．
4. `decide` はその結果を `choose` に渡します．この戦略では `choose` が `tit_for_two_tats` で，その返した手が `decide` の戻り値になります．

## 参考文献

- Axelrod, R. (1980). More effective choice in the prisoner's dilemma. *Journal of Conflict Resolution*, 24(3), 379–403.
- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.

[← 戦略の一覧](../strategies.ja.md)
