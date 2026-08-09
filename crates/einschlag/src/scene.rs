//! The scene: the surfaces the holes sit in, the solid matter a person cannot
//! be inside, the ground, and the bounds of the space that was described.
//!
//! A direction on its own gives a line to infinity. The scene is what turns that
//! into a region, and the extent is what stops the region being a claim nobody
//! made. `docs/decisions/0006-frame-and-units.md` fixes what a scene is: a set
//! of bounded planar surfaces, an optional set of solid obstacles, and an
//! extent, in a right-handed frame with Z up, in metres and radians.
//!
//! **The extent is stated and never inferred from the surfaces.**
//! `docs/decisions/0011-degenerate-cases.md` depends on it: a region that
//! reaches the extent boundary carries a flag saying so, and that flag means
//! nothing if the extent is something the tool made up. The difference between
//! "the shooter was inside this courtyard" and "the shooter was somewhere, and
//! this is the part of it we described" is the difference between a
//! reconstruction and an overstatement, and only the operator can say which one
//! they are making.

use crate::measurement::{Hole, Measured};

/// Why a scene was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// A coordinate or a bound that is not a finite number.
    NotFinite,
    /// A span whose lower bound is above its upper bound.
    SpanInverted,
    /// An identifier that is empty or is only whitespace.
    EmptyIdentifier(&'static str),
    /// Two surfaces, or two obstacles, or two holes, carrying one identifier. A
    /// reference to it would resolve to whichever came first, which is a choice
    /// nobody made.
    DuplicateIdentifier(String),
    /// A hole naming a surface the scene does not carry. Refused here, where the
    /// scene is assembled, rather than at the point of use: a reference that
    /// resolves to nothing fails somewhere further along, in a stack that no
    /// longer says which hole was wrong.
    UnknownSurface {
        /// The hole that made the reference.
        hole: String,
        /// The name it gave.
        surface: String,
    },
    /// An outline with fewer than three vertices. Two points bound no area, so
    /// there is no surface there.
    OutlineNotAPolygon,
    /// A solid with fewer than four faces. Nothing closed can be built out of
    /// three planar faces.
    SolidNotClosable,
    /// A ground level for which nothing was established. The ground is where a
    /// person stands; a scene that does not say where it is has not described
    /// the space.
    GroundNotMeasured,
}

/// A position in the scene frame, in metres.
///
/// Plain numbers rather than measured quantities, which is
/// `docs/decisions/0006-frame-and-units.md` read as it is written: it requires
/// an uncertainty on the orientation of a surface and says nothing about the
/// vertices of its outline. A vertex here is where the operator drew the bound
/// of what they described, not a measurement the answer is derived from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// Along the first axis of the frame.
    pub x: f64,
    /// Along the second.
    pub y: f64,
    /// Along the third, which the frame record fixes as up.
    pub z: f64,
}

impl Point {
    /// A position, or [`Refusal::NotFinite`].
    ///
    /// # Errors
    ///
    /// [`Refusal::NotFinite`] where any coordinate is not a finite number.
    pub fn new(x: f64, y: f64, z: f64) -> Result<Self, Refusal> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return Err(Refusal::NotFinite);
        }
        Ok(Self { x, y, z })
    }
}

/// A closed range along one axis, in metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    low: f64,
    high: f64,
}

impl Span {
    /// A range from `low` to `high`, inclusive.
    ///
    /// # Errors
    ///
    /// [`Refusal::NotFinite`], and [`Refusal::SpanInverted`] where the bounds
    /// are the wrong way round. A zero-width span is legal: an operator
    /// describing a plane rather than a volume is describing something.
    pub fn new(low: f64, high: f64) -> Result<Self, Refusal> {
        if !low.is_finite() || !high.is_finite() {
            return Err(Refusal::NotFinite);
        }
        if low > high {
            return Err(Refusal::SpanInverted);
        }
        Ok(Self { low, high })
    }

    /// The lower bound.
    #[must_use]
    pub fn low(self) -> f64 {
        self.low
    }

    /// The upper bound.
    #[must_use]
    pub fn high(self) -> f64 {
        self.high
    }

    /// Whether `value` lies strictly between the bounds, touching neither.
    fn holds_strictly(self, value: f64) -> bool {
        value > self.low && value < self.high
    }

