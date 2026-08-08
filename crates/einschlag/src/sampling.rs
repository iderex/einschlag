//! The one place a draw comes from.
//!
//! `docs/decisions/0004-uncertainty-model.md` carries every uncertainty by
//! sampling, so every region this tool reports rests on a population of draws.
//! `docs/decisions/0009-determinism.md` promises that the same input, the same
//! seed and the same build give byte-identical output, and it holds that promise
//! by requiring one explicitly seeded generator whose algorithm is fixed by this
//! project, with reductions in a fixed order.
//! `docs/decisions/0014-the-sampling-generator.md` decides that the arithmetic
//! for it is written here rather than imported.
//!
//! Nothing in this module reads a clock, an environment or a platform entropy
//! source. A [`Generator`] is built from an integer and from nothing else, which
//! is what makes a run repeatable by somebody who disagrees with it.
//! `crates/einschlag/tests/one_seeded_generator.rs` refuses another route into
//! the workspace by name.
//!
//! What is not here: the third input form
//! `docs/decisions/0007-input-format.md` allows, `unknown = true`, and the
//! truncation `0004-uncertainty-model.md` applies where a value is physically
//! impossible. Both attach to a measured quantity rather than to a distribution,
//! the type carrying a measured quantity is #30, and writing either here would
//! be guessing at that type's shape. Issue #37 records what is left.

use crate::math;

/// The seed a run is driven by.
///
/// Written by the operator or generated and recorded, never left implicit, which
/// `docs/decisions/0009-determinism.md` fixes. Nothing here generates one: a
/// value arrives from the caller, so this module has no way to produce a run
/// that cannot be repeated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seed {
    value: u64,
}

impl Seed {
    /// The seed with this value.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// The value, for the manifest to record.
    pub fn value(self) -> u64 {
        self.value
    }
}

/// How many draws a run makes.
///
/// A run parameter rather than a constant, because the count changes what the
/// answer can support. It is not defaulted implicitly: a caller writes
/// [`SampleCount::provisional_default`] and the name says what that number is
/// worth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleCount {
    count: usize,
}

impl SampleCount {
    /// The count to use where the caller has no reason to choose another.
    ///
    /// **This number is not derived from a measurement.** No region is
    /// constructed yet, so nothing has been measured about how many draws a
    /// stable tail at a stated level needs, and until that exists any figure
    /// here is a placeholder. It is named for what it is so that it cannot be
    /// quoted as though somebody had established it.
    ///
    /// What would settle it is the stability check in
    /// `docs/decisions/0009-determinism.md`, which computes per region and per
    /// level how many samples are needed and refuses the region where the run
    /// has fewer. That check belongs with the regions in #44, and it is why a
    /// wrong default here produces a refusal rather than a narrow answer.
    pub fn provisional_default() -> Self {
        Self { count: 10_000 }
    }

    /// A count of `count` draws, refusing zero.
    ///
    /// A run of no samples would reduce to a mean of nothing and a spread of
    /// nothing, and every downstream check would then be comparing empty
    /// populations and passing.
    pub fn new(count: usize) -> Result<Self, Refusal> {
        if count == 0 {
            return Err(Refusal::NoSamples);
        }
        Ok(Self { count })
    }

    /// The number of draws.
    pub fn count(self) -> usize {
        self.count
    }
}

/// The sampling half of a run's configuration: the seed and the count.
///
/// Both are recorded in the manifest `docs/decisions/0009-determinism.md`
/// specifies. That manifest lives inside the output artefact, which is #43 and
/// does not exist, so nothing writes these down yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunSampling {
    seed: Seed,
    samples: SampleCount,
}

impl RunSampling {
    /// The sampling configuration for a run.
    pub fn new(seed: Seed, samples: SampleCount) -> Self {
        Self { seed, samples }
    }

    /// The seed this run is driven by.
    pub fn seed(self) -> Seed {
        self.seed
    }

    /// How many draws this run makes.
    pub fn samples(self) -> SampleCount {
        self.samples
    }
}

