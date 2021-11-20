use nalgebra::vector;

use crate::{
    geometry::{bounding_volume::Aabb, faces::Triangle, Shape},
    math::Point,
};

impl Shape for fj::Sweep {
    fn aabb(&self) -> Aabb {
        let mut aabb = self.shape.aabb();
        aabb.max.z = self.length;
        aabb
    }

    fn edge_vertices(&self, _tolerance: f64) -> Vec<Vec<Point>> {
        // TASK: Implement.
        todo!()
    }

    fn triangles(&self, tolerance: f64) -> Vec<Triangle> {
        let mut triangles = Vec::new();

        let original_triangles = self.shape.triangles(tolerance);

        // Bottom face
        triangles.extend(
            original_triangles.iter().map(|triangle| triangle.invert()),
        );

        // Top face
        triangles.extend(original_triangles.iter().map(|triangle| {
            triangle.translate(vector![0.0, 0.0, self.length])
        }));

        let segments = self.shape.edge_segments(tolerance);

        let mut quads = Vec::new();
        for segment in segments {
            let [v0, v1] = segment.0;
            let [v3, v2] = segment.translate(vector![0.0, 0.0, self.length]).0;

            quads.push([v0, v1, v2, v3]);
        }

        for [v0, v1, v2, v3] in quads {
            triangles.push([v0, v1, v2].into());
            triangles.push([v0, v2, v3].into());
        }

        triangles
    }

    fn vertices(&self) -> Vec<Point> {
        // TASK Implement.
        todo!()
    }
}
