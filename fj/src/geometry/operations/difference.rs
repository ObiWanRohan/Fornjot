use nalgebra::Point;

use crate::geometry::attributes::{Distance, SignedDistanceField};

/// The difference of two bodies
pub struct Difference<A, B> {
    /// The body that is being subtracted from
    pub a: A,

    /// The body that is being subtracted
    pub b: B,
}

impl<A, B, const D: usize> SignedDistanceField<D> for Difference<A, B>
where
    A: SignedDistanceField<D>,
    B: SignedDistanceField<D>,
{
    fn distance(&self, point: impl Into<Point<f32, D>>) -> Distance<D> {
        let point = point.into();

        let dist_a = self.a.distance(point);
        let dist_b = self.b.distance(point);

        let dist_b = Distance {
            distance: -dist_b.distance,
            ..dist_b
        };

        if dist_a.distance > dist_b.distance {
            dist_a
        } else {
            dist_b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{
        attributes::SignedDistanceField as _, shapes::Sphere,
    };

    use super::Difference;

    #[test]
    fn distance() {
        let difference = Difference {
            a: Sphere::new().with_radius(1.0),
            b: Sphere::new().with_radius(0.5),
        };

        assert_eq!(difference.distance([0.0, 0.0, 0.0]).distance, 0.5);
        assert_eq!(difference.distance([0.5, 0.0, 0.0]).distance, 0.0);
        assert_eq!(difference.distance([0.625, 0.0, 0.0]).distance, -0.125);
        assert_eq!(difference.distance([0.75, 0.0, 0.0]).distance, -0.25);
        assert_eq!(difference.distance([0.875, 0.0, 0.0]).distance, -0.125);
        assert_eq!(difference.distance([1.0, 0.0, 0.0]).distance, 0.0);
        assert_eq!(difference.distance([1.5, 0.0, 0.0]).distance, 0.5);
    }
}
