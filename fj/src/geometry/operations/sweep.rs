use nalgebra::{Point, Vector};

use crate::geometry::attributes::{Distance, SignedDistanceField};

/// A 3-dimensional sweep of a 2-dimensional sketch
pub struct Sweep<Sketch> {
    /// The sketch being swept
    pub sketch: Sketch,

    /// The distance that the sketch is being swept through space
    pub distance: f32,
}

impl<Sketch> Sweep<Sketch> {
    /// Update sketch
    ///
    /// Returns a copy of `self` with the sketch replaced with `sketch`.
    pub fn with_sketch(mut self, sketch: Sketch) -> Self {
        self.sketch = sketch;
        self
    }

    /// Update height
    ///
    /// Returns a copy of `self` with the height replaced with `height`.
    pub fn with_height(mut self, height: f32) -> Self {
        self.distance = height;
        self
    }
}

impl<Sketch> SignedDistanceField<3> for Sweep<Sketch>
where
    Sketch: SignedDistanceField<2>,
{
    fn distance(&self, point: impl Into<Point<f32, 3>>) -> Distance<3> {
        let point = point.into();

        let sample_xy = self.sketch.distance(point.xy());

        let d_xy = sample_xy.distance;
        let d_z = point.z.abs() - self.distance / 2.0;

        let w = Vector::from([f32::max(d_xy, 0.0), f32::max(d_z, 0.0)]);

        let distance = f32::min(f32::max(d_xy, d_z), 0.0) + w.magnitude();

        Distance { point, distance }
    }
}

// `Sweep` is covered by a bunch of unit tests in `cylinder`.
