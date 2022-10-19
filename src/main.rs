use raytracer::material::Material;
use raytracer::scene::{Cam, Scene};
use raytracer::vector::Vec3;
use raytracer::*;

fn main() {
    let ivory = Material {
        diffuse_color: Vec3::new(0.4, 0.4, 0.3),
        diffuse_albedo: 0.6,
        specular_exponent: 50.0,
        specular_albedo: 0.3,
        reflective_albedo: 0.1,
        refractive_index: 1.0,
        refractive_albedo: 0.0,
    };
    let red_rubber = Material {
        diffuse_color: Vec3::new(0.3, 0.1, 0.1),
        diffuse_albedo: 0.9,
        specular_exponent: 10.0,
        specular_albedo: 0.1,
        reflective_albedo: 0.0,
        refractive_index: 1.0,
        refractive_albedo: 0.0,
    };
    let mirror = Material {
        diffuse_color: Vec3::new(1.0, 1.0, 1.0),
        diffuse_albedo: 0.0,
        specular_exponent: 1425.,
        specular_albedo: 10.0,
        reflective_albedo: 0.8,
        refractive_index: 1.0,
        refractive_albedo: 0.0,
    };
    let glass = Material {
        diffuse_color: Vec3::new(0.6, 0.7, 0.8),
        diffuse_albedo: 0.0,
        specular_exponent: 125.,
        specular_albedo: 0.5,
        reflective_albedo: 0.1,
        refractive_index: 1.5,
        refractive_albedo: 0.8,
    };

    let scene = Scene {
        cam: Cam {
            width: 1024,
            height: 768,
            fov: f64::to_radians(60.),
            pos: Vec3::new(0., 0., 0.),
            dir: Vec3::new(0., 0., -1.),
        },
        background_color: Vec3::new(0.2, 0.7, 0.8),
        objects: vec![
            Box::new(objects::sphere::Sphere::new(
                Vec3::new(-3., 0., -16.),
                2.,
                ivory,
            )),
            Box::new(objects::sphere::Sphere::new(
                Vec3::new(-1., -1.5, -12.),
                2.,
                glass,
            )),
            Box::new(objects::sphere::Sphere::new(
                Vec3::new(1.5, -0.5, -18.),
                3.,
                red_rubber,
            )),
            Box::new(objects::sphere::Sphere::new(
                Vec3::new(7., 5., -18.),
                4.,
                mirror,
            )),
            Box::new(objects::circular_plane::CircularPlane::new(
                Vec3::new(1., -4., -15.),
                7.,
                red_rubber,
                Vec3::new(1., 0., 0.),
                Vec3::new(0., 0., 1.),
            )),
        ],
        lights: vec![
            Box::new(light::Point::new(
                Vec3::new(-20., 20., 20.),
                1.5,
                Vec3::new(1., 1., 1.),
            )),
            Box::new(light::Point::new(
                Vec3::new(30., 50., -25.),
                1.8,
                Vec3::new(1., 1., 1.),
            )),
            Box::new(light::Point::new(
                Vec3::new(30., 20., 30.),
                1.7,
                Vec3::new(1., 1., 1.),
            )),
        ],
    };

    let img = render(&scene);
    img.save("output.png").unwrap();
}
