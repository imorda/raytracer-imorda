use crate::{intersect_objects, Normalize, Scene, Vec3};
use crate::objects::RayIntersect;
use crate::vector::Norm;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    position: Vec3<f64>,
    intensity: f64,
    color: Vec3<f64>,
}

impl Point {
    pub fn new(position: Vec3<f64>, intensity: f64, color: Vec3<f64>) -> Self {
        Self {
            position,
            intensity,
            color,
        }
    }
}

pub trait LightIntensity {
    fn get_diffuse_light_intensity(&self, point: Vec3<f64>, norm_dir: Vec3<f64>) -> f64;
    fn get_specular_light_intensity(
        &self,
        point: Vec3<f64>,
        norm_dir: Vec3<f64>,
        obj: &dyn RayIntersect,
        ray_dir: Vec3<f64>,
    ) -> f64;

    fn is_in_shadow(&self, point: Vec3<f64>, scene: &Scene) -> bool;

    fn get_light_color(&self) -> Vec3<f64>;
}

impl LightIntensity for Point {
    fn get_diffuse_light_intensity(&self, point: Vec3<f64>, norm: Vec3<f64>) -> f64 {
        let point_to_light = (self.position - point).normalize();

        self.intensity * (point_to_light * norm).max(0.) // Dot product of normalized vectors gives cos of desired angle that represents the final light intensity
    }

    fn get_specular_light_intensity(
        &self,
        point: Vec3<f64>,
        norm_dir: Vec3<f64>,
        obj: &dyn RayIntersect,
        ray_dir: Vec3<f64>,
    ) -> f64 {
        let point_to_light = (self.position - point).normalize();

        f64::powf(
            (crate::objects::reflect(point_to_light, norm_dir) * ray_dir).max(0.),
            obj.get_material().specular_exponent,
        ) * self.intensity
    }

    fn is_in_shadow(&self, point: Vec3<f64>, scene: &Scene) -> bool {
        match intersect_objects(point, (self.position - point).normalize(), scene) {
            None => false,
            Some((_, props)) => props.ray_length < (self.position - point).norm(),
        }
    }

    fn get_light_color(&self) -> Vec3<f64> {
        self.color
    }
}
