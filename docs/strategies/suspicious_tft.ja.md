[English](suspicious_tft.md) | **日本語**

# Suspicious Tit for Tat (`suspicious_tft`)

Suspicious Tit for Tat は，初回に D を出し，以降は相手の前回の手をまねます．Tit for Tat の初手を C から D に変えた戦略です．

## 由来と性質

Suspicious Tit for Tat (STFT) は，Tit for Tat の定番の変種です．Boyd と Lorberbaum (1987) は，繰り返し囚人のジレンマで進化的に安定な純粋戦略は存在しないという議論に，Tit for Two Tats とともにこの戦略を使っています．

Axelrod (1984) の言葉で言えば，次の性質を持ちます．

- **非先制型ではない (not nice)**：1 ラウンド目に自分から裏切ります．
- **報復的 (retaliatory)**：相手の D には次のラウンドで D を返します．
- **寛容 (forgiving)**：相手の C には，それまでの経緯にかかわらず次のラウンドで C を返します．
- **明快 (clear)**：1 ラウンド目より後は Tit for Tat そのものです．
- **決定的**：乱数を引きません．

初手の D は，無条件に協力する相手からは T = 5 を得ますが，相手が応報的だと損になります．Tit for Tat が相手なら初手の D がそのまま返され，2 人は対戦の最後まで D と C を交互に出し合います．自分自身が相手なら，2 人とも毎ラウンド D を出します．

## 動き

- 相手の手がまだ無い 1 ラウンド目は D を出す．
- それ以外は相手の前回の手を出す．

この戦略は **状態を持ちません**．ラウンドをまたいで何も保持せず，毎ラウンド，対戦の履歴 `Seen { opp, mine }` から手を計算し直します (`opp` は相手の手を古い順に並べたもの，`mine` は自分の前回の手で，この戦略では使いません)．`init_state` は `Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))` を返します．このリポジトリの，履歴だけで手を決める戦略に共通の，中身のない状態です．

以下の手順は，総当たりの出力から取った seed 0 の最初の 16 ラウンドです．決定的な戦略なので，どの seed でも同じ手になります．

`tit_for_tat` が相手なら，初手の D が行き来し続けます．

| ラウンド | 相手 (`tit_for_tat`) | `suspicious_tft` | 説明 |
|---:|:-:|:-:|---|
| 1 | C | D | 履歴が無いので D |
| 2 | D | C | 相手の 1 ラウンド目の C をまねる |
| 3 | C | D | 相手の 2 ラウンド目の D をまねる |
| 4 | D | C | C をまねる |
| 5〜16 | C，D，… | D，C，… | 交互の手が続く |

`all_c` が相手なら，1 回だけ D を出し，あとは協力します．

| ラウンド | 相手 (`all_c`) | `suspicious_tft` | 説明 |
|---:|:-:|:-:|---|
| 1 | C | D | 履歴が無いので D |
| 2〜16 | C | C | C をまねる |

自分自身 (`suspicious_tft` 対 `suspicious_tft`) が相手なら，2 人とも 16 ラウンドすべてで D を出します．

## コードの読み方

関数は [`strategies.rs` の `suspicious_tft`](../../simulation/src/strategies.rs#L67-L70) です．

```rust
pub fn suspicious_tft(s: &Seen, _: &mut SimRng) -> usize {
    s.opp.last().copied().unwrap_or(D)
}
```

- `s.opp.last()` は相手の前回の手です．1 ラウンド目は `None` になります．
- `.copied()` で `Option<&usize>` を `Option<usize>` に変えます．
- `.unwrap_or(D)` は，その手があればそれを，無ければ D を返します．Tit for Tat との違いはこの引数だけで，Tit for Tat ならここが C です．
- 乱数生成器は使わず (`_`)，`s.mine` も読みません．

呼ばれるまでの流れは次のとおりです．

1. [`arena.rs` の `rule_book`](../../simulation/src/arena.rs#L19-L34) が，配列 `history_rules` に `("suspicious_tft", suspicious_tft)` を並べ ([L22](../../simulation/src/arena.rs#L22))，各要素を `HistoryRule { name, choose }` として登録します．
2. 毎ラウンド，[`play_with`](../../simulation/src/arena.rs#L124-L125) が `sim.step()` を呼びます．そのステップの中で，socsim-games のエンジンが 2 人のプレイヤーそれぞれの戦略の `decide` を呼んで手を求めます．
3. [`HistoryRule::decide`](../../simulation/src/strategies.rs#L46-L56) が [`seen(ctx)`](../../simulation/src/strategies.rs#L17-L35) を呼びます．`seen` は唯一の相手を取り出し，この対戦での相手の履歴 (`opp`．行動が記録されていない回は除く) と，自分の前回の行動 (`mine`) を読みます．
4. `decide` はその結果を `choose` に渡します．この戦略では `choose` が `suspicious_tft` で，その返した手が `decide` の戻り値になります．

## 参考文献

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Boyd, R., & Lorberbaum, J. P. (1987). No pure strategy is evolutionarily stable in the repeated Prisoner's Dilemma game. *Nature*, 327, 58–59.

[← 戦略の一覧](../strategies.ja.md)
