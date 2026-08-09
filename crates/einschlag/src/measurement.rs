//! The hole record, and the measured quantity it is built out of.
//!
//! This is the central type of the project, and the shape of it is the argument.
//! A hole that carries a number without its uncertainty is the practice this
//! tool exists to replace, so a measured quantity here cannot be built without
//! one: there is no constructor that takes a bare number, and no field that can
//! be filled in later.
//!
//! `docs/decisions/0007-input-format.md` fixes what an operator may write and
//! `docs/decisions/0004-uncertainty-model.md` fixes what each form is read as.
//! Between them there are exactly three states a measured quantity can be in,
//! and [`Measured`] is those three and nothing else.
//!
//! Every value here is in the internal unit for its quantity, which
//! `docs/decisions/0006-frame-and-units.md` fixes as the metre and the radian.
//! **Nothing in this module refuses a value in the wrong unit**, because a
//! number carries no unit once it is a `f64`; the conversion happens once at the
//! parser boundary, which is #33, and the guard over it is #34.

/// Why a measured quantity or a hole record was refused.
///
/// One variant per refusal rather than a string, so a caller can act on the
/// reason and so that the set of things this module refuses is readable in one
/// place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// A value, a standard deviation or an interval bound that is not a finite
    /// number. An infinity or a NaN arriving here would carry through every
    /// draw made from it and reach the reported region as a value nothing
    /// downstream can reduce.
    NotFinite,
    /// A negative standard deviation. Zero is legal and means the operator is
    /// entitled to state a value with no spread on it, which
    /// `crates/einschlag/src/sampling.rs` already draws.
    NegativeStandardDeviation,
    /// An interval whose lower bound is above its upper bound.
    IntervalInverted,
    /// An interval that does not contain the value it qualifies. The two are
    /// asserted about one quantity, and a value outside its own bounds is a
    /// data entry mistake rather than a wide measurement.
    ValueOutsideItsInterval,
    /// A length that is negative, or an interval on a length reaching below
    /// zero. A perforation does not have a negative axis.
    NegativeLength,
    /// An axis length for which no value was established. `unknown = true` is a
    /// statement that nothing was measured, and a perforation with no measured
    /// axis is not a hole record; the ellipse relation is the ratio of the two
    /// axes and there is nothing here to take a ratio of.
    AxisNotMeasured,
    /// A record whose stated minor axis is longer than its stated major axis.
    /// Refused because it is a data entry mistake that otherwise produces a
    /// wrong answer rather than an error: see [`Hole::new`].
    MinorAxisExceedsMajor,
    /// A hole whose centre has no measured value on one of its coordinates. A
    /// position that was not established is not a position.
    CentreNotMeasured,
    /// An identifier that is empty or is only whitespace. A hole that names no
    /// surface, or a surface named by the empty string, cannot be resolved
    /// against a scene and would fail later, further from the mistake.
    EmptyIdentifier(&'static str),
}

/// What an operator stated about the uncertainty of one quantity.
///
/// The three forms are `docs/decisions/0007-input-format.md`'s, and what each
/// one is read as is `docs/decisions/0004-uncertainty-model.md`'s. They are
/// three different epistemic states rather than three spellings of one, which is
/// the distinction this project is about.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spread {
    /// `sd = s`, read as a normal distribution with that standard deviation,
    /// taken at face value rather than as a tolerance.
    StandardDeviation(f64),
    /// `interval = [a, b]`, read as uniform over the closed interval.
    Interval {
        /// The lower bound, in the internal unit of the quantity.
        low: f64,
        /// The upper bound, in the internal unit of the quantity.
        high: f64,
    },
    /// `unknown = true`. Nothing was established, and no value is asserted
    /// either: a value beside this declaration is refused by the format record,
    /// so there is no number here to be mistaken for a measurement.
    Unestablished,
}

/// One measured quantity: what was read, and what was said about how well.
///
/// **It cannot be built without one of the three forms.** The fields are
/// private and the only ways in are the three constructors below, each of which
/// names its form. There is no constructor taking a bare number, no `Default`,
/// and no setter, so a quantity whose uncertainty was left off does not exist as
/// a value of this type.
///
/// A bare number where one of these is expected does not compile:
///
/// ```compile_fail,E0308
/// use einschlag::measurement::Measured;
/// fn takes(_: Measured) {}
/// takes(14.8);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measured {
    /// `None` exactly when the spread is [`Spread::Unestablished`].
    value: Option<f64>,
    spread: Spread,
}

