use crate::material::Material;
use crate::objects::RayIntersect;
use crate::vector::{Cross, Norm, Vec3};
use crate::Normalize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CircularPlane {
    // We define a circular plane by a pinned circle and 2 directional vectors
    center: Vec3<f64>,
    radius: f64,
    material: Material,
    plane_vector_a: Vec3<f64>,
    plane_vector_b: Vec3<f64>,
}

impl CircularPlane {
    pub fn new(
        center: Vec3<f64>,
        radius: f64,
        material: Material,
        plane_vector_a: Vec3<f64>,
        plane_vector_b: Vec3<f64>,
    ) -> Self {
        Self {
            center,
            radius,
            material,
            plane_vector_a,
            plane_vector_b,
        }
    }
}

impl RayIntersect for CircularPlane {
    fn ray_intersect(
        &self,
        ray_origin: Vec3<f64>,
        ray_dir: Vec3<f64>,
    ) -> Option<(f64, Vec3<f64>, Vec3<f64>)> {
        let norm = self.plane_vector_a.cross(self.plane_vector_b).normalize();

        // Plane is defined by ax + by + cz + d = 0
        let a = norm[0];
        let b = norm[1];
        let c = norm[2];
        let d = -a * self.center[0] - b * self.center[1] - c * self.center[2];

        // Ray is defined by (x-x1)/m = (y-y1)/n = (z-z1)/k
        let m = ray_dir[0];
        let n = ray_dir[1];
        let k = ray_dir[2];
        let x1 = ray_origin[0];
        let y1 = ray_origin[1];
        let z1 = ray_origin[2];

        let mut z_denom = b * m * n + a * m * m + c * k * m;
        if z_denom == 0. {
            // Avoiding DBZ
            z_denom = f64::EPSILON;
        }
        let z = (b * m * n * z1 - b * k * m * y1 - a * k * m * x1 + a * m * m * z1 - d * k * m)
            / z_denom; // Calculating an actual intersection (magic 3x3 matrix generalization)
        let y = (k * y1 - n * z1 + n * z) / (if k == 0. { f64::EPSILON } else { k });
        let x = (n * x1 - m * y1 + m * y) / (if n == 0. { f64::EPSILON } else { n });

        let hit_point = Vec3::new(x, y, z);
        if (self.center - hit_point).norm() > self.radius || (hit_point - ray_origin) * ray_dir < 0.
        {
            None
        } else {
            Some((
                (hit_point - ray_origin).norm(),
                hit_point,
                if norm * ray_dir > 0. { -norm } else { norm },
            ))
        }
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}
