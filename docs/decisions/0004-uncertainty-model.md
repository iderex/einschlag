# 0004. Uncertainty is carried by sampling, and measurement errors are taken as independent with the cost stated

## Status

Accepted.

## Date

2026-08-08.

## The question

This is the decision the project is named for. Everything else can be rebuilt.
Getting this wrong makes the tool worse than string and a laser, because a wrong
distribution carries the authority of a computation and a laser does not.

Three things have to be decided together, because deciding any one of them alone
leaves the others to be settled inside an implementation, which is where a
decision stops being visible.

How an uncertainty is carried from the measured inputs to the reported region.
What a stated input uncertainty is taken to mean, given that
`0007-input-format.md` lets an operator write three different things and none of
them names a distribution. And what the model does about measurement errors that
are not independent of one another, which they are not.

## The options considered

**Sampling from the measured input distributions, carrying samples through the
geometry.** Handles the non-linear steps without needing a closed form, copes with
the near-normal degeneracy where a small measurement error opens the direction
wide, and produces a population that later stages can combine and cut without
re-deriving anything. Cost: run time, and a dependence on a seed, so the run is
reproducible only because `0009-determinism.md` already fixes the generator and
records the seed.

**Analytic propagation of a covariance through a linearisation.** Fast, exact for
small errors, and it produces a covariance matrix that a reader can quote. Cost: it
is wrong in precisely the region that matters most here. The ellipse relation is an
arcsine of a ratio, `../survey/ellipse-accuracy.md` records that its derived angle
becomes arbitrarily sensitive to the measurement as the ratio approaches one, and a
linearisation is a statement that the function is locally straight. It is least
straight where the tool most needs an answer.

**A hybrid: analytic where the linearisation is defensible, sampling elsewhere.**
Fastest in the common case and honest in the hard one. Cost: two code paths, and a
boundary somebody has to justify and then defend against every scene that lands
near it. The boundary would also have to be reported, because a run that took the
analytic path and one that took the sampling path are not the same computation, and
a reader comparing two reconstructions would need to know which they had.

**A closed form for the whole pipeline.** Not seriously in play, and listed so that
the list is not a rationalisation. The pipeline includes occlusion by solid volumes
and intersection with a described extent, which are not differentiable operations,
so there is nothing to write a closed form for.

## The option taken

**Sampling, throughout, with no analytic path.**

Every measured input is turned into a population of draws, the draws are carried
through the geometry, and the reported regions in #44 are constructed from the
resulting population of positions. There is one path and a reader never has to ask
which one a run took.

### What a stated input uncertainty is taken to mean

`0007-input-format.md` fixes three forms and names no distribution for any of them.
This record names them, and where there is a choice it takes the wider reading and
says so here rather than leaving it to the implementation.

**`sd = s` is read as a standard deviation, of a normal distribution with that
standard deviation.**

Reading it literally is itself the wider choice, and this is the part most likely
to be misread. An operator who writes plus or minus 0.5 mm usually means a
tolerance rather than a standard deviation, and a tolerance of that half-width
treated as a rectangular distribution has a standard deviation near 0.29 mm. Taking
the number at face value as a standard deviation therefore gives a wider input than
interpreting it as a tolerance would, and that is the direction this project errs
in.

The normal is truncated only where a value is physically impossible: a length is
not negative, a minor axis does not exceed a major axis. Truncation is not a
modelling choice made for convenience and it is recorded as an applied assumption
under #45 wherever it bit, because a truncation that moved the answer is exactly
the kind of narrowing this project promises to name.

**`interval = [a, b]` is read as uniform over the closed interval.**

Given only two bounds, uniform is the distribution that adds no information the
operator did not supply. Any peaked distribution on the same support asserts that
the middle is more likely, which the operator did not say. The uniform is bounded,
so it is not the widest distribution imaginable on that support, and this record
takes it anyway: the operator asserted the bounds, and widening past them would be
overriding a statement rather than declining to add one.

**`unknown = true` means the tool invents nothing.**

The quantity is treated as unconstrained over its physically possible range, and
the run names it in the output as unconstrained rather than as uncertain. This is
the same treatment `0011-degenerate-cases.md` fixes for the azimuth of a
near-circular perforation, and for the same reason: a wide distribution still has a
mode, and a mode drawn from a quantity nobody established is an artefact that a
reader will read as a finding.

Where a quantity marked unknown is one the reconstruction cannot proceed without,
the run does not substitute a default. It reports what the remaining constraints
give, which may be the unconstrained result `0011-degenerate-cases.md` already
defines.

### What the model does about correlated measurement error

**The first version takes measurement errors as independent, and this record states
what that costs rather than presenting it as a modelling choice with no downside.**

The reason is not that independence is believed. The major and minor axis of one
hole are read with the same instrument by the same person within the same minute,
and treating those two readings as independent is not a description of anything.
The reason is that no correlation figure exists to use.
`../survey/measurement-practice.md` records that no figure was obtained for
operator error on top of any instrument figure, for any quantity, and a correlation
coefficient is a stronger thing than the marginal figures that are already missing.
Inventing one would put a number in the model with nothing behind it, which is the
defect class this project exists against, and it would be invisible because it
would sit inside a covariance matrix.

The cost has two parts and they run in opposite directions.

**Within one hole, independence is very probably the conservative error.** For the
axis ratio, the variance of the log ratio is the sum of the two log variances minus
twice their covariance. A positive covariance, which is what a shared instrument
and a shared observer suggest, reduces that variance. Assuming zero covariance
therefore gives a wider ratio, a wider angle and a wider region than the truth.
Wider in the safe direction is a cost this project accepts. It is stated as very
probably rather than certainly, because a negative correlation is possible, for
instance if an observer who over-reads one axis systematically under-reads the
other, and no measurement here rules it out.

