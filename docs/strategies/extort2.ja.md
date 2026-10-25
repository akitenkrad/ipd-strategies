[English](extort2.md) | **日本語**

# Extort-2 (`extort2`)

Extort-2 は初回に C を出します．以降は前回の手の組に応じた確率で協力します．(自分の手, 相手の手) が (C, C) なら 8/9，(C, D) なら 1/2，(D, C) なら 1/3，(D, D) なら 0 です．長い対戦では，自分の利得の P からの上積みが，相手の上積みのちょうど 2 倍になるよう強制します．

## 由来と性質

Extort-2 は Press と Dyson (2012) のゼロ行列式 (ZD) 戦略のうち，χ = 2，φ = 1/18 の搾取的なものです．Extort-2 という名前とこの 4 つの確率は，Stewart と Plotkin (2012) が使ったもので，彼らはこの戦略を古典的な戦略と並べて大会に出しています．

プレイヤー X の記憶 1 の戦略は，X から見た前回の結果 (C, C)，(C, D)，(D, C)，(D, D) の後の協力確率を並べたベクトル p = (p1, p2, p3, p4) です．Press と Dyson は，

p̃ = (p1 − 1, p2 − 1, p3, p4) = φ [(S_X − P) − χ (S_Y − P)]

が成り立つなら，Y がどう振る舞っても，長い目で見た平均利得 s_X，s_Y のあいだに s_X − P = χ (s_Y − P) が成り立つことを示しました．ここで S_X = (R, S, T, P)，S_Y = (R, T, S, P) は，その 4 つの結果での 2 人の利得です．R = 3，S = 0，T = 5，P = 1，χ = 2，φ = 1/18 とすると次のようになります．

| 結果 (X, Y) | S_X − P | S_Y − P | p̃ = φ [(S_X − P) − 2 (S_Y − P)] | p |
|---|---:|---:|---:|---:|
| (C, C) | 2 | 2 | −1/9 | 8/9 |
| (C, D) | −1 | 4 | −1/2 | 1/2 |
| (D, C) | 4 | −1 | 1/3 | 1/3 |
| (D, D) | 0 | 0 | 0 | 0 |

したがって，この戦略は s_X − P = 2 (s_Y − P) を成り立たせます．

Axelrod (1984) の言葉で言えば，次の性質を持ちます．

- **非先制型ではない (not nice)**：相互協力の後でも，確率 1/9 で裏切ります．
- **報復的 (retaliatory)**：搾取された後 ((C, D)) は確率 1/2 で裏切り，相互裏切りの後は必ず裏切ります．
- **寛容さは限られる**：自分が D で相手が C だった後に C に戻る確率は 1/3 だけです．また p4 = 0 なので，相互裏切りに入ると抜け出せません．
- **確率的**：2 ラウンド目以降は毎ラウンド乱数を引きます．

強みは 1 対 1 の対戦にあります．相手の利得が P 以上なら，s_X − P = 2 (s_Y − P) から，Extort-2 は相手以上の利得を得ます．相手が自分の利得を上げれば，Extort-2 の利得はその 2 倍上がります．弱みは，自分と同じような相手からも搾取しようとすることです．Extort-2 どうしでは互いに相手へこの関係を強制し，それが両立するのは 2 人とも P を得るときだけです．相互裏切りに一度入ると，そこから出ることはありません．

## 動き

- 1 ラウンド目は C を出す．
- それ以外は，前回の (自分の手, 相手の手) を見て，(C, C)，(C, D)，(D, C)，(D, D) ならそれぞれ確率 8/9，1/2，1/3，0 で協力する．

初手の C はこの実装の選択です．ZD の構成が決めるのは 4 つの条件付き確率だけで，初手は長い目で見た関係に影響しません．

このリポジトリで履歴から手を決める戦略のうち，自分の前回の手を読むのはこの戦略だけです．それでも **状態は持ちません**．ラウンドをまたいで何も保持せず，毎ラウンド，対戦の履歴 `Seen { opp, mine }` から手を計算し直します (`opp` は相手の手を古い順に並べたもの，`mine` は自分の前回の手)．`init_state` は `Policy::Builtin(BuiltinPolicyState::Stateless(Stateless))` を返します．このリポジトリの，履歴だけで手を決める戦略に共通の，中身のない状態です．

以下の手順は，総当たりの出力から取った seed 0 の最初の 16 ラウンドです．確率的な戦略なので，どの手順も 1 つの標本にすぎず，seed が変われば手も変わります．

`all_c` が相手なら，ときどき裏切り，一度も罰せられません．

| ラウンド | 相手 (`all_c`) | `extort2` | 説明 |
|---:|:-:|:-:|---|
| 1 | C | C | 1 ラウンド目なので C |
| 2〜4 | C | C | (C, C) の後：確率 8/9 で C |
| 5 | C | D | (C, C) の後：D を引いた |
| 6 | C | C | (D, C) の後：C を引いた (確率 1/3) |
| 7 | C | D | (C, C) の後：D を引いた |
| 8 | C | C | (D, C) の後：C を引いた |
| 9〜16 | C | C | (C, C) の後：C |

