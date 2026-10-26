"""総当たり大会の CSV から図 (ヒートマップ・順位・勝利回数と平均利得) を作る．

Usage:
    ipd-strategies-tools plot-tournament [--input-dir out] [--output-dir fig]
"""
from __future__ import annotations

import argparse
import csv
from dataclasses import dataclass
from pathlib import Path
from typing import Any

import numpy as np

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.colors import LinearSegmentedColormap, Normalize
from matplotlib.patches import Patch
from matplotlib.transforms import Bbox

EXPECTED_NICE = frozenset(
    {
        "all_c",
        "generous_tft",
        "gradual",
        "grim",
        "soft_majority",
        "tit_for_tat",
        "tit_for_two_tats",
        "wsls",
    }
)

LABELS = {
    "all_c": "all_c",
    "all_d": "all_d",
    "tit_for_tat": "TFT",
    "grim": "grim",
    "wsls": "wsls",
    "fictitious_play": "fictitious_play",
    "uniform_random": "uniform_random",
    "logit": "logit",
    "roth_erev": "roth_erev",
    "tit_for_two_tats": "TF2T",
    "suspicious_tft": "Suspicious TFT",
    "generous_tft": "Generous TFT",
    "joss": "Joss",
    "alternator": "Alternator",
    "extort2": "Extort-2",
    "gradual": "Gradual",
    "prober": "Prober",
    "soft_majority": "Soft Majority",
}

INK = "#1f1f1e"
MUTED = "#6b6a63"
GRID = "#e4e2dc"
BLUE = "#2a78d6"
ORANGE = "#eb6834"
DPI = 150
HEATMAP_TITLE = "総当たりの平均利得表（200 ラウンド，seed 0〜29 の平均．行が自分の利得）"
RANKING_TITLE = (
    "全対戦の 1 ラウンドあたり平均利得"
    "（seed 0〜29 の平均．誤差棒は seed 間の標準偏差）"
)
SCATTER_TITLE = "勝利回数と平均利得（勝利回数は 17 の相手のうち累積利得で上回った数）"
RANKING_REFERENCE_NOTE_Y = 0.018
SCATTER_MARKER_SIZE = 54.0
SCATTER_MARKER_EDGE_WIDTH = 0.8
NICE_GROUP_SIZE = 8
MUTUAL_COOPERATION_PAYOFF = 3.0
MUTUAL_COOPERATION_TOLERANCE = 0.005
HEATMAP_MAX_PAYOFF = 5.0


@dataclass(frozen=True)
class RankingEntry:
    rank: int
    strategy: str
    mean_payoff: float
    std_over_seeds: float
    wins: int
    losses: int
    ties: int
    stochastic: bool


@dataclass(frozen=True)
class TournamentData:
    strategies: tuple[str, ...]
    matrix_mean: np.ndarray
    matrix_std: np.ndarray
    ranking: tuple[RankingEntry, ...]

    def payoff(self, own_strategy: str, opponent_strategy: str) -> float:
        own = self.strategies.index(own_strategy)
        opponent = self.strategies.index(opponent_strategy)
        return float(self.matrix_mean[own, opponent])


@dataclass(frozen=True)
class ScatterLabel:
    strategies: tuple[str, ...]
    text: Any
    leaders: dict[str, Any]


@dataclass(frozen=True)
class ScatterLayout:
    figure: Any
    axes: Any
    labels: tuple[ScatterLabel, ...]
    points: dict[str, tuple[float, float]]
    marker_size: float
    marker_edge_width: float


def _load_matrix(path: Path) -> tuple[tuple[str, ...], np.ndarray]:
    with path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.reader(handle))
    strategies = tuple(rows[0][1:])
    row_strategies = tuple(row[0] for row in rows[1:])
    values = np.asarray([[float(value) for value in row[1:]] for row in rows[1:]])
    if row_strategies != strategies or values.shape != (18, 18):
        raise ValueError(f"invalid 18x18 strategy matrix: {path}")
    return strategies, values


def _load_ranking(path: Path) -> tuple[RankingEntry, ...]:
    with path.open(newline="", encoding="utf-8") as handle:
        rows = csv.DictReader(handle)
        ranking = tuple(
            RankingEntry(
                rank=int(row["rank"]),
                strategy=row["strategy"],
                mean_payoff=float(row["mean_payoff"]),
                std_over_seeds=float(row["std_over_seeds"]),
                wins=int(row["wins"]),
                losses=int(row["losses"]),
                ties=int(row["ties"]),
                stochastic=row["stochastic"].lower() == "true",
            )
            for row in rows
        )
    if tuple(entry.rank for entry in ranking) != tuple(range(1, 19)):
        raise ValueError(f"ranking must contain ranks 1 through 18: {path}")
    return ranking