/// The generator every draw in this project comes from.
///
/// The state is 256 bits advanced by xor, shift and rotate, and the seed is
/// expanded into it by a multiply, shift and xor mixer. The shape is a published
/// one and the name is not claimed here: no published test vector was compared
/// against, so what the tree depends on is the arithmetic below and the test
/// that pins the sequence it produces.
/// `docs/decisions/0014-the-sampling-generator.md` argues that and says what it
/// costs.
#[derive(Debug, Clone)]
pub struct Generator {
    state: [u64; 4],
}

impl Generator {
    /// The generator this seed drives.
    ///
    /// The mixer runs four times to fill the state. Seeding all four words from
    /// one expander rather than writing the seed into one of them matters for a
    /// small seed: a state that is almost all zeroes takes many steps to leave,
    /// and a run seeded with 1 is exactly the case an operator will write.
    pub fn from_seed(seed: Seed) -> Self {
        let mut mixer = seed.value();
        let mut state = [0_u64; 4];
        for word in &mut state {
            *word = mix(&mut mixer);
        }
        Self { state }
    }

    /// The next value in the sequence.
    pub fn next_u64(&mut self) -> u64 {
        let result = self.state[1]
            .wrapping_mul(5)
            .rotate_left(7)
            .wrapping_mul(9);
        let shifted = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= shifted;
        self.state[3] = self.state[3].rotate_left(45);

        result
    }

    /// The next value in `[0, 1)`.
    ///
    /// The top 53 bits, which is every bit an `f64` can hold without rounding,
    /// scaled by an exact power of two. Both operations are exact, so no value
    /// outside the half-open interval can be produced by rounding.
    pub fn next_unit_interval(&mut self) -> f64 {
        const SCALE: f64 = 1.0 / (1_u64 << 53) as f64;
        (self.next_u64() >> 11) as f64 * SCALE
    }

    /// The next value in `(0, 1)`, for a caller that may not be handed a zero.
    ///
    /// A logarithm of zero is negative infinity and would carry through a whole
    /// population as a value nothing later can reduce. Redrawing rather than
    /// nudging the zero upward keeps the distribution the one it says it is.
    fn next_open_unit_interval(&mut self) -> f64 {
        loop {
            let drawn = self.next_unit_interval();
            if drawn > 0.0 {
                return drawn;
            }
        }
    }
}

/// One step of the seed expander.
fn mix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// What a stated input uncertainty is drawn from.
///
/// `docs/decisions/0004-uncertainty-model.md` fixes the reading of each form an
/// operator may write: `sd = s` is a normal with that standard deviation, taken
/// at face value because that is the wider reading, and `interval = [a, b]` is
/// uniform over the closed interval, because given only two bounds anything
/// peaked asserts something the operator did not say.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Distribution {
    /// A normal distribution. Constructed through [`Distribution::normal`].
    Normal {
        /// The measured value.
        mean: f64,
        /// The stated uncertainty, read as a standard deviation.
        standard_deviation: f64,
    },
    /// Uniform over a closed interval. Constructed through
    /// [`Distribution::uniform`].
    Uniform {
        /// The lower bound the operator stated.
        low: f64,
        /// The upper bound the operator stated.
        high: f64,
    },
}

impl Distribution {
    /// A normal distribution, refusing a standard deviation that is negative or
    /// not a number.
    ///
    /// A standard deviation of zero is allowed and means the operator stated a
    /// value with no uncertainty on it. That is a claim they are entitled to
    /// make and it draws the same number every time.
    pub fn normal(mean: f64, standard_deviation: f64) -> Result<Self, Refusal> {
        if !mean.is_finite() || !standard_deviation.is_finite() {
            return Err(Refusal::NotFinite);
        }
        if standard_deviation < 0.0 {
            return Err(Refusal::NegativeStandardDeviation(standard_deviation));
        }
        Ok(Self::Normal {
            mean,
            standard_deviation,
        })
    }