    /// Whether `value` lies between the bounds or on one of them.
    fn holds(self, value: f64) -> bool {
        value >= self.low && value <= self.high
    }
}

/// The space the operator described, as a box in the scene frame.
///
/// A box because that is the shape `docs/decisions/0007-input-format.md` gives
/// an operator to write, in its `extent` key.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extent {
    /// The range along the first axis.
    pub x: Span,
    /// The range along the second.
    pub y: Span,
    /// The range along the third.
    pub z: Span,
}

impl Extent {
    /// Whether a position is inside the described space, counting the boundary
    /// as inside.
    #[must_use]
    pub fn holds(self, point: Point) -> bool {
        self.x.holds(point.x) && self.y.holds(point.y) && self.z.holds(point.z)
    }

    /// Whether a position is inside and touching no face of the box.
    #[must_use]
    pub fn holds_strictly(self, point: Point) -> bool {
        self.x.holds_strictly(point.x)
            && self.y.holds_strictly(point.y)
            && self.z.holds_strictly(point.z)
    }

    /// How a set of consistent positions sits against this extent.
    ///
    /// The positions are whatever a caller has: a drawn population, the corners
    /// of something, one point. This makes no claim about what produced them.
    #[must_use]
    pub fn reach_of(self, positions: &[Point]) -> Reach {
        Reach {
            reaches_boundary: positions.iter().any(|p| !self.holds_strictly(*p)),
            empty_within: !positions.iter().any(|p| self.holds(*p)),
        }
    }
}

/// What an extent did to a set of consistent positions.
///
/// The two flags `docs/decisions/0011-degenerate-cases.md` names, and they are
/// not exclusive: a set of positions lying entirely outside the described space
/// carries both.
///
/// **This is what stops a region that was cut short being read as a region that
/// ended.** A region ending at the extent boundary looks in a picture exactly
/// like a region that ended because the evidence ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reach {
    reaches_boundary: bool,
    empty_within: bool,
}

impl Reach {
    /// Whether any consistent position lies on the boundary of the described
    /// space or beyond it.
    ///
    /// True means the answer is bounded by the space the operator described
    /// rather than by the evidence, somewhere. It is deliberately the weaker
    /// test of the two available: a position exactly on a face counts, and so
    /// does one outside, so the flag is raised where it is arguable rather than
    /// withheld until it is certain.
    #[must_use]
    pub fn reaches_boundary(self) -> bool {
        self.reaches_boundary
    }

    /// Whether no consistent position lies inside the described space at all.
    ///
    /// Not a contradiction. The constraints agree with each other and place the
    /// shooter somewhere the operator did not describe, which usually means the
    /// described space is too small rather than that the reconstruction failed.
    #[must_use]
    pub fn empty_within_extent(self) -> bool {
        self.empty_within
    }
}

/// What the operator declared about the shape of a surface.
///
/// ANSI/ASB Standard 196 clause 4.3.4 requires this to be documented, which
/// `../../docs/survey/standards.md` establishes by reading the standard, so the
/// operator already has the value. `docs/decisions/0006-frame-and-units.md`
/// decides what the tool does with it: a hole in a surface that is not flat is
/// refused for direction derivation, per hole and not per scene, and nothing
/// here widens an uncertainty to cover curvature because there is no measured
/// figure to widen it by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Contour {
    /// Flat, and a hole in it can be used to derive a direction.
    Flat,
    /// Curved away from the shooter.
    Convex,
    /// Curved towards the shooter.
    Concave,
}

/// A bounded planar surface: a wall, a panel, a door.
#[derive(Debug, Clone, PartialEq)]
pub struct Surface {
    id: String,
    material: String,
    normal_azimuth: Measured,
    normal_elevation: Measured,
    outline: Vec<Point>,
    contour: Contour,
}

