"""ipd-strategies-tools — 統合 CLI ディスパッチャ．

Usage:
    ipd-strategies-tools visualize [...]
    ipd-strategies-tools visualize-sweep [...]
    ipd-strategies-tools show-experiment-settings [...]
"""
from __future__ import annotations

import argparse
import sys


def main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(
        prog="ipd-strategies-tools",
        description="Visualization & analysis tools",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("visualize", help="単一実行結果の可視化", add_help=False)
    subparsers.add_parser(
        "visualize-sweep",
        help="スイープ結果の可視化 (パラメータ依存図 + 組み合わせ別グリッドアニメーション)",
        add_help=False,
    )
    subparsers.add_parser(
        "show-experiment-settings",
        help="実行結果ディレクトリの設定値を表示 (config.json / sweep_config.json)",
        add_help=False,
    )

    argv = sys.argv[1:] if argv is None else argv
    if not argv or argv[0] in {"-h", "--help"}:
        parser.parse_args(argv)
        return

    command = argv[0]
    rest = argv[1:]
    if command == "visualize":
        from ipd_strategies_tools.visualize import main as run_main
        run_main(rest)
    elif command == "visualize-sweep":
        from ipd_strategies_tools.visualize_sweep import main as run_main
        run_main(rest)
    elif command == "show-experiment-settings":
        from ipd_strategies_tools.show_experiment_settings import main as run_main
        run_main(rest)
    else:
        parser.parse_args(argv)  # 不正なサブコマンドはここでエラーになる


if __name__ == "__main__":
    main()