impl Measured {
    /// A value with a standard deviation on it, in the same unit as the value.
    ///
    /// # Errors
    ///
    /// [`Refusal::NotFinite`] where either number is not finite, and
    /// [`Refusal::NegativeStandardDeviation`] where the deviation is below zero.
    pub fn deviation(value: f64, standard_deviation: f64) -> Result<Self, Refusal> {
        if !value.is_finite() || !standard_deviation.is_finite() {
            return Err(Refusal::NotFinite);
        }
        if standard_deviation < 0.0 {
            return Err(Refusal::NegativeStandardDeviation);
        }
        Ok(Self {
            value: Some(value),
            spread: Spread::StandardDeviation(standard_deviation),
        })
    }

    /// A value known to lie between two bounds, in the same unit as the value.
    ///
    /// # Errors
    ///
    /// [`Refusal::NotFinite`], [`Refusal::IntervalInverted`] where the bounds
    /// are the wrong way round, and [`Refusal::ValueOutsideItsInterval`] where
    /// the value is not between them.
    pub fn interval(value: f64, low: f64, high: f64) -> Result<Self, Refusal> {
        if !value.is_finite() || !low.is_finite() || !high.is_finite() {
            return Err(Refusal::NotFinite);
        }
        if low > high {
            return Err(Refusal::IntervalInverted);
        }
        if value < low || value > high {
            return Err(Refusal::ValueOutsideItsInterval);
        }
        Ok(Self {
            value: Some(value),
            spread: Spread::Interval { low, high },
        })
    }

    /// A quantity for which nothing was established.
    ///
    /// It carries no value, which is `docs/decisions/0007-input-format.md`'s
    /// rule rather than a simplification here: a number stated beside a
    /// declaration that nothing was established asserts two contradictory
    /// things about one quantity, and the format refuses it.
    #[must_use]
    pub fn unestablished() -> Self {
        Self {
            value: None,
            spread: Spread::Unestablished,
        }
    }

    /// The value that was read, or `None` where nothing was established.
    #[must_use]
    pub fn value(self) -> Option<f64> {
        self.value
    }

    /// What was said about the uncertainty.
    #[must_use]
    pub fn spread(self) -> Spread {
        self.spread
    }

    /// Whether this quantity could take a value below zero, reading the spread
    /// as the interval it bounds rather than as its centre.
    ///
    /// A standard deviation is unbounded, so a normal always reaches below zero
    /// somewhere. That is not what this asks: truncation at a physically
    /// impossible value is
    /// `docs/decisions/0004-uncertainty-model.md`'s and happens at the draw.
    /// This asks whether what the operator wrote down was already below zero.
    fn is_negative_as_stated(self) -> bool {
        let below = |x: f64| x < 0.0;
        self.value.is_some_and(below)
            || match self.spread {
                Spread::Interval { low, .. } => below(low),
                Spread::StandardDeviation(_) | Spread::Unestablished => false,
            }
    }
}

/// The centre of a perforation, in the scene frame, in metres.
///
/// Three measured quantities rather than three numbers, because each coordinate
/// is measured and `docs/decisions/0007-input-format.md` carries an uncertainty
/// per coordinate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Centre {
    /// Along the first axis of the scene frame.
    pub x: Measured,
    /// Along the second.
    pub y: Measured,
    /// Along the third, which `docs/decisions/0006-frame-and-units.md` fixes as
    /// up.
    pub z: Measured,
}

/// How deformed a perforation is, as a closed set.
///
/// **Free text was refused here and a closed set is not automatically better.**
/// A grading nobody can apply the same way twice is a field nobody fills in
/// consistently, so each value below says what it is about rather than naming a
/// degree. What they are all about is one thing: whether the edge of the
/// perforation is still the edge of an ellipse, because the ellipse relation is
/// the only thing the axes are read for.
/// `../../docs/survey/measurement-practice.md` records that finding the edge on
/// a torn or deformed hole is the hard part of the measurement, and
/// `../../docs/survey/ellipse-accuracy.md` records one study reporting
/// considerable error from deformation of the material without quantifying it.
///
/// **No study puts a number on any of these boundaries**, and none of the
/// distinctions below is sourced from one. They are a vocabulary for a
/// judgement an operator is already making, written down so that two operators
/// mean the same thing by it, and #76 is the issue on the reading that would
/// give them figures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Deformation {
    /// The edge is clean around the whole perforation and an ellipse can be
    /// fitted to it without a judgement about where the edge is.
    None,
    /// The edge is disturbed somewhere but the ellipse is still determined by
    /// the parts that are clean, so a second operator fitting it would place it
    /// in the same position.
    Slight,
    /// Enough of the edge is torn, petalled or bevelled that where the ellipse
    /// lies is a judgement, and two operators would place it differently.
    Moderate,
    /// The perforation still has a measurable extent, and it is no longer an
    /// ellipse. An axis read off it is a description of the damage rather than
    /// of the projectile's path.
    Severe,
    /// The deformation was not graded. It is not a fourth degree between the
    /// others and it is not a default: it has to be chosen, in the same way and
    /// for the same reason `unknown = true` has to be typed.
    NotAssessed,
}