impl Surface {
    /// A surface, or the reason it is not one.
    ///
    /// The orientation is the azimuth and elevation of the outward normal, in
    /// radians, each a measured quantity. A wall assumed to be plumb carries an
    /// interval rather than a standard deviation, which is
    /// `docs/decisions/0007-input-format.md`'s example rather than a convention
    /// invented here; nothing in this type requires one form over the other.
    ///
    /// # Errors
    ///
    /// [`Refusal::EmptyIdentifier`] for the identifier or the material, and
    /// [`Refusal::OutlineNotAPolygon`] for an outline of fewer than three
    /// vertices.
    pub fn new(
        id: &str,
        material: &str,
        normal_azimuth: Measured,
        normal_elevation: Measured,
        outline: Vec<Point>,
        contour: Contour,
    ) -> Result<Self, Refusal> {
        if id.trim().is_empty() {
            return Err(Refusal::EmptyIdentifier("surface id"));
        }
        if material.trim().is_empty() {
            return Err(Refusal::EmptyIdentifier("surface material"));
        }
        if outline.len() < 3 {
            return Err(Refusal::OutlineNotAPolygon);
        }
        Ok(Self {
            id: id.to_owned(),
            material: material.to_owned(),
            normal_azimuth,
            normal_elevation,
            outline,
            contour,
        })
    }

    /// What the operator called this surface, and what a hole refers to.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// What it is made of, to be resolved against the material table, #31.
    #[must_use]
    pub fn material(&self) -> &str {
        &self.material
    }

    /// The azimuth of the outward normal, in radians.
    #[must_use]
    pub fn normal_azimuth(&self) -> Measured {
        self.normal_azimuth
    }

    /// The elevation of the outward normal, in radians.
    #[must_use]
    pub fn normal_elevation(&self) -> Measured {
        self.normal_elevation
    }

    /// The bound of the surface on its plane, in metres.
    #[must_use]
    pub fn outline(&self) -> &[Point] {
        &self.outline
    }

    /// The declared contour.
    #[must_use]
    pub fn contour(&self) -> Contour {
        self.contour
    }
}

/// A closed solid the shooter could not have been inside.
///
/// Bounded by planar faces, which is not a new primitive: it is the one
/// `docs/decisions/0006-frame-and-units.md` already fixes a scene out of.
///
/// **Nothing here checks that the faces close.** That is a geometric question
/// about the whole set of faces, the geometry that would answer it is milestone
/// 5, and until then the count is the only thing this can refuse. Whether an
/// obstacle narrowing a region is geometry or a prior, and what the tool has to
/// say when it applies one, is `docs/decisions/0008-priors.md` and not decided
/// here.
#[derive(Debug, Clone, PartialEq)]
pub struct Obstacle {
    id: String,
    faces: Vec<Vec<Point>>,
}

impl Obstacle {
    /// A solid, or the reason it is not one.
    ///
    /// # Errors
    ///
    /// [`Refusal::EmptyIdentifier`], [`Refusal::SolidNotClosable`] for fewer
    /// than four faces, and [`Refusal::OutlineNotAPolygon`] for a face of fewer
    /// than three vertices.
    pub fn new(id: &str, faces: Vec<Vec<Point>>) -> Result<Self, Refusal> {
        if id.trim().is_empty() {
            return Err(Refusal::EmptyIdentifier("obstacle id"));
        }
        if faces.len() < 4 {
            return Err(Refusal::SolidNotClosable);
        }
        if faces.iter().any(|face| face.len() < 3) {
            return Err(Refusal::OutlineNotAPolygon);
        }
        Ok(Self {
            id: id.to_owned(),
            faces,
        })
    }

    /// What the operator called this solid.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The planar faces bounding it.
    #[must_use]
    pub fn faces(&self) -> &[Vec<Point>] {
        &self.faces
    }
}

/// Where the ground is, as a horizontal plane in the scene frame.
///
/// **A horizontal plane is the whole of what this represents, and a sloping
/// site is not expressible.** `docs/decisions/0006-frame-and-units.md` fixes Z
/// as up and says nothing about the ground, so a level is the reading of that
/// record which adds least; a scene on a slope needs a shape no record decides
/// yet and no issue holds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ground {
    level: Measured,
}

impl Ground {
    /// The ground at this height, in metres, along the up axis.
    ///
    /// # Errors
    ///
    /// [`Refusal::GroundNotMeasured`] where nothing was established about it.
    /// An uncertainty on the level is expected and is not refused: a floor read
    /// off a sketch is a measured quantity like any other.
    pub fn at(level: Measured) -> Result<Self, Refusal> {
        if level.value().is_none() {
            return Err(Refusal::GroundNotMeasured);
        }
        Ok(Self { level })
    }

    /// The height of the ground plane, in metres.
    #[must_use]
    pub fn level(self) -> Measured {
        self.level
    }
}

