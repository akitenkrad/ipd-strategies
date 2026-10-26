from __future__ import annotations

import hashlib
import itertools
import math
import struct
import tempfile
import unittest
from pathlib import Path

import numpy as np


# リポジトリのルート．テストは `cargo run --release -q` が書き出した out/ を読む．
PROJECT_DIR = Path(__file__).resolve().parents[2]

from ipd_strategies_tools import plot_tournament as tournament  # noqa: E402
from matplotlib.transforms import Bbox  # noqa: E402


class TournamentDataContractTest(unittest.TestCase):
    def test_csv_contract_and_reference_values(self) -> None:
        data = tournament.load_tournament(PROJECT_DIR / "out")

        self.assertEqual(len(data.strategies), 18)
        self.assertEqual(data.matrix_mean.shape, (18, 18))
        self.assertEqual(data.ranking[0].strategy, "gradual")
        self.assertAlmostEqual(data.ranking[0].mean_payoff, 2.453, places=3)
        self.assertAlmostEqual(data.payoff("extort2", "all_c"), 3.50, places=2)
        self.assertAlmostEqual(data.payoff("all_d", "all_c"), 5.00, places=2)
        self.assertAlmostEqual(data.payoff("gradual", "all_d"), 0.94, places=2)

    def test_nice_strategies_are_derived_from_all_c_matches(self) -> None:
        derive = getattr(tournament, "derive_nice_strategies", None)
        self.assertIsNotNone(derive)
        nice = derive(PROJECT_DIR / "out" / "matches.csv")

        self.assertEqual(
            nice,
            frozenset(
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
            ),
        )

    def test_three_zenn_pngs_are_generated(self) -> None:
        generate = getattr(tournament, "generate_figures", None)
        self.assertIsNotNone(generate)
        with tempfile.TemporaryDirectory() as temporary_dir:
            output_dir = Path(temporary_dir)
            paths = generate(PROJECT_DIR / "out", output_dir)

            self.assertEqual(
                {path.name for path in paths},
                {
                    "tournament-heatmap.png",
                    "tournament-ranking.png",
                    "tournament-wins-vs-payoff.png",
                },
            )
            for path in paths:
                self.assertTrue(path.is_file())
                with path.open("rb") as handle:
                    self.assertEqual(handle.read(8), b"\x89PNG\r\n\x1a\n")
                    handle.read(8)
                    width, height = struct.unpack(">II", handle.read(8))
                self.assertGreaterEqual(width, 1200)
                self.assertGreaterEqual(height, 850)

    def test_missing_input_and_repeat_generation(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_dir:
            temporary_path = Path(temporary_dir)
            with self.assertRaises(FileNotFoundError):
                tournament.generate_figures(
                    temporary_path / "missing", temporary_path / "unused"
                )

            output_dir = temporary_path / "fig"
            first_paths = tournament.generate_figures(PROJECT_DIR / "out", output_dir)
            first_hashes = {
                path.name: hashlib.sha256(path.read_bytes()).hexdigest()
                for path in first_paths
            }
            second_paths = tournament.generate_figures(PROJECT_DIR / "out", output_dir)
            second_hashes = {
                path.name: hashlib.sha256(path.read_bytes()).hexdigest()
                for path in second_paths
            }
            self.assertEqual(first_hashes, second_hashes)

    def test_titles_use_fullwidth_japanese_punctuation(self) -> None:
        expected = {
            "HEATMAP_TITLE": "総当たりの平均利得表（200 ラウンド，seed 0〜29 の平均．行が自分の利得）",
            "RANKING_TITLE": "全対戦の 1 ラウンドあたり平均利得（seed 0〜29 の平均．誤差棒は seed 間の標準偏差）",
            "SCATTER_TITLE": "勝利回数と平均利得（勝利回数は 17 の相手のうち累積利得で上回った数）",
        }
        for name, title in expected.items():
            self.assertEqual(getattr(tournament, name, None), title)

    def test_heatmap_uses_blue_through_three_then_switches_to_red(self) -> None:
        color_mapping = getattr(tournament, "_heatmap_color_mapping", None)
        self.assertIsNotNone(color_mapping)
        cmap, norm = color_mapping()
        color_at_one = np.asarray(cmap(norm(1.0)))[:3]
        color_at_three = np.asarray(cmap(norm(3.0)))[:3]
        color_at_three_with_tolerance = np.asarray(cmap(norm(3.004)))[:3]
        color_at_three_point_zero_one = np.asarray(cmap(norm(3.01)))[:3]
        color_at_four = np.asarray(cmap(norm(4.0)))[:3]
        color_at_five = np.asarray(cmap(norm(5.0)))[:3]

        self.assertGreater(np.linalg.norm(color_at_one - color_at_three), 0.35)
        np.testing.assert_allclose(color_at_three, color_at_three_with_tolerance)
        self.assertGreater(color_at_three[2], color_at_three[0])
        self.assertGreater(color_at_three_point_zero_one[0], color_at_three_point_zero_one[2])
        self.assertGreater(color_at_four[0], color_at_four[2])
        self.assertGreater(np.linalg.norm(color_at_three - color_at_four), 0.35)
        self.assertGreater(np.linalg.norm(color_at_four - color_at_five), 0.1)

    def test_heatmap_grouped_order_keeps_ranking_order_within_each_group(self) -> None:
        grouped_order = getattr(tournament, "_grouped_order", None)
        self.assertIsNotNone(grouped_order)
        data = tournament.load_tournament(PROJECT_DIR / "out")
        nice = tournament.derive_nice_strategies(PROJECT_DIR / "out" / "matches.csv")

        order = grouped_order(data, nice)
        expected_nice = tuple(
            entry.strategy for entry in data.ranking if entry.strategy in nice
        )
        expected_other = tuple(
            entry.strategy for entry in data.ranking if entry.strategy not in nice
        )

        self.assertEqual(len(expected_nice), 8)
        self.assertEqual(order[:8], expected_nice)
        self.assertEqual(order[8:], expected_other)

    def test_ranking_reference_notes_are_near_the_lower_edge(self) -> None:
        note_y = getattr(tournament, "RANKING_REFERENCE_NOTE_Y", None)
        self.assertIsNotNone(note_y)
        self.assertGreaterEqual(note_y, 0.0)
        self.assertLessEqual(note_y, 0.08)

    def test_scatter_label_geometry_is_unambiguous(self) -> None:
        build_layout = getattr(tournament, "_build_wins_vs_payoff", None)
        self.assertIsNotNone(build_layout)
        tournament._configure_style()
        data = tournament.load_tournament(PROJECT_DIR / "out")
        nice = tournament.derive_nice_strategies(PROJECT_DIR / "out" / "matches.csv")
        layout = build_layout(data, nice)
        try:
            layout.figure.set_dpi(tournament.DPI)
            layout.figure.canvas.draw()
            renderer = layout.figure.canvas.get_renderer()
            label_boxes = {
                label.text.get_text(): label.text.get_window_extent(renderer)
                for label in layout.labels
            }
            for (first_name, first_box), (second_name, second_box) in itertools.combinations(
                label_boxes.items(), 2
            ):
                self.assertFalse(
                    first_box.overlaps(second_box),
                    f"label overlap: {first_name!r} and {second_name!r}",
                )

            marker_radius = (
                math.sqrt(layout.marker_size) / 2.0 + layout.marker_edge_width / 2.0
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
            for label_name, label_box in label_boxes.items():
                for strategy, marker_box in marker_boxes.items():
                    self.assertFalse(
                        label_box.overlaps(marker_box),
                        f"label {label_name!r} overlaps marker {strategy!r}",
                    )

            self.assertIn(
                "TFT / Generous TFT",
                {label.text.get_text() for label in layout.labels},
            )
            self.assertIn(
                "wsls / TF2T",
                {label.text.get_text() for label in layout.labels},
            )
            for label in layout.labels:
                for strategy in label.strategies:
                    leader = label.leaders.get(strategy)
                    if leader is not None:
                        self.assertAlmostEqual(leader.get_xdata()[0], layout.points[strategy][0])
                        self.assertAlmostEqual(leader.get_ydata()[0], layout.points[strategy][1])
                        continue
                    label_box = label_boxes[label.text.get_text()]
                    label_center = np.asarray(
                        [
                            (label_box.x0 + label_box.x1) / 2.0,
                            (label_box.y0 + label_box.y1) / 2.0,
                        ]
                    )
                    distances = {
                        point_strategy: np.linalg.norm(
                            label_center
                            - layout.axes.transData.transform(point_coordinates)
                        )
                        for point_strategy, point_coordinates in layout.points.items()
                    }
                    self.assertEqual(min(distances, key=distances.get), strategy)
        finally:
            tournament.plt.close(layout.figure)


if __name__ == "__main__":
    unittest.main()