/// The ellipse a perforation presents on the surface it is in.
///
/// The three quantities the impact angle is derived from, in one type, because
/// the refusal below is about the relation between two of them and not about
/// either one alone.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Perforation {
    major_axis: Measured,
    minor_axis: Measured,
    bearing: Measured,
}

impl Perforation {
    /// An ellipse, or the reason it is not one.
    ///
    /// # What is refused, and why each one is here
    ///
    /// **A minor axis longer than the major axis.** This is the refusal the
    /// issue that asked for this type is built around, and it is not data
    /// hygiene. The impact angle comes out of the arcsine of the ratio of the
    /// minor axis to the major axis, which
    /// `../../docs/survey/ellipse-accuracy.md` records. A swapped pair gives a
    /// ratio above one and an arcsine with no answer, which would surface as a
    /// domain error somewhere further along, away from the mistake. A pair that
    /// was swapped after being mismeasured gives a ratio below one and a
    /// plausible wrong angle, which would surface as nothing at all. Refusing
    /// the ordering catches the first case where the mistake is, and it is the
    /// only place a machine can catch the second.
    ///
    /// **An axis with no measured value.** An explicit declaration that nothing
    /// was established carries no number, and there is no ratio to take.
    ///
    /// **A negative length**, on either axis, as stated or as the lower bound of
    /// an interval.
    ///
    /// # What is not refused
    ///
    /// A bearing with nothing established, which
    /// `docs/decisions/0011-degenerate-cases.md` makes the ordinary case rather
    /// than an error: as a perforation approaches circular its major axis stops
    /// being defined, and the azimuth is carried as unconstrained.
    ///
    /// A value in the wrong unit. Once a quantity is an `f64` it carries no
    /// unit; the conversion is at the parser boundary, #33, and its guard is
    /// #34.
    ///
    /// # Errors
    ///
    /// One [`Refusal`] per reason above.
    pub fn new(
        major_axis: Measured,
        minor_axis: Measured,
        bearing: Measured,
    ) -> Result<Self, Refusal> {
        for axis in [major_axis, minor_axis] {
            if axis.is_negative_as_stated() {
                return Err(Refusal::NegativeLength);
            }
        }
        let (Some(major), Some(minor)) = (major_axis.value(), minor_axis.value()) else {
            return Err(Refusal::AxisNotMeasured);
        };
        if minor > major {
            return Err(Refusal::MinorAxisExceedsMajor);
        }
        Ok(Self {
            major_axis,
            minor_axis,
            bearing,
        })
    }

    /// The longer axis, in metres.
    #[must_use]
    pub fn major_axis(self) -> Measured {
        self.major_axis
    }

    /// The shorter axis, in metres. Never longer than the major axis, which
    /// [`Perforation::new`] refuses.
    #[must_use]
    pub fn minor_axis(self) -> Measured {
        self.minor_axis
    }

    /// Where the major axis points on the surface, in radians. May carry
    /// nothing established, which is the near-circular case.
    #[must_use]
    pub fn bearing(self) -> Measured {
        self.bearing
    }
}

/// One perforation, with its geometry and everything stated about how well it
/// was measured.
///
/// Every measured field is a [`Measured`], so the uncertainty travels in the
/// same structure as the value rather than in a parallel one that can go out of
/// step. The identifiers are plain strings here and are resolved against a
/// scene elsewhere; #32 is the scene and the surface a hole names is checked
/// against it at load time there, not here.
#[derive(Debug, Clone, PartialEq)]
pub struct Hole {
    id: String,
    surface: String,
    material: String,
    centre: Centre,
    perforation: Perforation,
    deformation: Deformation,
}