/// One described scene: everything a reconstruction is derived from except the
/// arithmetic.
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    surfaces: Vec<Surface>,
    obstacles: Vec<Obstacle>,
    ground: Ground,
    extent: Extent,
    holes: Vec<Hole>,
}

impl Scene {
    /// A scene, or the reason it is not one.
    ///
    /// # What is refused
    ///
    /// **A hole naming a surface the scene does not carry.** Here, where the
    /// scene is assembled, rather than at the point of use. A reference that
    /// resolves to nothing fails somewhere further along, in a place that no
    /// longer knows which hole was wrong, and on a run that has already
    /// reported half an answer.
    ///
    /// **Two surfaces, two obstacles or two holes under one identifier.** A
    /// reference would resolve to whichever came first, which is a choice
    /// nobody made.
    ///
    /// # What is not refused
    ///
    /// **An extent that does not contain the surfaces.** The extent is what the
    /// operator described and the surfaces are what they measured, and a
    /// surface outside the described space is a statement, not a mistake. It is
    /// also not inferred from them: nothing here widens an extent to fit, which
    /// is the whole reason `docs/decisions/0011-degenerate-cases.md` can put a
    /// flag on a region that reaches its boundary.
    ///
    /// **A scene with no holes**, which is a described space nobody has
    /// measured a perforation in yet.
    ///
    /// **Geometry.** No face is checked for closure, no outline for planarity,
    /// no hole for lying on the surface it names. Those need the geometry of
    /// milestone 5.
    ///
    /// # Errors
    ///
    /// One [`Refusal`] per reason above.
    pub fn new(
        surfaces: Vec<Surface>,
        obstacles: Vec<Obstacle>,
        ground: Ground,
        extent: Extent,
        holes: Vec<Hole>,
    ) -> Result<Self, Refusal> {
        duplicate(surfaces.iter().map(Surface::id))?;
        duplicate(obstacles.iter().map(Obstacle::id))?;
        duplicate(holes.iter().map(Hole::id))?;

        for hole in &holes {
            if !surfaces.iter().any(|s| s.id() == hole.surface()) {
                return Err(Refusal::UnknownSurface {
                    hole: hole.id().to_owned(),
                    surface: hole.surface().to_owned(),
                });
            }
        }

        Ok(Self {
            surfaces,
            obstacles,
            ground,
            extent,
            holes,
        })
    }

    /// The surfaces, in the order they were given.
    #[must_use]
    pub fn surfaces(&self) -> &[Surface] {
        &self.surfaces
    }

    /// The solids a shooter could not have been inside.
    #[must_use]
    pub fn obstacles(&self) -> &[Obstacle] {
        &self.obstacles
    }

    /// Where the ground is.
    #[must_use]
    pub fn ground(&self) -> Ground {
        self.ground
    }

    /// The space the operator described. Exactly what was given, never widened
    /// to fit anything.
    #[must_use]
    pub fn extent(&self) -> Extent {
        self.extent
    }

    /// The holes, every one of which names a surface this scene carries.
    #[must_use]
    pub fn holes(&self) -> &[Hole] {
        &self.holes
    }

    /// The surface a hole names, which is always present.
    #[must_use]
    pub fn surface_of(&self, hole: &Hole) -> Option<&Surface> {
        self.surfaces.iter().find(|s| s.id() == hole.surface())
    }
}