def load_tournament(input_dir: Path) -> TournamentData:
    mean_strategies, matrix_mean = _load_matrix(input_dir / "matrix_mean.csv")
    std_strategies, matrix_std = _load_matrix(input_dir / "matrix_std.csv")
    if std_strategies != mean_strategies:
        raise ValueError("matrix_mean.csv and matrix_std.csv use different strategies")
    ranking = _load_ranking(input_dir / "ranking.csv")
    if {entry.strategy for entry in ranking} != set(mean_strategies):
        raise ValueError("ranking.csv and matrix CSVs use different strategies")
    return TournamentData(mean_strategies, matrix_mean, matrix_std, ranking)


def derive_nice_strategies(matches_path: Path) -> frozenset[str]:
    evidence: dict[str, list[tuple[int, int, int]]] = {}
    with matches_path.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            if row["a"] == "all_c":
                opponent = row["b"]
                cooperation_count = int(row["coop_b"])
            elif row["b"] == "all_c":
                opponent = row["a"]
                cooperation_count = int(row["coop_a"])
            else:
                continue
            rounds = sum(int(row[key]) for key in ("cc", "cd", "dc", "dd"))
            evidence.setdefault(opponent, []).append(
                (int(row["seed"]), cooperation_count, rounds)
            )

    if len(evidence) != 18:
        raise ValueError("matches.csv must contain all 18 strategies against all_c")
    expected_seeds = set(range(30))
    for strategy, observations in evidence.items():
        if {seed for seed, _, _ in observations} != expected_seeds:
            raise ValueError(f"missing seed evidence for {strategy} against all_c")
        if any(rounds != 200 for _, _, rounds in observations):
            raise ValueError(f"non-200-round match found for {strategy} against all_c")

    nice = frozenset(
        strategy
        for strategy, observations in evidence.items()
        if all(cooperations == 200 for _, cooperations, _ in observations)
    )
    assert nice == EXPECTED_NICE, (
        f"nice strategies derived from matches.csv differ: {sorted(nice)}"
    )
    return nice


def _configure_style() -> None:
    plt.rcParams.update(
        {
            "font.family": ["Hiragino Sans", "sans-serif"],
            "axes.unicode_minus": False,
            "figure.facecolor": "white",
            "axes.facecolor": "white",
            "axes.edgecolor": MUTED,
            "axes.labelcolor": INK,
            "axes.titlecolor": INK,
            "axes.titlesize": 11.5,
            "axes.titleweight": "regular",
            "axes.axisbelow": True,
            "xtick.color": MUTED,
            "ytick.color": MUTED,
            "xtick.labelsize": 9.5,
            "ytick.labelsize": 9.5,
            "grid.color": GRID,
            "grid.linewidth": 0.8,
            "grid.alpha": 1.0,
            "text.color": INK,
            "savefig.facecolor": "white",
        }
    )


def _style_axes(ax: plt.Axes, grid_axis: str | None = None) -> None:
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.spines["left"].set_color(MUTED)
    ax.spines["bottom"].set_color(MUTED)
    ax.tick_params(axis="both", colors=MUTED, labelsize=9.5)
    if grid_axis is not None:
        ax.grid(axis=grid_axis, color=GRID, linewidth=0.8)


def _rank_order(data: TournamentData) -> tuple[str, ...]:
    return tuple(entry.strategy for entry in data.ranking)


def _grouped_order(
    data: TournamentData, nice: frozenset[str]
) -> tuple[str, ...]:
    ranked = _rank_order(data)
    return tuple(strategy for strategy in ranked if strategy in nice) + tuple(
        strategy for strategy in ranked if strategy not in nice
    )


def _validate_reference_values(data: TournamentData) -> None:
    assert data.ranking[0].strategy == "gradual"
    assert np.isclose(data.ranking[0].mean_payoff, 2.453, atol=0.0005)
    assert np.isclose(data.payoff("extort2", "all_c"), 3.50, atol=0.005)
    assert np.isclose(data.payoff("all_d", "all_c"), 5.00, atol=0.005)
    assert np.isclose(data.payoff("gradual", "all_d"), 0.94, atol=0.005)