impl Hole {
    /// A hole record, or the reason it is not one.
    ///
    /// The ellipse arrives already built, so what is refused here is what is
    /// about the record rather than about the geometry. [`Perforation::new`]
    /// holds the axis refusals and says why each is there.
    ///
    /// # What is refused
    ///
    /// **An empty identifier**, on the hole, the surface or the material. A
    /// hole naming no surface cannot be resolved against a scene, and it would
    /// fail later, further from the mistake.
    ///
    /// **A centre with no measured value** on any coordinate. A position that
    /// was not established is not a position.
    ///
    /// # What is not refused here
    ///
    /// A surface this hole names that no scene contains. That comparison needs a
    /// scene, and #32 makes it at load time.
    ///
    /// # Errors
    ///
    /// One [`Refusal`] per reason above.
    pub fn new(
        id: &str,
        surface: &str,
        material: &str,
        centre: Centre,
        perforation: Perforation,
        deformation: Deformation,
    ) -> Result<Self, Refusal> {
        for (name, text) in [("id", id), ("surface", surface), ("material", material)] {
            if text.trim().is_empty() {
                return Err(Refusal::EmptyIdentifier(name));
            }
        }

        for coordinate in [centre.x, centre.y, centre.z] {
            if coordinate.value().is_none() {
                return Err(Refusal::CentreNotMeasured);
            }
        }

        Ok(Self {
            id: id.to_owned(),
            surface: surface.to_owned(),
            material: material.to_owned(),
            centre,
            perforation,
            deformation,
        })
    }

    /// What the operator called this hole.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The surface this hole is in, to be resolved against a scene.
    #[must_use]
    pub fn surface(&self) -> &str {
        &self.surface
    }

    /// The material the surface is made of, to be resolved against the material
    /// table, which is #31.
    #[must_use]
    pub fn material(&self) -> &str {
        &self.material
    }

    /// The centre of the perforation, in the scene frame, in metres.
    #[must_use]
    pub fn centre(&self) -> Centre {
        self.centre
    }

    /// The ellipse this hole presents on its surface.
    #[must_use]
    pub fn perforation(&self) -> Perforation {
        self.perforation
    }

    /// How deformed the perforation is.
    #[must_use]
    pub fn deformation(&self) -> Deformation {
        self.deformation
    }
}

#[cfg(test)]
mod tests {
    use super::{Centre, Deformation, Hole, Measured, Perforation, Refusal, Spread};

    /// A centre that is not the thing under test, so a test about the axes is
    /// not also a test about the position.
    fn a_centre() -> Centre {
        let at = |v: f64| Measured::deviation(v, 0.005).expect("a valid coordinate");
        Centre {
            x: at(4.210),
            y: at(0.0),
            z: at(1.480),
        }
    }

    /// An ellipse from the two axes, with a bearing that is not under test.
    fn an_ellipse(major: Measured, minor: Measured) -> Result<Perforation, Refusal> {
        Perforation::new(
            major,
            minor,
            Measured::deviation(1.0995, 0.0698).expect("a valid bearing"),
        )
    }

    fn a_hole(perforation: Perforation) -> Result<Hole, Refusal> {
        Hole::new(
            "A1",
            "wall-north",
            "gypsum-wallboard-12mm",
            a_centre(),
            perforation,
            Deformation::Moderate,
        )
    }

    fn mm(millimetres: f64, sd_millimetres: f64) -> Measured {
        Measured::deviation(millimetres / 1000.0, sd_millimetres / 1000.0).expect("a valid length")
    }

    /// The refusal this type exists around. A swapped pair is a data entry
    /// mistake that produces a wrong answer rather than an error, and this is
    /// the only place a machine can catch it.
    #[test]
    fn a_minor_axis_longer_than_the_major_axis_is_refused() {
        assert_eq!(
            an_ellipse(mm(9.1, 0.6), mm(14.8, 0.6)),
            Err(Refusal::MinorAxisExceedsMajor)
        );
    }

    #[test]
    fn the_same_two_axes_the_right_way_round_are_accepted() {
        let hole = a_hole(an_ellipse(mm(14.8, 0.6), mm(9.1, 0.6)).expect("a valid ellipse"))
            .expect("a valid hole");
        assert_eq!(hole.perforation().major_axis().value(), Some(0.0148));
        assert_eq!(hole.perforation().minor_axis().value(), Some(0.0091));
    }

    /// A circular perforation is not a swapped one.
    #[test]
    fn two_equal_axes_are_accepted() {
        assert!(an_ellipse(mm(12.0, 0.6), mm(12.0, 0.6)).is_ok());
    }

