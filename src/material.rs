use crate::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    pub diffuse_color: Vec3<f64>,
    pub diffuse_albedo: f64,
    pub specular_exponent: f64,
    pub specular_albedo: f64,
    pub reflective_albedo: f64,
    pub refractive_index: f64,
    pub refractive_albedo: f64,
}
