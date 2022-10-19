use crate::material::Material;
use crate::Normalize;
use crate::objects::RayIntersect;
use crate::vector::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    center: Vec3<f64>,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3<f64>, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl RayIntersect for Sphere {
    fn ray_intersect(
        &self,
        ray_origin: Vec3<f64>,
        ray_dir: Vec3<f64>,
    ) -> Option<(f64, Vec3<f64>, Vec3<f64>)> {
        let cam_to_center = self.center - ray_origin;
        let directed_cam_to_center = cam_to_center * ray_dir; // Creates a right triangle with the cam_to_center vector
        let center_to_ray_distance_squared =
            cam_to_center * cam_to_center - directed_cam_to_center * directed_cam_to_center; // Pythagoras
        if center_to_ray_distance_squared > self.radius * self.radius {
            // Ray missed the sphere
            return None;
        }
        let intersected_ray_part_half_length =
            f64::sqrt(self.radius * self.radius - center_to_ray_distance_squared);
        // Another pythagoras to find length of intersected part of the ray
        let cam_to_intersect_length_in = directed_cam_to_center - intersected_ray_part_half_length;
        let cam_to_intersect_length_out = directed_cam_to_center + intersected_ray_part_half_length;
        let hit_point;
        if cam_to_intersect_length_in > 0. {
            // If origin is outside sphere
            hit_point = ray_origin + ray_dir * cam_to_intersect_length_in;
            Some((
                cam_to_intersect_length_in,
                hit_point,
                (hit_point - self.center).normalize(),
            ))
        } else if cam_to_intersect_length_out > 0. {
            // If origin is inside sphere
            hit_point = ray_origin + ray_dir * cam_to_intersect_length_out;
            Some((
                cam_to_intersect_length_out,
                hit_point,
                (hit_point - self.center).normalize(),
            ))
        } else {
            // If sphere is behind the ray
            None
        }
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}