def _heatmap_color_mapping() -> tuple[LinearSegmentedColormap, Normalize]:
    cooperation_position = MUTUAL_COOPERATION_PAYOFF / HEATMAP_MAX_PAYOFF
    tolerance_position = (
        MUTUAL_COOPERATION_PAYOFF + MUTUAL_COOPERATION_TOLERANCE
    ) / HEATMAP_MAX_PAYOFF
    cmap = LinearSegmentedColormap.from_list(
        "tournament_payoff",
        [
            (0.0, "#f7fbff"),
            (cooperation_position / 2.0, "#8ebdec"),
            (cooperation_position, BLUE),
            (tolerance_position, BLUE),
            (tolerance_position + 0.0001, "#f6c9b8"),
            (1.0, "#c0392b"),
        ],
        N=4096,
    )
    return cmap, Normalize(vmin=0.0, vmax=HEATMAP_MAX_PAYOFF, clip=True)


def _save_heatmap(
    data: TournamentData, nice: frozenset[str], output_path: Path
) -> None:
    order = _grouped_order(data, nice)
    positions = [data.strategies.index(strategy) for strategy in order]
    matrix = data.matrix_mean[np.ix_(positions, positions)]
    ranks = {entry.strategy: entry.rank for entry in data.ranking}
    cmap, norm = _heatmap_color_mapping()

    fig, ax = plt.subplots(figsize=(10.6, 9.4))
    image = ax.imshow(matrix, cmap=cmap, norm=norm, aspect="equal")
    ax.set_xticks(np.arange(18), labels=[LABELS[strategy] for strategy in order])
    ax.set_yticks(
        np.arange(18),
        labels=[f"{ranks[strategy]}  {LABELS[strategy]}" for strategy in order],
    )
    plt.setp(ax.get_xticklabels(), rotation=55, ha="right", rotation_mode="anchor")
    ax.set_xlabel("相手の戦略", labelpad=12)
    ax.set_ylabel("自分の戦略", labelpad=12)
    ax.set_title(HEATMAP_TITLE, loc="left", pad=38)
    _style_axes(ax)
    for label, strategy in zip(ax.get_xticklabels(), order):
        label.set_color(BLUE if strategy in nice else ORANGE)
    for label, strategy in zip(ax.get_yticklabels(), order):
        label.set_color(BLUE if strategy in nice else ORANGE)

    group_specs = (
        ((NICE_GROUP_SIZE - 1) / 2.0, f"非先制型 ({NICE_GROUP_SIZE})", BLUE),
        (
            NICE_GROUP_SIZE + (len(order) - NICE_GROUP_SIZE - 1) / 2.0,
            f"先制型 ({len(order) - NICE_GROUP_SIZE})",
            ORANGE,
        ),
    )
    for center, label, color in group_specs:
        ax.text(
            center,
            1.012,
            label,
            transform=ax.get_xaxis_transform(),
            ha="center",
            va="bottom",
            color=color,
            fontsize=9.5,
        )

    group_boundary = NICE_GROUP_SIZE - 0.5
    ax.axhline(group_boundary, color=INK, linewidth=1.6)
    ax.axvline(group_boundary, color=INK, linewidth=1.6)

    for row in range(18):
        for column in range(18):
            value = matrix[row, column]
            red, green, blue, _ = cmap(norm(value))
            luminance = 0.2126 * red + 0.7152 * green + 0.0722 * blue
            ax.text(
                column,
                row,
                f"{value:.2f}",
                ha="center",
                va="center",
                color="white" if luminance < 0.56 else INK,
                fontsize=7.5,
            )

    colorbar = fig.colorbar(
        image,
        ax=ax,
        fraction=0.042,
        pad=0.035,
        ticks=(0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0),
    )
    colorbar.ax.axhline(MUTUAL_COOPERATION_PAYOFF, color=INK, linewidth=1.0)
    colorbar.set_label("1 ラウンドあたりの平均利得", color=INK, labelpad=10)
    colorbar.ax.tick_params(colors=MUTED, labelsize=9.5)
    colorbar.outline.set_edgecolor(MUTED)
    fig.subplots_adjust(left=0.25, right=0.90, bottom=0.27, top=0.91)
    fig.savefig(output_path, dpi=DPI, bbox_inches="tight")
    plt.close(fig)


