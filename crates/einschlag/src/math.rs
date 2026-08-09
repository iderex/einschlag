//! The only route the core has to a transcendental function.
//!
//! `docs/decisions/0009-determinism.md` promises that the same input file, the
//! same seed and the same build produce byte-identical output. The standard
//! library does not support the unbounded half of that: its documentation
//! reserves the right for a transcendental to return a different result from one
//! invocation to the next inside a single execution, and it makes no promise
//! across platforms or across compiler releases. Where the answer is a region
//! that somebody is placed inside or outside of, a last-place difference that
//! moves with the machine is not a rounding detail, it is two reconstructions
//! that cannot be compared.
//!
//! `docs/decisions/0013-platform-math-out-of-the-numeric-core.md` is where the
//! choice of implementation is argued, what it costs, and what would falsify it.
//! What is decided there and implemented here: every transcendental the core
//! evaluates goes through `libm`, at one pinned version, and the platform's own
//! implementation is not reachable from the core at all.
//!
//! `crates/einschlag/tests/platform_math_stays_out_of_the_core.rs` is the
//! mechanism that keeps it that way, because the platform version of every
//! function below is reachable by typing a dot and a convention does not survive
//! that.
//!
//! Two kinds of function are deliberately absent. Square root, fused multiply
//! add and the ordinary arithmetic operators are specified by IEEE 754 to be
//! correctly rounded, so the platform has no latitude in them and nothing is
//! gained by routing them anywhere. Degrees and radians are here rather than
//! absent, because the standard library's conversions carry the same reservation
//! as the transcendentals while being one multiplication, and one multiplication
//! written out is exact.

/// Sine of an angle in radians.
pub fn sin(x: f64) -> f64 {
    libm::sin(x)
}

/// Cosine of an angle in radians.
pub fn cos(x: f64) -> f64 {
    libm::cos(x)
}

/// Tangent of an angle in radians.
pub fn tan(x: f64) -> f64 {
    libm::tan(x)
}

/// Arcsine, in radians.
///
/// This is the one the ellipse relation is built on: the impact angle of a
/// perforation comes out of the ratio of the minor axis to the major axis
/// through an arcsine, so every hole in a scene reaches this function.
pub fn asin(x: f64) -> f64 {
    libm::asin(x)
}

/// Arccosine, in radians.
pub fn acos(x: f64) -> f64 {
    libm::acos(x)
}

/// Arctangent, in radians.
pub fn atan(x: f64) -> f64 {
    libm::atan(x)
}

/// Arctangent of `y / x`, in radians, using the signs of both to pick the
/// quadrant.
pub fn atan2(y: f64, x: f64) -> f64 {
    libm::atan2(y, x)
}

/// `e` raised to `x`.
pub fn exp(x: f64) -> f64 {
    libm::exp(x)
}

/// Natural logarithm.
pub fn ln(x: f64) -> f64 {
    libm::log(x)
}

/// Base-2 logarithm.
pub fn log2(x: f64) -> f64 {
    libm::log2(x)
}

/// Base-10 logarithm.
pub fn log10(x: f64) -> f64 {
    libm::log10(x)
}

/// `base` raised to a real exponent.
pub fn powf(base: f64, exponent: f64) -> f64 {
    libm::pow(base, exponent)
}

/// The length of the hypotenuse of a right triangle with the given sides,
/// without the intermediate overflow a naive form has.
pub fn hypot(x: f64, y: f64) -> f64 {
    libm::hypot(x, y)
}

/// Cube root.
pub fn cbrt(x: f64) -> f64 {
    libm::cbrt(x)
}

/// `base` raised to an integer power, by squaring.
///
/// `libm` has no integer-exponent form and the platform's carries the same
/// reservation as the rest, so this one is written out. Every step is an IEEE
/// multiplication, which is correctly rounded and therefore identical
/// everywhere, so the result depends on this function's shape and on nothing
/// underneath it. A negative exponent is the reciprocal of the positive case,
/// which is one further correctly rounded operation.
pub fn powi(base: f64, exponent: i32) -> f64 {
    let mut remaining = exponent.unsigned_abs();
    let mut factor = base;
    let mut accumulated = 1.0f64;
    while remaining > 0 {
        if remaining & 1 == 1 {
            accumulated *= factor;
        }
        remaining >>= 1;
        if remaining > 0 {
            factor *= factor;
        }
    }
    if exponent < 0 {
        1.0 / accumulated
    } else {
        accumulated
    }
}

