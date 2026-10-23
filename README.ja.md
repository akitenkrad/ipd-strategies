<p align="center"><img src="docs/assets/hero.svg" width="100%"></p>

[English](README.md) | **日本語**

# ipd-strategies

繰り返し囚人のジレンマの 18 戦略を総当たりで対戦させるプログラムです．Rust で書き，
[socsim-games](https://github.com/akitenkrad/rs-social-simulation-tools/tree/3b246d2/crates/socsim-games) の上に作っています．
18 戦略のうち 9 つは socsim-games の組み込みで，残りの 9 つは `DecisionRule` トレイトを実装してここで作りました．
すべての組を seed 0〜29 で 200 ラウンドずつ対戦させ，利得行列と順位表を CSV に書き出します．
Zenn の記事 [ゲーム理論×シミュレーション: 囚人のジレンマにおける戦略と Rust による実装](https://zenn.dev/akitenkrad/articles/9d99e25e96f48b) のコードです．

## クイックスタート

安定版の Rust ツールチェーンが要ります．リポジトリのルートで実行します．

```bash
# 総当たりを実行し，out/ に CSV を書き出す
cargo run --release -q

# テストを実行する
cargo test
```

## ドキュメント

- [使い方](docs/usage.ja.md) — コマンド，`legacy` モード，出力する CSV
- [戦略](docs/strategies.ja.md) — 18 の戦略，利得，順位の決め方
- [構成](docs/architecture.ja.md) — ソースの構成，規則の状態の置き場所，テスト

## ライセンス

[MIT](LICENSE)