**Across holes, independence is the dangerous assumption, and it is the one this
record is most exposed on.** An operator who reads every hole in a scene with the
same caliper and the same habit carries a component of error that is identical on
every hole. #40 combines several trajectories into one distribution over shooter
positions, and combination narrows. Under independence the narrowing behaves as
though each trajectory were a fresh, unbiased look at the same position. Under a
shared systematic error it does not: every trajectory is displaced together, and
the region shrinks around a point that has moved. That produces a narrow answer
that is wrong, which is the failure this whole project is against, arriving through
the door marked "conservative assumption".

Nothing in this record fixes that. What it does is refuse to let it be discovered
by accident: **#78** is open for the calibration in #50 to be run across a
parameter that gives generated scenes a shared systematic component, so that the
coverage figure is reported as a function of it rather than as one number obtained
where the model's own assumption holds.

## The reasons

**Sampling was taken because the linearisation fails where the tool is needed
most.** A tool that is accurate for small errors on well-conditioned scenes and
wrong on the near-normal and nearly-parallel cases has its accuracy exactly where
an operator does not need help. `0011-degenerate-cases.md` already commits this
project to treating those cases as ordinary rather than exceptional, and an
analytic core would have had to special-case each of them back out.

**The hybrid was rejected on reportability rather than on accuracy.** It would
probably have been accurate. The problem is that a run would have to disclose which
path it took, a reader would have to understand what that meant, and the boundary
would become a thing to argue about in a hearing. One path is worth the run time.

**The three input forms were given the readings above because they are what the
operator actually said.** The pattern across all three is the same: take what was
stated, do not narrow it, do not widen past a stated bound, and where nothing was
stated, say nothing rather than assume something. The tie-break, wherever there was
one, went to the wider reading, and this record says which readings those were so
that the choice can be argued with.

**Independence was taken because the alternative was an invented number.** The
honest options were an unmeasured correlation inside the model or a stated
assumption outside it. The second can be argued with by a reader and measured by
#78. The first cannot be seen at all.

## What this costs

**Run time, and it is not small.** Every reported region rests on a population
large enough for its tail to be stable, and #43 and #44 both depend on that
population. The sample count becomes an input that affects the answer, which
`0009-determinism.md` already anticipates in deciding what happens when the count
is too low for the regions being reported.

**A seed in every run, and a run that cannot be repeated without it.** That cost is
already paid by `0009-determinism.md` and is named here because it is this record
that creates it.

**No covariance matrix to quote.** A reader who wants one, and some will, gets a
population and regions instead. This is a real loss of a familiar summary, and it
is partly the point: a covariance quoted for a strongly non-linear relation is a
number that looks more informative than it is.

**A wider answer than the truth, within each hole.** From the independence
assumption above. This cost falls on the operator, who gets a larger region than a
correlation-aware model would give, and it is accepted.

**An answer that may be too narrow when several trajectories are combined.** From
the same assumption, in the other direction. This cost falls on the reader of the
output, who is the person least able to detect it, and it is the reason #78 exists
rather than a footnote.

**Nothing here is measured.** No sample count, no run time and no coverage figure
appears in this record, because none has been measured. Every number in it is a
number an operator wrote in an input file.

## What would falsify this

**The calibration in #50.** This is the test the issue asked this record to name. A
region stated at a given level should contain the true position at about that rate
over many generated cases. If the stated regions are systematically too narrow, the
propagation is wrong, and since there is only one path the fault is in this record
rather than in a boundary between two of them.

**The same calibration run across the systematic-error parameter, which is #78.**
If coverage collapses as soon as a shared component is introduced, the independence
assumption is not merely imprecise but load-bearing, and this record is superseded
by one that represents correlation, with a measured figure or with a declared range
swept as a sensitivity.

**A measured correlation figure appearing in the literature.** The independence
assumption rests on the absence of a number, so a number retires it.
`../survey/README.md` records that the reading behind that absence is thin and #76
is open on it.

**Run time making the tool unusable on a laptop.** `../survey/field-practice.md`
establishes that the tool has to run on ordinary hardware. If the sample count
needed for a stable tail at the levels operators want cannot be reached there, the
one-path decision is wrong and the hybrid returns, with the boundary reported as
this record said it would have to be.

**The uniform reading of `interval` turning out to be what operators mean by a
photograph estimate.** If an interval from a photograph is in practice a rough
one-sigma rather than a bound, then reading it as a hard bound is narrower than the
operator intended, and the tie-break this record claims to have taken toward the
wider reading went the wrong way for that form.

## Evidence

The near-normal sensitivity that decided against the linearisation is recorded in
`../survey/ellipse-accuracy.md`, which states that the relation takes the arcsine
of the axis ratio so the derived angle becomes arbitrarily sensitive to the
measurement as the ratio approaches one, and that the studies read report their
worst behaviour where the perforation is least elongated. Abstract only, as that
file records.

The absence of any operator-error figure, on which the independence assumption
rests, is recorded in `../survey/measurement-practice.md`.

The relation used above between the variance of a log ratio and the covariance of
its two terms is standard and is not a measurement of anything in this project. It
is stated so the direction of the cost can be checked rather than taken on trust.

No figure in this record was measured. The sample counts, run times and coverage
rates this decision will eventually be judged on do not exist yet, and #50 is where
they are produced.