    /// Uniform over `[low, high]`, refusing bounds the wrong way round.
    ///
    /// Bounds the wrong way round is a transcription mistake rather than a
    /// distribution. Silently swapping them would accept a file nobody meant to
    /// write and report a result as though it had been read correctly.
    pub fn uniform(low: f64, high: f64) -> Result<Self, Refusal> {
        if !low.is_finite() || !high.is_finite() {
            return Err(Refusal::NotFinite);
        }
        if low > high {
            return Err(Refusal::BoundsAreTheWrongWayRound { low, high });
        }
        Ok(Self::Uniform { low, high })
    }

    /// One draw.
    ///
    /// The normal is produced by the polar-coordinate transform of two uniform
    /// draws, which needs a logarithm and a cosine. Both go through
    /// `crate::math`, because
    /// `docs/decisions/0013-platform-math-out-of-the-numeric-core.md` keeps the
    /// platform's own implementations out of the core, and the second value the
    /// transform produces is discarded rather than cached: a cache would make a
    /// draw depend on how many draws preceded it in a way a reader has to hold
    /// in their head, and two uniform draws are cheap.
    pub fn draw(self, generator: &mut Generator) -> f64 {
        match self {
            Self::Normal {
                mean,
                standard_deviation,
            } => {
                if standard_deviation == 0.0 {
                    return mean;
                }
                let first = generator.next_open_unit_interval();
                let second = generator.next_unit_interval();
                let radius = (-2.0 * math::ln(first)).sqrt();
                let angle = 2.0 * std::f64::consts::PI * second;
                mean + standard_deviation * radius * math::cos(angle)
            }
            Self::Uniform { low, high } => low + (high - low) * generator.next_unit_interval(),
        }
    }
}

/// A population of draws, in the order they were drawn.
///
/// The order is part of the value rather than an accident of it. Every reduction
/// below runs over the values in index order in one thread, which is what
/// `docs/decisions/0009-determinism.md` requires: a parallel sum over samples is
/// the most likely way for two runs on one machine to differ, and it differs for
/// a reason that has nothing to do with the geometry.
#[derive(Debug, Clone, PartialEq)]
pub struct Population {
    values: Vec<f64>,
}

impl Population {
    /// Draws a population from one distribution.
    pub fn draw(distribution: Distribution, run: RunSampling) -> Self {
        let mut generator = Generator::from_seed(run.seed());
        let values = (0..run.samples().count())
            .map(|_| distribution.draw(&mut generator))
            .collect();
        Self { values }
    }

    /// The draws, in the order they were made.
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// How many draws there are.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether there are no draws.
    ///
    /// [`SampleCount`] refuses zero, so this is false for any population a run
    /// produced. It is here because a length without it reads as an invitation
    /// to compare against zero by hand.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// The mean, summed in index order.
    pub fn mean(&self) -> f64 {
        let mut total = 0.0;
        for value in &self.values {
            total += value;
        }
        total / self.values.len() as f64
    }

    /// The spread of the draws, over `n - 1`.
    ///
    /// Zero for a population of one, because one draw says nothing about spread
    /// and the alternative is a division by zero arriving in a reported region.
    pub fn standard_deviation(&self) -> f64 {
        if self.values.len() < 2 {
            return 0.0;
        }
        let mean = self.mean();
        let mut total = 0.0;
        for value in &self.values {
            let difference = value - mean;
            total += difference * difference;
        }
        (total / (self.values.len() - 1) as f64).sqrt()
    }

    /// How far the mean of this population is expected to sit from the mean of
    /// the distribution it came from.
    ///
    /// This is the number two runs on different seeds are compared against. A
    /// pair of runs whose means disagree by much more than this reports a
    /// sampling error that is wrong, which is worse than a wide answer because
    /// it is a narrow one that looks earned.
    pub fn standard_error(&self) -> f64 {
        self.standard_deviation() / (self.values.len() as f64).sqrt()
    }

    /// The population as text, one draw per line.
    ///
    /// It exists so that byte identity between two runs can be checked before
    /// there is an output artefact to compare. The artefact is #43 and this is
    /// not it: the format here is the shortest thing that round-trips a value
    /// and it makes no claim to be what a reconstruction will write.
    pub fn as_lines(&self) -> String {
        let mut out = String::new();
        for value in &self.values {
            out.push_str(&format!("{value:?}\n"));
        }
        out
    }
}