def _save_ranking(
    data: TournamentData, nice: frozenset[str], output_path: Path
) -> None:
    entries = data.ranking
    y_positions = np.arange(len(entries))
    colors = [BLUE if entry.strategy in nice else ORANGE for entry in entries]

    fig, ax = plt.subplots(figsize=(10.2, 7.3))
    ax.barh(
        y_positions,
        [entry.mean_payoff for entry in entries],
        height=0.68,
        color=colors,
        edgecolor="none",
        zorder=2,
    )
    for y_position, entry in zip(y_positions, entries):
        if entry.std_over_seeds > 0:
            ax.errorbar(
                entry.mean_payoff,
                y_position,
                xerr=entry.std_over_seeds,
                fmt="none",
                ecolor=INK,
                elinewidth=1.0,
                capsize=2.5,
                capthick=1.0,
                zorder=3,
            )
        ax.text(
            entry.mean_payoff + entry.std_over_seeds + 0.025,
            y_position,
            f"{entry.mean_payoff:.3f}",
            va="center",
            ha="left",
            fontsize=8.8,
            color=INK,
        )

    ax.set_yticks(
        y_positions,
        labels=[f"{entry.rank}  {LABELS[entry.strategy]}" for entry in entries],
    )
    ax.invert_yaxis()
    ax.set_xlim(0, 3)
    ax.set_xlabel("1 ラウンドあたりの平均利得")
    ax.set_title(RANKING_TITLE, loc="left", pad=16)
    _style_axes(ax, grid_axis="x")
    for x_value, label, alignment in (
        (1.0, "相互裏切り=1", "left"),
        (3.0, "相互協力=3", "right"),
    ):
        if ax.get_xlim()[0] <= x_value <= ax.get_xlim()[1]:
            ax.axvline(x_value, color=MUTED, linestyle="--", linewidth=1.0, zorder=1)
            ax.text(
                x_value,
                RANKING_REFERENCE_NOTE_Y,
                label,
                transform=ax.get_xaxis_transform(),
                ha=alignment,
                va="bottom",
                fontsize=8.5,
                color=MUTED,
                bbox={
                    "facecolor": "white",
                    "edgecolor": "none",
                    "alpha": 0.88,
                    "pad": 0.8,
                },
                zorder=4,
            )
    ax.legend(
        handles=[
            Patch(facecolor=BLUE, label="非先制型の戦略"),
            Patch(facecolor=ORANGE, label="先制型の戦略"),
        ],
        loc="lower right",
        bbox_to_anchor=(1.0, 0.04),
        frameon=False,
        fontsize=9.5,
    )
    fig.subplots_adjust(left=0.23, right=0.97, bottom=0.11, top=0.88)
    fig.savefig(output_path, dpi=DPI, bbox_inches="tight")
    plt.close(fig)


def _build_wins_vs_payoff(
    data: TournamentData, nice: frozenset[str]
) -> ScatterLayout:
    fig, ax = plt.subplots(figsize=(10.2, 6.2))
    points = {
        entry.strategy: (float(entry.wins), entry.mean_payoff)
        for entry in data.ranking
    }
    for entry in data.ranking:
        ax.scatter(
            entry.wins,
            entry.mean_payoff,
            s=SCATTER_MARKER_SIZE,
            color=BLUE if entry.strategy in nice else ORANGE,
            edgecolor="white",
            linewidth=SCATTER_MARKER_EDGE_WIDTH,
            zorder=3,
        )

    ax.set_xlim(-0.65, 17.65)
    payoffs = [entry.mean_payoff for entry in data.ranking]
    ax.set_ylim(min(payoffs) - 0.15, max(payoffs) + 0.15)
    ax.set_xticks(range(18))
    ax.set_xlabel("勝利回数")
    ax.set_ylabel("1 ラウンドあたりの平均利得")
    ax.set_title(SCATTER_TITLE, loc="left", pad=14)
    _style_axes(ax, grid_axis="both")
    ax.legend(
        handles=[
            Patch(facecolor=BLUE, label="非先制型の戦略"),
            Patch(facecolor=ORANGE, label="先制型の戦略"),
        ],
        loc="upper right",
        frameon=False,
        fontsize=9.5,
    )

    placement_specs = (
        (("gradual",), "Gradual", (4.18, 2.493), "left", "center"),
        (("soft_majority",), "Soft Majority", (2.82, 2.472), "right", "center"),
        (
            ("tit_for_tat", "generous_tft"),
            "TFT / Generous TFT",
            (0.48, 2.368),
            "left",
            "center",
        ),
        (("grim",), "grim", (5.20, 2.258), "left", "center"),
        (
            ("wsls", "tit_for_two_tats"),
            "wsls / TF2T",
            (0.48, 2.245),
            "left",
            "center",
        ),
        (("alternator",), "Alternator", (6.20, 2.160), "left", "center"),
        (("prober",), "Prober", (7.20, 2.110), "left", "center"),
        (
            ("suspicious_tft",),
            "Suspicious TFT",
            (8.20, 2.060),
            "left",
            "center",
        ),
        (("all_c",), "all_c", (0.35, 2.052), "left", "center"),
        (
            ("uniform_random",),
            "uniform_random",
            (5.82, 1.938),
            "right",
            "center",
        ),
        (("joss",), "Joss", (13.15, 1.905), "left", "center"),
        (("roth_erev",), "roth_erev", (7.15, 1.895), "left", "center"),
        (("logit",), "logit", (8.20, 1.805), "left", "center"),
        (
            ("fictitious_play",),
            "fictitious_play",
            (15.00, 1.700),
            "center",
            "center",
        ),
        (("all_d",), "all_d", (16.22, 1.810), "left", "center"),
        (("extort2",), "Extort-2", (12.82, 1.680), "right", "center"),
    )
    labels = []
    for strategies, label, label_position, horizontal, vertical in placement_specs:
        leaders = {}
        for strategy in strategies:
            point = points[strategy]
            (leader,) = ax.plot(
                [point[0], label_position[0]],
                [point[1], label_position[1]],
                color=MUTED,
                linewidth=0.65,
                alpha=0.85,
                zorder=2,
            )
            leaders[strategy] = leader
        text = ax.text(
            label_position[0],
            label_position[1],
            label,
            ha=horizontal,
            va=vertical,
            fontsize=8.3,
            color=INK,
            bbox={"facecolor": "white", "edgecolor": "none", "alpha": 0.9, "pad": 0.3},
            zorder=4,
        )
        labels.append(ScatterLabel(strategies, text, leaders))

    fig.subplots_adjust(left=0.10, right=0.97, bottom=0.12, top=0.89)
    return ScatterLayout(
        fig,
        ax,
        tuple(labels),
        points,
        SCATTER_MARKER_SIZE,
        SCATTER_MARKER_EDGE_WIDTH,
    )