/// Refuses a repeated identifier, naming it.
fn duplicate<'a>(ids: impl Iterator<Item = &'a str>) -> Result<(), Refusal> {
    let mut seen: Vec<&str> = Vec::new();
    for id in ids {
        if seen.contains(&id) {
            return Err(Refusal::DuplicateIdentifier(id.to_owned()));
        }
        seen.push(id);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use core::f64::consts::PI;

    use super::{Contour, Extent, Ground, Obstacle, Point, Refusal, Scene, Span, Surface};
    use crate::measurement::{Centre, Deformation, Hole, Measured, Perforation};

    fn at(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z).expect("a valid position")
    }

    fn span(low: f64, high: f64) -> Span {
        Span::new(low, high).expect("a valid span")
    }

    /// The extent from the worked example in
    /// `docs/decisions/0007-input-format.md`.
    fn an_extent() -> Extent {
        Extent {
            x: span(0.0, 12.0),
            y: span(0.0, 8.0),
            z: span(0.0, 3.0),
        }
    }

    fn a_ground() -> Ground {
        Ground::at(Measured::deviation(0.0, 0.01).expect("a valid level")).expect("a valid ground")
    }

    fn a_surface(id: &str) -> Surface {
        Surface::new(
            id,
            "gypsum-wallboard-12mm",
            // A wall assumed plumb and facing south, as an interval rather than
            // a standard deviation, because nothing was measured to have one.
            Measured::interval(PI, PI - 0.0349, PI + 0.0349).expect("a valid azimuth"),
            Measured::interval(0.0, -0.0349, 0.0349).expect("a valid elevation"),
            vec![
                at(0.0, 0.0, 0.0),
                at(12.0, 0.0, 0.0),
                at(12.0, 0.0, 3.0),
                at(0.0, 0.0, 3.0),
            ],
            Contour::Flat,
        )
        .expect("a valid surface")
    }

    fn a_hole(id: &str, surface: &str) -> Hole {
        let mm =
            |v: f64, sd: f64| Measured::deviation(v / 1000.0, sd / 1000.0).expect("a valid length");
        let coordinate = |v: f64| Measured::deviation(v, 0.005).expect("a valid coordinate");
        Hole::new(
            id,
            surface,
            "gypsum-wallboard-12mm",
            Centre {
                x: coordinate(4.210),
                y: coordinate(0.0),
                z: coordinate(1.480),
            },
            Perforation::new(
                mm(14.8, 0.6),
                mm(9.1, 0.6),
                Measured::deviation(1.0995, 0.0698).expect("a valid bearing"),
            )
            .expect("a valid ellipse"),
            Deformation::Moderate,
        )
        .expect("a valid hole")
    }

    fn a_scene(holes: Vec<Hole>) -> Result<Scene, Refusal> {
        Scene::new(
            vec![a_surface("wall-north")],
            Vec::new(),
            a_ground(),
            an_extent(),
            holes,
        )
    }

    /// The refusal this issue asks for by name, and the reason it is here rather
    /// than where the reference is followed.
    #[test]
    fn a_hole_naming_a_surface_the_scene_does_not_carry_is_refused_when_the_scene_is_built() {
        assert_eq!(
            a_scene(vec![a_hole("A1", "wall-south")]),
            Err(Refusal::UnknownSurface {
                hole: "A1".to_owned(),
                surface: "wall-south".to_owned(),
            })
        );
    }

    #[test]
    fn a_hole_naming_a_surface_the_scene_carries_resolves_to_it() {
        let scene = a_scene(vec![a_hole("A1", "wall-north")]).expect("a valid scene");
        let hole = &scene.holes()[0];
        assert_eq!(scene.surface_of(hole).map(Surface::id), Some("wall-north"));
    }

    #[test]
    fn two_surfaces_under_one_identifier_are_refused() {
        assert_eq!(
            Scene::new(
                vec![a_surface("wall-north"), a_surface("wall-north")],
                Vec::new(),
                a_ground(),
                an_extent(),
                Vec::new(),
            ),
            Err(Refusal::DuplicateIdentifier("wall-north".to_owned()))
        );
    }

    /// The extent is what the operator described. A surface reaching past it is
    /// a statement rather than a mistake, and nothing here widens the extent to
    /// cover it.
    #[test]
    fn the_extent_is_what_was_stated_and_not_what_the_surfaces_span() {
        let reaching_past = Surface::new(
            "wall-long",
            "gypsum-wallboard-12mm",
            Measured::unestablished(),
            Measured::unestablished(),
            vec![at(0.0, 0.0, 0.0), at(40.0, 0.0, 0.0), at(40.0, 0.0, 3.0)],
            Contour::Flat,
        )
        .expect("a valid surface");
        let scene = Scene::new(
            vec![reaching_past],
            Vec::new(),
            a_ground(),
            an_extent(),
            Vec::new(),
        )
        .expect("a valid scene");
        assert_eq!(
            scene.extent(),
            an_extent(),
            "the extent is the one that was stated, not the one the surface spans"
        );
    }

    /// The case `docs/decisions/0011-degenerate-cases.md` calls a region bounded
    /// by the space the operator described rather than by the evidence. What
    /// this asserts is that the answer says so, not that it stops there.
    #[test]
    fn a_region_reaching_the_boundary_says_so_rather_than_ending_there() {
        let extent = an_extent();
        let reaching = [at(6.0, 4.0, 1.5), at(9.0, 6.0, 1.5), at(12.0, 6.0, 1.5)];
        let reach = extent.reach_of(&reaching);
        assert!(
            reach.reaches_boundary(),
            "a region with a position on the boundary is bounded by the described space"
        );
        assert!(
            !reach.empty_within_extent(),
            "and it is not empty within it, because two of its positions are inside"
        );
        assert!(
            extent.holds(reaching[2]),
            "the position on the boundary is still inside the described space, \
             so nothing here is claiming the region was cut away"
        );
    }

    #[test]
    fn a_region_that_touches_nothing_carries_neither_flag() {
        let reach = an_extent().reach_of(&[at(6.0, 4.0, 1.5), at(6.1, 4.1, 1.6)]);
        assert!(!reach.reaches_boundary());
        assert!(!reach.empty_within_extent());
    }

    /// Consistent constraints placing the shooter outside the described space
    /// are a region with two flags, not a contradiction.
    #[test]
    fn a_region_entirely_outside_the_extent_carries_both_flags() {
        let reach = an_extent().reach_of(&[at(30.0, 30.0, 1.5), at(31.0, 30.0, 1.5)]);
        assert!(reach.reaches_boundary());
        assert!(reach.empty_within_extent());
    }

    #[test]
    fn an_empty_set_of_positions_is_empty_within_the_extent_and_reaches_nothing() {
        let reach = an_extent().reach_of(&[]);
        assert!(!reach.reaches_boundary());
        assert!(reach.empty_within_extent());
    }

    #[test]
    fn a_span_the_wrong_way_round_is_refused_and_a_zero_width_one_is_not() {
        assert_eq!(Span::new(3.0, 1.0), Err(Refusal::SpanInverted));
        assert!(Span::new(2.0, 2.0).is_ok());
        assert_eq!(Span::new(0.0, f64::NAN), Err(Refusal::NotFinite));
    }

    #[test]
    fn an_outline_that_bounds_no_area_is_refused() {
        assert_eq!(
            Surface::new(
                "wall-north",
                "gypsum-wallboard-12mm",
                Measured::unestablished(),
                Measured::unestablished(),
                vec![at(0.0, 0.0, 0.0), at(1.0, 0.0, 0.0)],
                Contour::Flat,
            ),
            Err(Refusal::OutlineNotAPolygon)
        );
    }

    #[test]
    fn a_solid_with_too_few_faces_is_refused() {
        let face = vec![at(0.0, 0.0, 0.0), at(1.0, 0.0, 0.0), at(0.0, 1.0, 0.0)];
        assert_eq!(
            Obstacle::new("pillar", vec![face.clone(), face.clone(), face]),
            Err(Refusal::SolidNotClosable)
        );
        assert!(
            Obstacle::new(
                "pillar",
                vec![
                    vec![at(0.0, 0.0, 0.0), at(1.0, 0.0, 0.0), at(0.0, 1.0, 0.0)],
                    vec![at(0.0, 0.0, 0.0), at(1.0, 0.0, 0.0), at(0.0, 0.0, 1.0)],
                    vec![at(0.0, 0.0, 0.0), at(0.0, 1.0, 0.0), at(0.0, 0.0, 1.0)],
                    vec![at(1.0, 0.0, 0.0), at(0.0, 1.0, 0.0), at(0.0, 0.0, 1.0)],
                ],
            )
            .is_ok()
        );
    }

    #[test]
    fn a_ground_nobody_established_is_refused() {
        assert_eq!(
            Ground::at(Measured::unestablished()),
            Err(Refusal::GroundNotMeasured)
        );
    }

    #[test]
    fn a_surface_with_no_identifier_or_no_material_is_refused() {
        let build = |id: &str, material: &str| {
            Surface::new(
                id,
                material,
                Measured::unestablished(),
                Measured::unestablished(),
                vec![at(0.0, 0.0, 0.0), at(1.0, 0.0, 0.0), at(0.0, 1.0, 0.0)],
                Contour::Flat,
            )
        };
        assert_eq!(
            build(" ", "gypsum"),
            Err(Refusal::EmptyIdentifier("surface id"))
        );
        assert_eq!(
            build("wall-north", ""),
            Err(Refusal::EmptyIdentifier("surface material"))
        );
    }
}
