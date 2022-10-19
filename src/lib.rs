use image::{ImageBuffer, RgbImage};

use scene::Scene;

use crate::objects::RayIntersect;
use crate::vector::{Normalize, Vec3};

pub mod light;
pub mod material;
pub mod objects;
pub mod scene;
pub mod vector;

const EPS: f64 = 1e-3;
const MAX_DEPTH: usize = 4;

pub fn render(scene: &Scene) -> RgbImage {
    let mut result: RgbImage = ImageBuffer::new(scene.cam.width as u32, scene.cam.height as u32);

    for j in 0..scene.cam.height {
        for i in 0..scene.cam.width {
            let x = (2. * ((i as f64) + 0.5) / (scene.cam.width as f64) - 1.)
                * f64::tan(scene.cam.fov / 2.)
                * (scene.cam.width as f64)
                / (scene.cam.height as f64);
            let y = -(2. * ((j as f64) + 0.5) / (scene.cam.height as f64) - 1.)
                * f64::tan(scene.cam.fov / 2.);

            let ray_dir = (scene.cam.dir + Vec3::new(x, y, 0.)).normalize();

            result.put_pixel(
                i as u32,
                j as u32,
                ray_trace(scene.cam.pos, ray_dir, scene, 0).into(),
            );
        }
    }

    result
}

struct RayHitProperties {
    ray_length: f64,
    hit_point: Vec3<f64>,
    norm_dir: Vec3<f64>,
}

fn intersect_objects(
    origin: Vec3<f64>,
    dir: Vec3<f64>,
    scene: &Scene,
) -> Option<(&dyn RayIntersect, RayHitProperties)> {
    let mut closest_intersection = f64::INFINITY;
    let mut closest_intersection_object = None;
    let mut closest_intersection_hit_point = Vec3::new(0., 0., 0.);
    let mut closest_intersection_norm_dir = Vec3::new(0., 0., 0.);

    for i in &scene.objects {
        if let Some((intersection, hit_point, norm_dir)) = i.ray_intersect(origin, dir) {
            if intersection < closest_intersection {
                closest_intersection = intersection;
                closest_intersection_object = Some(i);
                closest_intersection_hit_point = hit_point;
                closest_intersection_norm_dir = norm_dir;
            }
        }
    }

    closest_intersection_object.map(|obj| {
        (
            obj.as_ref(),
            RayHitProperties {
                ray_length: closest_intersection,
                hit_point: closest_intersection_hit_point,
                norm_dir: closest_intersection_norm_dir,
            },
        )
    })
}

fn ray_trace(origin: Vec3<f64>, dir: Vec3<f64>, scene: &Scene, depth: usize) -> Vec3<f64> {
    if depth > MAX_DEPTH {
        return scene.background_color;
    }

    match intersect_objects(origin, dir, scene) {
        None => scene.background_color,

        Some((obj, props)) => {
            let mut diffuse_light_intensity = 0.;
            let mut specular_light_intensity = 0.;
            let mut total_light_color = Vec3::new(0., 0., 0.);
            for i in &scene.lights {
                if i.is_in_shadow(props.hit_point + props.norm_dir * EPS, scene) {
                    continue;
                }

                diffuse_light_intensity +=
                    i.get_diffuse_light_intensity(props.hit_point, props.norm_dir);
                specular_light_intensity +=
                    i.get_specular_light_intensity(props.hit_point, props.norm_dir, obj, dir);
                total_light_color = total_light_color + i.get_light_color();
            }

            total_light_color = total_light_color * (1. / scene.lights.len() as f64);

            let reflect_color = ray_trace(
                props.hit_point + props.norm_dir * EPS,
                objects::reflect(dir, props.norm_dir).normalize(),
                scene,
                depth + 1,
            );
            let refract_dir =
                objects::refract(dir, props.norm_dir, obj.get_material().refractive_index)
                    .normalize();
            let refract_color = ray_trace(
                if refract_dir * props.norm_dir > 0. {
                    props.hit_point + props.norm_dir * EPS
                } else {
                    props.hit_point - props.norm_dir * EPS
                },
                refract_dir,
                scene,
                depth + 1,
            );

            obj.get_material().diffuse_color
                * diffuse_light_intensity
                * obj.get_material().diffuse_albedo
                + total_light_color * specular_light_intensity * obj.get_material().specular_albedo
                + reflect_color * obj.get_material().reflective_albedo
                + refract_color * obj.get_material().refractive_albedo
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::{Norm, Normalize, Vec3};

    #[test]
    fn test_vec3_add() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let vec2 = Vec3::new(-5.0, 10.0, 0.0);
        assert_eq!(Vec3::new(-4.0, 15.0, 7.0), vec1 + vec2);
    }

    #[test]
    fn test_vec3_sub() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let vec2 = Vec3::new(-5.0, 10.0, 0.0);
        assert_eq!(Vec3::new(6.0, -5.0, 7.0), vec1 - vec2);
    }

    #[test]
    fn test_vec3_dot() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let vec2 = Vec3::new(-5.0, 10.0, 0.0);
        assert_eq!(45.0, vec1 * vec2);
    }

    #[test]
    fn test_vec3_mul_scalar() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let b = -5.0;
        assert_eq!(Vec3::new(-5.0, -25.0, -35.0), vec1 * b);
    }

    #[test]
    fn test_vec3_neg() {
        let vec1 = Vec3::new(1.0, -5.0, 7.0);
        assert_eq!(Vec3::new(-1.0, 5.0, -7.0), -vec1);
    }

    #[test]
    fn test_vec3_index() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        assert_eq!(1.0, vec1[0]);
        assert_eq!(5.0, vec1[1]);
        assert_eq!(7.0, vec1[2]);
    }

    #[test]
    fn test_vec3_norm() {
        let vec1 = Vec3::new(3.0, -4.0, 12.0);
        assert_eq!(13.0, vec1.norm());
    }

    #[test]
    fn test_vec3_normalized() {
        let vec1: Vec3<f64> = Vec3::new(3.0, -4.0, 12.0);
        assert_eq!(Vec3::new(3. / 13., -4. / 13., 12. / 13.), vec1.normalize());
        assert_ne!(1.0, vec1.norm());
        assert_eq!(1.0, vec1.normalize().norm());
    }
}
