use crate::{Shape, Shape2d};

/// A 3-dimensional shape
#[derive(Clone, Debug)]
#[repr(C)]
pub enum Shape3d {
    /// A group of two 3-dimensional shapes
    Group(Box<Group>),

    /// A sweep of 2-dimensional shape along the z-axis
    Sweep(Sweep),

    /// A transformed 3-dimensional shape
    Transform(Box<Transform>),
}

impl From<Shape3d> for Shape {
    fn from(shape: Shape3d) -> Self {
        Self::Shape3d(shape)
    }
}

/// A group of two 3-dimensional shapes
///
/// A group is a collection of disjoint shapes. It is not a union, in that the
/// shapes in the group are not allowed to touch or overlap.
///
/// # Limitations
///
/// Whether the shapes in the group touch or overlap is not currently checked.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Group {
    /// The first of the shapes
    pub a: Shape3d,

    /// The second of the shapes
    pub b: Shape3d,
}

impl From<Group> for Shape {
    fn from(shape: Group) -> Self {
        Self::Shape3d(Shape3d::Group(Box::new(shape)))
    }
}

impl From<Group> for Shape3d {
    fn from(shape: Group) -> Self {
        Self::Group(Box::new(shape))
    }
}

/// A transformed 3-dimensional shape
///
/// # Limitations
///
/// Transformations are currently limited to a rotation, followed by a
/// translation.
///
/// See issue:
/// <https://github.com/hannobraun/Fornjot/issues/101>
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Transform {
    /// The shape being rotated
    pub shape: Shape3d,

    /// The axis of the rotation
    pub axis: [f64; 3],

    /// The angle of the rotation
    pub angle: f64,

    /// The offset of the translation
    pub offset: [f64; 3],
}

impl From<Transform> for Shape {
    fn from(shape: Transform) -> Self {
        Self::Shape3d(Shape3d::Transform(Box::new(shape)))
    }
}

impl From<Transform> for Shape3d {
    fn from(shape: Transform) -> Self {
        Self::Transform(Box::new(shape))
    }
}

/// A sweep of a 2-dimensional shape along the z-axis
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Sweep {
    /// The 2-dimensional shape being swept
    shape: Shape2d,

    /// The length of the sweep
    length: f64,
}

impl Sweep {
    pub fn from_shape_and_length(shape: Shape2d, length: f64) -> Self {
        Self { shape, length }
    }

    pub fn shape(&self) -> &Shape2d {
        &self.shape
    }

    pub fn length(&self) -> f64 {
        self.length
    }

    pub fn color(&self) -> [u8; 4] {
        self.shape().color()
    }
}

impl From<Sweep> for Shape {
    fn from(shape: Sweep) -> Self {
        Self::Shape3d(shape.into())
    }
}

impl From<Sweep> for Shape3d {
    fn from(shape: Sweep) -> Self {
        Self::Sweep(shape)
    }
}
