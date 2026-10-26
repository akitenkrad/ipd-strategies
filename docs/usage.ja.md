[English](usage.md) | **日本語**

# 使い方

コマンドはすべてリポジトリのルートで実行します．シミュレーションの crate は `simulation/` にあり，`ipd-strategies` という名前のバイナリを作ります．
依存の socsim-games と socsim-core は，[rs-social-simulation-tools](https://github.com/akitenkrad/rs-social-simulation-tools) のコミット `3b246d2` に固定しています．

## コマンド

```bash
# 総当たりを実行し，out/ に CSV を書き出す
cargo run --release -q

# TFT を自作の 5 戦略と 1 対 1 で対戦させ，
# Generous TFT と Joss を対戦させる
cargo run --release -q -- checks

# 単体テストと CLI のテストを実行する
cargo test

# lint を実行する
cargo clippy --all-targets -- -D warnings

# 整形を確かめる
cargo fmt --check
```

`rustfmt.toml` で `max_width` を 100 (rustfmt の既定値) に固定しています．ホームディレクトリの設定で `max_width` を変えていても，コードは整形し直されません．

## 総当たり

引数なしで実行すると，18 戦略から 2 つを選ぶ 171 組 (自分自身との組を含む) を，seed 0〜29 で 200 ラウンドずつ対戦させます．
カレントディレクトリの `out/` に CSV を 5 つ書き出し，順位表を標準出力に出します．

| ファイル | 中身 |
|---|---|
| `matches.csv` | 組と seed ごとに 1 行．各自の累積利得，C の回数，(C, C)・(C, D)・(D, C)・(D, D) の回数 |
| `matrix_mean.csv` | 18 × 18 の利得行列．行の戦略が列の戦略から得た 1 ラウンドあたりの利得の seed 平均 |
| `matrix_std.csv` | 同じ利得行列の seed 間の標準偏差 |
| `ranking.csv` | 順位表．平均利得，seed 間の標準偏差，勝ち・負け・引き分け，確率的な戦略かどうか |
| `first_moves.csv` | 各組の seed 0 での最初の 16 ラウンドの手 |

`ranking.csv` の列は `rank,strategy,mean_payoff,std_over_seeds,wins,losses,ties,stochastic` です．
値の求め方は [戦略](strategies.ja.md#順位の決め方) に書きました．

確率的な戦略を含まない組は，seed によらず同じ結果になるはずです．そうならない組があると，順位表のあとに `WARNING` の行を出します．

## 図

`tools/` の Python パッケージが，`out/` の CSV から Zenn の記事の図を描きます．
Python 3.11 以上と [uv](https://docs.astral.sh/uv/) が要ります．先に総当たりを実行しておきます．

```bash
# fig/ に PNG を 3 つ書き出す
uv run ipd-strategies-tools plot-tournament

# 入力と出力のディレクトリを指定する
uv run ipd-strategies-tools plot-tournament --input-dir out --output-dir fig

# 図のテストを実行する (out/ を読みます)
uv run python -m unittest discover -s tools/tests
```

| ファイル | 中身 |
|---|---|
| `tournament-heatmap.png` | `matrix_mean.csv` の利得行列．非先制型を前に，先制型を後ろに並べ，それぞれの中は順位の順 |
| `tournament-ranking.png` | 各戦略の平均利得と seed 間の標準偏差．順位の順 |
| `tournament-wins-vs-payoff.png` | 18 戦略の勝利回数と平均利得の散布図 |

どの戦略が非先制型かは `matches.csv` から求めます．記事が想定する組と違えば，エラーで止まります．

## `checks` モード

`cargo run --release -q -- checks` は次の 2 つを出力します．

1. TFT を TF2T，Generous TFT，Alternator，Suspicious TFT，Joss と seed 0〜9 で対戦させた結果．累積利得，C の回数，(C, C) と (D, D) の回数を表にし，続けて各対戦の最初の 16 ラウンドの手を出します．
2. Generous TFT と Joss の対戦の結果．形式は 1 と同じです．
