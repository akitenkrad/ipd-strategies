[English](architecture.md) | **日本語**

# 構成

## ソースの構成

```
simulation/
├── Cargo.toml
├── src/
│   ├── main.rs            入口．総当たりと `checks` を振り分ける
│   ├── state.rs           行動の定数，戦略の状態 `Policy`，実験状態 `Exp`
│   ├── strategies.rs      自作の戦略．前半は状態を持たない 7 つ，
│   │                      後半は状態を持つ Alternator と Gradual
│   ├── arena.rs           戦略集と，2 つの戦略の 1 回の対戦
│   ├── checks.rs          TFT と自作の戦略の 1 対 1 の対戦
│   ├── tournament.rs      総当たり，集計，CSV の出力
│   └── tests.rs           自作の戦略の単体テスト
└── tests/
    ├── cli.rs             2 つの CLI モードの通しのテスト
    └── fixtures/          `checks` の出力と `ranking.csv` の期待値
```

`tools/` は Python のパッケージです．`plot_tournament.py` が `out/` から記事の図を描きます ([使い方](usage.ja.md#図))．ほかのモジュールは親モノレポのテンプレートで，まだ使っていません．

## 戦略の実装

戦略は，socsim-games の `DecisionRule<MatrixGame, Exp>` を実装した型です．ここでは 3 つのメソッドを使います．

| メソッド | 呼ばれるとき | 用途 |
|---|---|---|
| `init_state` | 開始時に 1 回 | エージェントの最初の `Policy` を作る |
| `decide` | ステップ内の対戦ごと | 手を 1 つ返す (`0` = C，`1` = D) |
| `learn_step` | 各ステップの終わりに 1 回 (全対戦のあと) | 1 ステップに 1 回だけ進めたい状態を進める |

`decide` は `&self`，そのエージェントの `&mut Policy`，`DecisionContext`，乱数を受け取ります．

- **戦略は共有するコードです．** 1 つの戦略のオブジェクトを，その戦略を使うすべてのエージェントが共有するので，`decide` は `&self` で呼ばれます．エージェントごとの値を戦略のフィールドに (`Cell` などで) 持たせてはいけません．その値は保存されず，スナップショットから再開した run がずれます．
- **エージェントごとの状態は `Policy` に置きます．** `Policy` は `GameWorld` の中に保存されるので，スナップショットから復元できます．組み込みの戦略の状態 (`Policy::Builtin`) と，状態を持つ 2 つの自作戦略の状態 (`Policy::Alternator`，`Policy::Gradual`) の enum です．組み込みの戦略は `Slot<T>` トレイトで自分の状態を探すので，`state.rs` がその探索を `Policy::Builtin` に委ねています．
- **履歴は `DecisionContext` から読みます．** `opponent_history` はいま対戦している相手の履歴だけを返し，ほかのエージェントでは `None` を返します．繰り返しゲームの情報構造を，ライブラリの側で守るためです．
- **乱数は引数の `rng` からだけ引きます．** 同じ seed なら同じ結果になります．

### 状態を持たない戦略

`HistoryRule` (`strategies.rs` の前半) は，名前と関数 `fn(&Seen, &mut SimRng) -> usize` を 1 つ持つだけの戦略です．`Seen` は，相手の過去の手 (古い順) と自分の前回の手です．
TF2T，Suspicious TFT，Generous TFT，Joss，Extort-2，Prober，Soft Majority の 7 つを，この関数として書いています．状態は `Policy::Builtin(Stateless)` です．

### 状態を持つ戦略

- **Alternator** は，次に出す手を `Policy::Alternator { next }` に持ちます．`decide` は読むだけで，入れ替えは `learn_step` で行います．1 ステップに `decide` が何度呼ばれても，状態は 1 ステップに 1 つだけ進みます．
- **Gradual** は，残りの報復の回数と残りの鎮める C の回数を `Policy::Gradual { remaining_defections, remaining_calming }` に持ち，`decide` の中で進めます．各エージェントが 1 ステップに 1 回しか対戦しないので，ここではこれで正しく動きます．1 ステップに何度も `decide` が呼ばれる設定では，Alternator のように更新を `learn_step` に移す必要があります．

## 総当たりの流れ

`tournament.rs` の `tournament()` は，次の関数を順に呼びます．

1. `play_all`：全組 × 全 seed を対戦させ，利得のセル，`matches.csv`，`first_moves.csv` を作る．
2. `matrix_csv`：利得行列 (seed 平均または seed 間の標準偏差) を作る．
3. `rank_strategies`：平均利得，標準偏差，勝ち・負け・引き分けを求め，戦略を並べる．
4. `ranking_csv`：順位表を CSV にする．
5. `warn_seed_dependent_deterministic`：決定的な戦略どうしの組が seed によって変わったら `WARNING` を出す．

## テスト

- `simulation/src/tests.rs` には，自作の戦略ごとの単体テストと，Pavlov のテストがあります (計 10 個) ．状態を持たない戦略は，`Seen` を組み立てて関数を直接呼びます．確率は，seed を固定して 12,000 回引いた割合を，±0.02 の幅で確かめます．Alternator と Gradual は，`play` で `all_c` や `all_d` と対戦させて確かめます．Pavlov はテストの中でだけ定義して戦略集に足し，`play_with` で 6 つの相手・seed 0〜9 について組み込みの `wsls` と同じ結果になることを確かめます．
- `simulation/tests/cli.rs` はバイナリを実行します．`checks` の出力と総当たりの順位表が `simulation/tests/fixtures/` のファイルとバイト単位で一致すること，総当たりが CSV を 5 つ書き出すことを確かめます．
- `tools/tests/test_plot_tournament.py` は `out/` を読み，CSV の形式，非先制型の組，図の色の割り当てと配置，2 回描いて同じファイルになることを確かめます (計 9 個) ．
