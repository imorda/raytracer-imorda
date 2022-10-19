use crate::light::LightIntensity;
use crate::objects::RayIntersect;
use crate::vector::Vec3;

pub struct Scene {
    pub cam: Cam,
    pub background_color: Vec3<f64>,
    pub objects: Vec<Box<dyn RayIntersect>>,
    pub lights: Vec<Box<dyn LightIntensity>>,
}

pub struct Cam {
    pub width: usize,
    pub height: usize,
    pub fov: f64,
    pub pos: Vec3<f64>,
    pub dir: Vec3<f64>,
}