def _assert_scatter_layout(layout: ScatterLayout) -> None:
    layout.figure.set_dpi(DPI)
    layout.figure.canvas.draw()
    renderer = layout.figure.canvas.get_renderer()
    boxes = {
        label.text.get_text(): label.text.get_window_extent(renderer)
        for label in layout.labels
    }
    box_items = tuple(boxes.items())
    for index, (first_name, first_box) in enumerate(box_items):
        for second_name, second_box in box_items[index + 1 :]:
            assert not first_box.overlaps(second_box), (
                f"scatter label overlap: {first_name!r} and {second_name!r}"
            )

    marker_radius = (
        np.sqrt(layout.marker_size) / 2.0 + layout.marker_edge_width / 2.0
    ) * layout.figure.dpi / 72.0
    marker_boxes = {}
    for strategy, point in layout.points.items():
        center_x, center_y = layout.axes.transData.transform(point)
        marker_boxes[strategy] = Bbox.from_extents(
            center_x - marker_radius,
            center_y - marker_radius,
            center_x + marker_radius,
            center_y + marker_radius,
        )
    for label_name, label_box in boxes.items():
        for strategy, marker_box in marker_boxes.items():
            assert not label_box.overlaps(marker_box), (
                f"scatter label {label_name!r} overlaps marker {strategy!r}"
            )

    for label in layout.labels:
        for strategy in label.strategies:
            leader = label.leaders[strategy]
            point = layout.points[strategy]
            assert np.isclose(leader.get_xdata()[0], point[0])
            assert np.isclose(leader.get_ydata()[0], point[1])


def _save_wins_vs_payoff(
    data: TournamentData, nice: frozenset[str], output_path: Path
) -> None:
    layout = _build_wins_vs_payoff(data, nice)
    _assert_scatter_layout(layout)
    layout.figure.savefig(output_path, dpi=DPI, bbox_inches="tight")
    plt.close(layout.figure)


def generate_figures(input_dir: Path, output_dir: Path) -> tuple[Path, ...]:
    _configure_style()
    data = load_tournament(input_dir)
    nice = derive_nice_strategies(input_dir / "matches.csv")
    _validate_reference_values(data)
    output_dir.mkdir(parents=True, exist_ok=True)
    paths = (
        output_dir / "tournament-heatmap.png",
        output_dir / "tournament-ranking.png",
        output_dir / "tournament-wins-vs-payoff.png",
    )
    _save_heatmap(data, nice, paths[0])
    _save_ranking(data, nice, paths[1])
    _save_wins_vs_payoff(data, nice, paths[2])
    return paths


def main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(
        prog="ipd-strategies-tools plot-tournament",
        description="総当たり大会の CSV から図を生成する",
    )
    parser.add_argument(
        "--input-dir",
        type=Path,
        default=Path("out"),
        help="シミュレーションの出力ディレクトリ (default: out)",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path("fig"),
        help="図の書き出し先 (default: fig)",
    )
    args = parser.parse_args(argv)
    for path in generate_figures(args.input_dir, args.output_dir):
        print(path)


if __name__ == "__main__":
    main()