/// Radians per degree, as one correctly rounded constant.
const RADIANS_PER_DEGREE: f64 = core::f64::consts::PI / 180.0;

/// Degrees per radian, as one correctly rounded constant.
const DEGREES_PER_RADIAN: f64 = 180.0 / core::f64::consts::PI;

/// An angle in degrees, as radians.
///
/// The scene format states angles in degrees, because that is what a protractor,
/// a total station and a published figure all use, so this conversion runs on
/// every input this tool reads.
pub fn to_radians(degrees: f64) -> f64 {
    degrees * RADIANS_PER_DEGREE
}

/// An angle in radians, as degrees.
pub fn to_degrees(radians: f64) -> f64 {
    radians * DEGREES_PER_RADIAN
}

#[cfg(test)]
// Every exact comparison below is one of two shapes: a repeated product against
// the product written out, or a conversion against a constant the standard
// library holds exactly. In both the exactness is what is being tested, and a
// tolerance would pass on the arithmetic having changed, which is the failure
// these exist to catch.
//
// The expectation is on the module rather than on each test, because the
// argument is one argument and four copies of it would drift. What that costs is
// reach: a later test in here that compares two computed values, where a
// tolerance would be right, is covered by this as well and nothing would say so.
// Such a test belongs beside a comparison written with an explicit bound, as
// `the_two_angle_conversions_are_each_others_inverse_to_the_last_places` below
// already is.
#[expect(clippy::float_cmp, reason = "see the paragraphs above this attribute")]
mod tests {
    use super::{powi, to_degrees, to_radians};

    /// The failure this refuses is an exponent loop that is off by one, which
    /// would be invisible against the platform's own version because nothing in
    /// the core calls that one to compare against.
    #[test]
    fn integer_powers_are_the_repeated_product() {
        for base in [0.5f64, 1.5, 2.0, 7.25, -3.0] {
            let mut expected = 1.0f64;
            for exponent in 0..=6 {
                assert_eq!(
                    powi(base, exponent),
                    expected,
                    "{base} to the power {exponent} is not the repeated product"
                );
                expected *= base;
            }
        }
    }

    #[test]
    fn a_negative_exponent_is_the_reciprocal_of_the_positive_one() {
        for base in [0.5f64, 1.5, 2.0, 7.25, -3.0] {
            for exponent in 1..=6 {
                assert_eq!(
                    powi(base, -exponent),
                    1.0 / powi(base, exponent),
                    "{base} to the power -{exponent} is not the reciprocal"
                );
            }
        }
    }

    #[test]
    fn anything_to_the_zeroth_power_is_one() {
        for base in [0.5f64, 1.5, 2.0, 7.25, -3.0, 0.0] {
            assert_eq!(powi(base, 0), 1.0, "{base} to the power 0 is not one");
        }
    }

    /// A sign error here puts an angle on the wrong side of the normal, and the
    /// two conversions being each other's inverse is the cheapest thing that
    /// would notice.
    #[test]
    fn the_two_angle_conversions_are_each_others_inverse_to_the_last_places() {
        for degrees in [-179.0f64, -90.0, -12.5, 0.0, 12.5, 45.0, 90.0, 179.0] {
            let round_trip = to_degrees(to_radians(degrees));
            assert!(
                (round_trip - degrees).abs() <= 1e-12 * degrees.abs().max(1.0),
                "{degrees} degrees came back as {round_trip}"
            );
        }
    }

    #[test]
    fn a_right_angle_is_half_of_pi() {
        assert_eq!(to_radians(90.0), core::f64::consts::FRAC_PI_2);
        assert_eq!(to_degrees(core::f64::consts::FRAC_PI_2), 90.0);
    }
}
