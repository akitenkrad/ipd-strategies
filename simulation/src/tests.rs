use socsim_core::SimRng;

use crate::arena::{play, ROUNDS};
use crate::history_rules::{self, Choose, Seen, GENEROSITY, JOSS_DEFECT};
use crate::state::{C, D};

const PROBABILITY_TRIALS: usize = 12_000;
const PROBABILITY_TOLERANCE: f64 = 0.02;

fn observed(opp: &[usize], mine: Option<usize>) -> Seen {
    Seen {
        opp: opp.to_vec(),
        mine,
    }
}

fn dummy_rng() -> SimRng {
    SimRng::from_seed(0)
}

fn cooperation_rate(choose: Choose, seen: &Seen, seed: u64) -> f64 {
    let mut rng = SimRng::from_seed(seed);
    let cooperations = (0..PROBABILITY_TRIALS)
        .filter(|_| choose(seen, &mut rng) == C)
        .count();
    cooperations as f64 / PROBABILITY_TRIALS as f64
}

fn assert_rate_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= PROBABILITY_TOLERANCE,
        "expected rate {expected:.6} +/- {PROBABILITY_TOLERANCE:.6}, got {actual:.6}"
    );
}

#[test]
fn tit_for_two_tats() {
    let mut rng = dummy_rng();

    assert_eq!(
        history_rules::tit_for_two_tats(&observed(&[C, D], None), &mut rng),
        C
    );
    assert_eq!(
        history_rules::tit_for_two_tats(&observed(&[D, D], None), &mut rng),
        D
    );
}

#[test]
fn suspicious_tft() {
    let mut rng = dummy_rng();

    assert_eq!(
        history_rules::suspicious_tft(&observed(&[], None), &mut rng),
        D
    );
    assert_eq!(
        history_rules::suspicious_tft(&observed(&[C], None), &mut rng),
        C
    );
    assert_eq!(
        history_rules::suspicious_tft(&observed(&[D], None), &mut rng),
        D
    );
}

#[test]
fn generous_tft() {
    let after_cooperation = observed(&[C], None);
    let mut rng = SimRng::from_seed(1);
    assert!((0..PROBABILITY_TRIALS)
        .all(|_| history_rules::generous_tft(&after_cooperation, &mut rng) == C));

    let rate = cooperation_rate(history_rules::generous_tft, &observed(&[D], None), 2);
    assert_rate_close(rate, GENEROSITY);
}

#[test]
fn joss() {
    let mut rng = SimRng::from_seed(3);
    assert_eq!(history_rules::joss(&observed(&[], None), &mut rng), C);
    assert!(
        (0..PROBABILITY_TRIALS).all(|_| history_rules::joss(&observed(&[D], None), &mut rng) == D)
    );

    let cooperation_rate = cooperation_rate(history_rules::joss, &observed(&[C], None), 4);
    assert_rate_close(1.0 - cooperation_rate, JOSS_DEFECT);
}

#[test]
fn pavlov() {
    let mut rng = dummy_rng();
    assert_eq!(history_rules::pavlov(&observed(&[], None), &mut rng), C);

    for (mine, opponent, expected) in [(C, C, C), (C, D, D), (D, C, D), (D, D, C)] {
        assert_eq!(
            history_rules::pavlov(&observed(&[opponent], Some(mine)), &mut rng),
            expected
        );
    }
}

#[test]
fn extort2() {
    let mut rng = dummy_rng();
    assert_eq!(history_rules::extort2(&observed(&[], None), &mut rng), C);
    assert!((0..PROBABILITY_TRIALS)
        .all(|_| history_rules::extort2(&observed(&[D], Some(D)), &mut rng) == D));

    for (seed, mine, opponent, expected) in [
        (5, C, C, 8.0 / 9.0),
        (6, C, D, 1.0 / 2.0),
        (7, D, C, 1.0 / 3.0),
    ] {
        let rate = cooperation_rate(
            history_rules::extort2,
            &observed(&[opponent], Some(mine)),
            seed,
        );
        assert_rate_close(rate, expected);
    }
}

#[test]
fn prober() {
    let mut rng = dummy_rng();
    assert_eq!(history_rules::prober(&observed(&[], None), &mut rng), D);
    assert_eq!(history_rules::prober(&observed(&[D], None), &mut rng), C);
    assert_eq!(history_rules::prober(&observed(&[D, D], None), &mut rng), C);
    assert_eq!(
        history_rules::prober(&observed(&[D, C, C], None), &mut rng),
        D
    );
    assert_eq!(
        history_rules::prober(&observed(&[D, D, C], None), &mut rng),
        C
    );
    assert_eq!(
        history_rules::prober(&observed(&[D, C, D], None), &mut rng),
        D
    );
}

#[test]
fn soft_majority() {
    let mut rng = dummy_rng();
    assert_eq!(
        history_rules::soft_majority(&observed(&[], None), &mut rng),
        C
    );
    assert_eq!(
        history_rules::soft_majority(&observed(&[C, D], None), &mut rng),
        C
    );
    assert_eq!(
        history_rules::soft_majority(&observed(&[C, D, D], None), &mut rng),
        D
    );
}

#[test]
fn alternator() {
    let (_, moves) = play("alternator", "all_c", 0);

    assert_eq!(moves.len(), ROUNDS);
    for (round, actions) in moves.iter().enumerate() {
        assert_eq!(actions[0], if round % 2 == 0 { C } else { D });
    }
}

#[test]
fn gradual() {
    let (_, moves) = play("gradual", "all_d", 0);
    let expected = [
        C, D, C, C, D, D, D, D, C, C, D, D, D, D, D, D, D, D, D, D, C, C,
    ];

    assert_eq!(moves.len(), ROUNDS);
    assert_eq!(
        moves[..expected.len()]
            .iter()
            .map(|actions| actions[0])
            .collect::<Vec<_>>(),
        expected
    );
}