/// Why a sampling input was refused.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Refusal {
    /// A run of zero draws.
    NoSamples,
    /// A parameter that was infinite or not a number.
    NotFinite,
    /// A negative standard deviation.
    NegativeStandardDeviation(f64),
    /// An interval whose lower bound is above its upper bound.
    BoundsAreTheWrongWayRound {
        /// The stated lower bound.
        low: f64,
        /// The stated upper bound.
        high: f64,
    },
}

impl Refusal {
    /// What to put in front of the person who ran this.
    ///
    /// A method rather than the standard formatting trait, whose name is on the
    /// refused list in `crates/einschlag/tests/headless_and_unprivileged.rs`.
    pub fn message(self) -> String {
        match self {
            Self::NoSamples => {
                "a run of zero draws has no population to reduce, and every check over \
                 it would compare nothing against nothing"
                    .to_owned()
            }
            Self::NotFinite => {
                "a distribution parameter is infinite or is not a number".to_owned()
            }
            Self::NegativeStandardDeviation(value) => format!(
                "the standard deviation {value} is negative, and an uncertainty below \
                 zero is a transcription mistake rather than a narrow measurement"
            ),
            Self::BoundsAreTheWrongWayRound { low, high } => format!(
                "the interval [{low}, {high}] has its bounds the wrong way round; they \
                 are not swapped here, because a file nobody meant to write would then \
                 be read as one that was correct"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Distribution, Generator, Population, Refusal, RunSampling, SampleCount, Seed};

    fn run(seed: u64, count: usize) -> RunSampling {
        RunSampling::new(
            Seed::new(seed),
            SampleCount::new(count).expect("a positive count"),
        )
    }

    /// The pin. Any change to the state advance or to the seed expander moves
    /// these values, and every region this project ever reports moves with them.
    ///
    /// **These numbers were produced by this implementation and were not
    /// compared against a published test vector.** They fix the sequence rather
    /// than attest to its provenance, which
    /// `docs/decisions/0014-the-sampling-generator.md` states in the same words.
    #[test]
    fn the_sequence_this_seed_produces_is_pinned() {
        let mut generator = Generator::from_seed(Seed::new(1));
        let first: Vec<u64> = (0..4).map(|_| generator.next_u64()).collect();
        assert_eq!(
            first,
            vec![
                12966619160104079557,
                9600361134598540522,
                10590380919521690900,
                7218738570589545383
            ],
            "the draw sequence moved, which moves every region this tool reports. \
             If that was deliberate, docs/decisions/0014-the-sampling-generator.md \
             says what it costs and this pin is updated in the same change."
        );
    }

    /// A small seed is the case an operator will actually write, and a state
    /// that is nearly all zeroes is the case a generator recovers from slowly.
    #[test]
    fn a_seed_of_one_does_not_start_the_state_at_almost_nothing() {
        let generator = Generator::from_seed(Seed::new(1));
        let zero_words = format!("{generator:?}").matches(" 0,").count();
        assert_eq!(
            zero_words, 0,
            "a word of the state is zero after seeding: {generator:?}"
        );
    }

    #[test]
    fn the_same_seed_gives_the_same_population_byte_for_byte() {
        let distribution = Distribution::normal(12.4, 0.5).expect("a valid normal");
        let first = Population::draw(distribution, run(20260809, 500));
        let second = Population::draw(distribution, run(20260809, 500));
        assert_eq!(
            first.as_lines().into_bytes(),
            second.as_lines().into_bytes(),
            "two runs with one seed produced different populations, which is the \
             promise in docs/decisions/0009-determinism.md failing rather than its \
             bound being reached"
        );
    }

    #[test]
    fn two_seeds_disagree_by_no_more_than_the_sampling_error_the_run_reports() {
        let distribution = Distribution::normal(12.4, 0.5).expect("a valid normal");
        let first = Population::draw(distribution, run(1, 20_000));
        let second = Population::draw(distribution, run(2, 20_000));

        let difference = (first.mean() - second.mean()).abs();
        let reported = (first.standard_error() * first.standard_error()
            + second.standard_error() * second.standard_error())
        .sqrt();

        // Four times the reported error on the difference. A pair of runs this
        // far apart is not a wide answer, it is a run whose stated sampling
        // error is wrong, and a caller comparing two reconstructions would be
        // told they disagree when the model does not.
        assert!(
            difference <= 4.0 * reported,
            "two seeds gave means {} and {}, differing by {difference}, against a \
             reported sampling error of {reported} on the difference",
            first.mean(),
            second.mean()
        );
        assert!(
            reported > 0.0,
            "the run reports a sampling error of zero, so the comparison above \
             passed on nothing"
        );
    }

    /// The population has to be near what it was drawn from, or the two checks
    /// above are comparing two equally wrong things and agreeing.
    #[test]
    fn a_drawn_population_sits_where_the_distribution_it_came_from_says() {
        let distribution = Distribution::normal(12.4, 0.5).expect("a valid normal");
        let drawn = Population::draw(distribution, run(7, 20_000));
        assert!(
            (drawn.mean() - 12.4).abs() <= 4.0 * drawn.standard_error(),
            "the mean of the population is {} against a stated 12.4",
            drawn.mean()
        );
        assert!(
            (drawn.standard_deviation() - 0.5).abs() <= 0.05,
            "the spread of the population is {} against a stated 0.5",
            drawn.standard_deviation()
        );
    }

    #[test]
    fn a_uniform_draw_stays_inside_the_bounds_the_operator_stated() {
        let distribution = Distribution::uniform(2.0, 3.0).expect("a valid interval");
        let drawn = Population::draw(distribution, run(11, 5_000));
        for value in drawn.values() {
            assert!(
                (2.0..=3.0).contains(value),
                "a draw of {value} fell outside the stated interval [2, 3]"
            );
        }
        assert!(
            (drawn.mean() - 2.5).abs() <= 4.0 * drawn.standard_error(),
            "the mean of a uniform draw over [2, 3] is {}",
            drawn.mean()
        );
    }

    /// An operator entitled to state a value with no uncertainty on it gets
    /// that value, rather than a spread invented around it.
    #[test]
    fn a_stated_uncertainty_of_zero_draws_the_stated_value() {
        let distribution = Distribution::normal(12.4, 0.0).expect("a valid normal");
        let drawn = Population::draw(distribution, run(3, 100));
        assert!(
            drawn.values().iter().all(|value| *value == 12.4),
            "a draw moved away from a value stated with no uncertainty"
        );
        // Not exactly zero. The mean is a sum in index order divided by the
        // count, and summing one hundred copies of 12.4 lands a few last places
        // away from it, so the spread around that mean is of that size rather
        // than of zero. Measured at 1.79e-15 while writing this.
        assert!(
            drawn.standard_deviation() < 1e-9,
            "the spread of a population of one repeated value is {}, which is              larger than the rounding of its own mean",
            drawn.standard_deviation()
        );
    }

    #[test]
    fn a_negative_uncertainty_is_refused() {
        let refusal = Distribution::normal(12.4, -0.5).expect_err("a negative spread is refused");
        assert!(matches!(refusal, Refusal::NegativeStandardDeviation(_)));
    }

    #[test]
    fn an_interval_the_wrong_way_round_is_refused_rather_than_swapped() {
        let refusal = Distribution::uniform(3.0, 2.0).expect_err("swapped bounds are refused");
        assert!(matches!(
            refusal,
            Refusal::BoundsAreTheWrongWayRound { .. }
        ));
    }

    #[test]
    fn a_run_of_no_samples_is_refused() {
        assert_eq!(
            SampleCount::new(0).expect_err("zero samples is refused"),
            Refusal::NoSamples
        );
    }

    /// The default is a placeholder and the test says so rather than pinning a
    /// number nobody measured. What it holds is that the placeholder is usable.
    #[test]
    fn the_provisional_default_is_a_count_a_run_could_use() {
        assert!(SampleCount::provisional_default().count() > 0);
    }
}