自分自身が相手なら，2 人は相互裏切りへ流れ込み，そこにとどまります．「プレイヤー 0」と「プレイヤー 1」は 2 つの複製で，説明にはそれぞれの前回の (自分の手, 相手の手) を書きます．

| ラウンド | プレイヤー 0 | プレイヤー 1 | 説明 |
|---:|:-:|:-:|---|
| 1 | C | C | 1 ラウンド目 |
| 2 | C | C | 2 人とも (C, C) の後 |
| 3 | C | D | 2 人とも (C, C) の後．プレイヤー 1 が D を引いた |
| 4 | C | D | プレイヤー 0 は (C, D) の後，プレイヤー 1 は (D, C) の後 |
| 5 | C | D | 同じ組 |
| 6 | C | C | 同じ組．プレイヤー 1 が C を引いた |
| 7 | C | C | 2 人とも (C, C) の後 |
| 8 | C | C | 2 人とも (C, C) の後 |
| 9 | C | D | 2 人とも (C, C) の後．プレイヤー 1 が D を引いた |
| 10 | D | D | (C, D) の後のプレイヤー 0 も，(D, C) の後のプレイヤー 1 も D を引いた |
| 11〜16 | D | D | 2 人とも (D, D) の後：確率 0 |

`all_d` が相手なら，C，C と出したあと，3 ラウンド目からずっと D を出します．(C, D) の後に 2 ラウンド目は C を，3 ラウンド目は D を引き，(D, D) の後は二度と協力しません．

## コードの読み方

関数は [`strategies.rs` の `extort2`](../../simulation/src/strategies.rs#L90-L101) です．

```rust
pub fn extort2(s: &Seen, rng: &mut SimRng) -> usize {
    let cooperate = match (s.mine, s.opp.last().copied()) {
        (None, _) => return C,
        (Some(C), Some(C)) => 8.0 / 9.0,
        (Some(C), Some(D)) => 1.0 / 2.0,
        (Some(D), Some(C)) => 1.0 / 3.0,
        (Some(D), Some(D)) => 0.0,
        _ => 0.0,
    };
    usize::from(!rng.gen_bool(cooperate))
}
```

- `match` は (自分の前回の手, 相手の前回の手) の組を見ます．`s.mine` は [`seen`](../../simulation/src/strategies.rs#L33) が自分の履歴から取り出したものです．
- `(None, _) => return C`：自分の前回の手が無いのは 1 ラウンド目です．乱数を引かずに，すぐ C を返します．
- 続く 4 つの腕が，(C, C)，(C, D)，(D, C)，(D, D) のときの協力確率です．`C` と `D` は定数 0 と 1 で，ここではパターンとして使っています．
- `_ => 0.0` は残りの場合です．自分の前回の手はあるのに相手の手が無い場合で，2 人が毎ラウンド手を出す対戦では起こりません．確率 0，つまり D にしています．
- `rng.gen_bool(cooperate)` は確率 `cooperate` で真になります．`!` で「協力する」を「裏切る」に反転し，`usize::from` で `false` を 0 = C に，`true` を 1 = D に変えます．

呼ばれるまでの流れは次のとおりです．

1. [`arena.rs` の `rule_book`](../../simulation/src/arena.rs#L19-L34) が，配列 `history_rules` に `("extort2", extort2)` を並べ ([L25](../../simulation/src/arena.rs#L25))，各要素を `HistoryRule { name, choose }` として登録します．
2. 毎ラウンド，[`play_with`](../../simulation/src/arena.rs#L124-L125) が `sim.step()` を呼びます．そのステップの中で，socsim-games のエンジンが 2 人のプレイヤーそれぞれの戦略の `decide` を，乱数生成器を渡して呼び，手を求めます．
3. [`HistoryRule::decide`](../../simulation/src/strategies.rs#L46-L56) が [`seen(ctx)`](../../simulation/src/strategies.rs#L17-L35) を呼びます．`seen` は唯一の相手を取り出し，この対戦での相手の履歴 (`opp`．行動が記録されていない回は除く) と，自分の前回の行動 (`mine`) を読みます．
4. `decide` はその結果と乱数生成器を `choose` に渡します．この戦略では `choose` が `extort2` で，その返した手が `decide` の戻り値になります．

## 参考文献

- Axelrod, R. (1984). *The evolution of cooperation*. Basic Books.
- Press, W. H., & Dyson, F. J. (2012). Iterated Prisoner's Dilemma contains strategies that dominate any evolutionary opponent. *Proceedings of the National Academy of Sciences*, 109(26), 10409–10413.
- Stewart, A. J., & Plotkin, J. B. (2012). Extortion and cooperation in the Prisoner's Dilemma. *Proceedings of the National Academy of Sciences*, 109(26), 10134–10135.

[← 戦略の一覧](../strategies.ja.md)
