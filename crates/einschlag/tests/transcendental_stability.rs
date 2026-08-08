//! What the platform's floating-point functions actually did on the machine this
//! ran on, rather than what their documentation permits them to do.
//!
//! This file is a measurement and not a guard. The distinction matters: a guard
//! refuses a thing this project controls, and nothing here is under this
//! project's control. What is recorded is the behaviour the numeric core would
//! have inherited had it called the platform, which is the evidence
//! `docs/decisions/0013-platform-math-out-of-the-numeric-core.md` rests on. The
//! refusal that keeps the core away from it is a different file,
//! `platform_math_stays_out_of_the_core.rs`.
//!
//! Both tests print their figures. Run them with the output shown:
//!
//! ```text
//! cargo test --test transcendental_stability -- --nocapture
//! ```
//!
//! This is where the platform's own functions are called on purpose, and it sits
//! outside `src/` for that reason: the check that refuses those calls reads the
//! shipped source and does not read the tests beside it.

use std::hint::black_box;

/// Angles and ratios spread across the range a reconstruction reaches. The
/// arcsine and arccosine are only defined on `[-1, 1]`, which is what bounds the
/// grid, and it is the interesting interval anyway because the ellipse relation
/// evaluates an arcsine of a ratio of two measured axes.
fn grid() -> Vec<f64> {
    (0..64)
        .map(|step| -0.99 + f64::from(step) * (1.98 / 63.0))
        .collect()
}

/// Every value in one sweep, in a fixed order, computed through the platform.
///
/// `black_box` on each argument is what makes this a runtime call rather than a
/// number the compiler worked out once and pasted in. Without it the two sweeps
/// being compared could both be the same compile-time constant, and the test
/// would be comparing a literal against itself and passing on nothing.
fn platform_sweep(x: f64, positive: f64) -> [f64; 14] {
    [
        black_box(x).sin(),
        black_box(x).cos(),
        black_box(x).tan(),
        black_box(x).asin(),
        black_box(x).acos(),
        black_box(x).atan(),
        black_box(x).atan2(black_box(positive)),
        black_box(x).exp(),
        black_box(positive).ln(),
        black_box(positive).powf(black_box(x)),
        black_box(x).hypot(black_box(positive)),
        black_box(x).cbrt(),
        black_box(x).to_radians(),
        black_box(x).to_degrees(),
    ]
}

/// The clause that is inside the determinism promise rather than inside its
/// stated bound: "can even differ within the same execution from one invocation
/// to the next". Whether it bites in practice was not established when
/// this was written and had to be measured.
///
/// A run of this test that finds nothing is a negative result and is reported as
/// one. It says the licence was not exercised here, on this build, on this
/// machine. It does not say the licence is not there, and it says nothing about
/// any other platform.
#[test]
fn the_platform_transcendentals_did_not_move_within_one_execution() {
    const REPETITIONS: usize = 2_000;

    let mut compared = 0usize;
    let mut moved = 0usize;

    for x in grid() {
        let positive = black_box(x).abs() + 0.01;
        let first = platform_sweep(x, positive);
        for _ in 0..REPETITIONS {
            let again = platform_sweep(x, positive);
            for (before, after) in first.iter().zip(again.iter()) {
                compared += 1;
                if before.to_bits() != after.to_bits() {
                    moved += 1;
                }
            }
        }
    }

    println!(
        "within one execution: {moved} of {compared} repeated invocations returned different bits"
    );

    assert_eq!(
        moved, 0,
        "a platform function returned different bits for the same argument inside \
         one execution, {moved} times out of {compared}. Its documentation permits \
         this. docs/decisions/0009-determinism.md promises it will not happen to \
         the output, and docs/decisions/0013-platform-math-out-of-the-numeric-core.md \
         is the reason the core does not call these. This is the measurement that \
         record rests on, and it has changed."
    );
}

/// Whether the choice of implementation is observable at all.
///
/// The whole of 0013 rests on it being observable: if the platform and `libm`
/// returned identical bits everywhere, pinning one of them would buy nothing and
/// the record would be ceremony. This is where that premise is checked rather
/// than assumed, and the per-function figures it prints are the ones the record
/// quotes.
#[test]
fn the_platform_and_the_pinned_implementation_do_not_agree_bit_for_bit() {
    const NAMES: [&str; 12] = [
        "sin", "cos", "tan", "asin", "acos", "atan", "atan2", "exp", "ln", "powf", "hypot", "cbrt",
    ];

    let points = grid();
    let mut differing = [0usize; 12];
    let mut largest = [0i64; 12];

    for &x in &points {
        let positive = black_box(x).abs() + 0.01;
        let platform: [f64; 12] = [
            black_box(x).sin(),
            black_box(x).cos(),
            black_box(x).tan(),
            black_box(x).asin(),
            black_box(x).acos(),
            black_box(x).atan(),
            black_box(x).atan2(black_box(positive)),
            black_box(x).exp(),
            black_box(positive).ln(),
            black_box(positive).powf(black_box(x)),
            black_box(x).hypot(black_box(positive)),
            black_box(x).cbrt(),
        ];
        let pinned: [f64; 12] = [
            einschlag::math::sin(x),
            einschlag::math::cos(x),
            einschlag::math::tan(x),
            einschlag::math::asin(x),
            einschlag::math::acos(x),
            einschlag::math::atan(x),
            einschlag::math::atan2(x, positive),
            einschlag::math::exp(x),
            einschlag::math::ln(positive),
            einschlag::math::powf(positive, x),
            einschlag::math::hypot(x, positive),
            einschlag::math::cbrt(x),
        ];

        for index in 0..NAMES.len() {
            let apart = last_places_apart(platform[index], pinned[index]);
            if apart != 0 {
                differing[index] += 1;
            }
            if apart.abs() > largest[index].abs() {
                largest[index] = apart;
            }
        }
    }

    for index in 0..NAMES.len() {
        println!(
            "{:<6} differs at {:>2} of {} grid points, largest difference {} in the last place",
            NAMES[index],
            differing[index],
            points.len(),
            largest[index]
        );
    }

    let total: usize = differing.iter().sum();
    assert!(
        total > 0,
        "the platform and the pinned implementation agreed on every one of the \
         {} points measured. That is not a failure of either of them, and it is a \
         reason to re-read docs/decisions/0013-platform-math-out-of-the-numeric-core.md: \
         its argument is that the choice of implementation is visible in the last \
         place of the answer, and on this evidence it was not.",
        points.len() * NAMES.len()
    );
}

/// How many representable values lie between two finite numbers of the same
/// sign, signed by which one is larger.
///
/// The bit patterns of positive doubles increase monotonically with the value,
/// so their difference counts the gap. This is used only to describe a
/// disagreement, never to accept one.
fn last_places_apart(left: f64, right: f64) -> i64 {
    if left.is_nan() || right.is_nan() {
        return 0;
    }
    (left.to_bits() as i64).saturating_sub(right.to_bits() as i64)
}