    #[test]
    fn an_axis_with_nothing_established_is_refused() {
        assert_eq!(
            an_ellipse(Measured::unestablished(), mm(9.1, 0.6)),
            Err(Refusal::AxisNotMeasured)
        );
        assert_eq!(
            an_ellipse(mm(14.8, 0.6), Measured::unestablished()),
            Err(Refusal::AxisNotMeasured)
        );
    }

    #[test]
    fn a_negative_axis_is_refused_as_stated_and_as_a_bound() {
        assert_eq!(
            an_ellipse(mm(-14.8, 0.6), mm(9.1, 0.6)),
            Err(Refusal::NegativeLength)
        );
        let reaching_below_zero = Measured::interval(0.001, -0.002, 0.004).expect("a valid range");
        assert_eq!(
            an_ellipse(mm(14.8, 0.6), reaching_below_zero),
            Err(Refusal::NegativeLength)
        );
    }

    /// `docs/decisions/0011-degenerate-cases.md` carries the azimuth of a
    /// near-circular perforation as unconstrained rather than as an error, so
    /// the one field that may say nothing was established is this one.
    #[test]
    fn a_bearing_with_nothing_established_is_accepted() {
        let ellipse = Perforation::new(mm(15.2, 0.6), mm(9.4, 0.6), Measured::unestablished())
            .expect("a valid ellipse");
        let hole = Hole::new(
            "A2",
            "wall-north",
            "gypsum-wallboard-12mm",
            a_centre(),
            ellipse,
            Deformation::Severe,
        )
        .expect("a valid hole");
        assert_eq!(hole.perforation().bearing().value(), None);
        assert_eq!(hole.perforation().bearing().spread(), Spread::Unestablished);
    }

    #[test]
    fn a_centre_with_nothing_established_is_refused() {
        let centre = Centre {
            x: Measured::unestablished(),
            ..a_centre()
        };
        assert_eq!(
            Hole::new(
                "A1",
                "wall-north",
                "gypsum-wallboard-12mm",
                centre,
                an_ellipse(mm(14.8, 0.6), mm(9.1, 0.6)).expect("a valid ellipse"),
                Deformation::NotAssessed,
            ),
            Err(Refusal::CentreNotMeasured)
        );
    }

    #[test]
    fn an_empty_identifier_is_refused_and_the_refusal_names_the_field() {
        let build = |id: &str, surface: &str, material: &str| {
            Hole::new(
                id,
                surface,
                material,
                a_centre(),
                an_ellipse(mm(14.8, 0.6), mm(9.1, 0.6)).expect("a valid ellipse"),
                Deformation::None,
            )
        };
        assert_eq!(
            build("  ", "wall-north", "gypsum"),
            Err(Refusal::EmptyIdentifier("id"))
        );
        assert_eq!(
            build("A1", "", "gypsum"),
            Err(Refusal::EmptyIdentifier("surface"))
        );
        assert_eq!(
            build("A1", "wall-north", "\t"),
            Err(Refusal::EmptyIdentifier("material"))
        );
    }

    #[test]
    fn a_standard_deviation_below_zero_is_refused_and_zero_is_not() {
        assert_eq!(
            Measured::deviation(1.0, -0.1),
            Err(Refusal::NegativeStandardDeviation)
        );
        assert!(Measured::deviation(1.0, 0.0).is_ok());
    }

    #[test]
    fn an_interval_the_wrong_way_round_or_not_containing_its_value_is_refused() {
        assert_eq!(
            Measured::interval(1.0, 2.0, 0.5),
            Err(Refusal::IntervalInverted)
        );
        assert_eq!(
            Measured::interval(3.0, 0.5, 2.0),
            Err(Refusal::ValueOutsideItsInterval)
        );
        assert!(Measured::interval(1.0, 0.5, 2.0).is_ok());
    }

    #[test]
    fn a_quantity_that_is_not_a_finite_number_is_refused() {
        assert_eq!(Measured::deviation(f64::NAN, 1.0), Err(Refusal::NotFinite));
        assert_eq!(
            Measured::deviation(1.0, f64::INFINITY),
            Err(Refusal::NotFinite)
        );
        assert_eq!(
            Measured::interval(1.0, f64::NEG_INFINITY, 2.0),
            Err(Refusal::NotFinite)
        );
    }

    /// Nothing established carries no value, which is the format record's rule
    /// rather than a simplification here.
    #[test]
    fn nothing_established_carries_no_value() {
        assert_eq!(Measured::unestablished().value(), None);
    }
}
