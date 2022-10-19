use crate::material::Material;
use crate::vector::Vec3;

pub mod circular_plane;
pub mod sphere;

pub trait RayIntersect {
    fn ray_intersect(
        &self,
        ray_origin: Vec3<f64>,
        ray_dir: Vec3<f64>,
    ) -> Option<(f64, Vec3<f64>, Vec3<f64>)>;
    fn get_material(&self) -> &Material;
}

pub fn reflect(ray: Vec3<f64>, norm: Vec3<f64>) -> Vec3<f64> {
    ray - norm * 2. * (ray * norm) // ray and norm must be normalized
}

pub fn refract(ray: Vec3<f64>, norm: Vec3<f64>, refraction_index_obj: f64) -> Vec3<f64> {
    let refraction_index_air = 1.0;
    let mut cos_alpha = -(ray * norm).max(-1.).min(1.); // Dot product of unit vectors to get angle between them

    let refraction_ratio;
    let oriented_norm;
    if cos_alpha < 0. {
        // When ray goes out from inside of an object
        cos_alpha = cos_alpha.abs();
        refraction_ratio = refraction_index_obj / refraction_index_air;
        oriented_norm = -norm;
    } else {
        refraction_ratio = refraction_index_air / refraction_index_obj;
        oriented_norm = norm;
    }

    let k = 1. - refraction_ratio * refraction_ratio * (1. - cos_alpha * cos_alpha); // Snell's law magic
    if k < 0. {
        ray // Dummy vector in case of no refraction
    } else {
        ray * refraction_ratio + oriented_norm * (refraction_ratio * cos_alpha - k.sqrt())
        // Even more magic
    }
}
