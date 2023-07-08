use image::{ImageBuffer, Rgb};
use crate::helper::ray_tracer::trace_ray;
use crate::structs::material::Material;
use crate::structs::ray::Ray;
use crate::structs::scene::Scene;
use crate::structs::sphere::Sphere;
use crate::structs::vector3::Vector3;
use crate::structs::vector4::Vector4;
use crate::structs::light::Light;
use crate::structs::camera::Camera;

mod structs;
mod helper;

fn main() {
    println!("Hello, world!");
    let sphere = Sphere {
        center: Vector3::new(-3.0, 0.0, -5.0),
        radius: 1.0,
        material: Material {
            albedo: Vector4::new(0.9, 0.9, 0.9, 1.0),
            roughness: 0.0,
            metalness: 1.0,
            specular: 1.0,
            shininess: 100.0,
            refractive_index: 1.0,
            transmission: 0.0,
        }
    };

    let sphere2 = Sphere {
        center: Vector3::new(-1.0, 0.0, -5.0),
        radius: 1.0,
        material: Material {
            albedo: Vector4::new(1.0, 1.0, 0.0, 1.0),
            roughness: 0.0,
            metalness: 1.0,
            specular: 1.0,
            shininess: 1.0,
            refractive_index: 1.0,
            transmission: 0.0,
        }
    };

    let sphere3 = Sphere {
        center: Vector3::new(1.0, 0.0, -5.0),
        radius: 1.0,
        material: Material {
            albedo: Vector4::new(0.4, 0.4, 0.4, 1.0),
            roughness: 0.0,
            metalness: 0.0,
            specular: 0.0,
            shininess: 1000.0,
            refractive_index: 10.0,
            transmission: 1.0,
        }
    };

    let sphere4 = Sphere {
        center: Vector3::new(3.0, 0.0, -5.0),
        radius: 1.0,
        material: Material {
            albedo: Vector4::new(0.1, 0.1, 0.1, 1.0),
            roughness: 0.0,
            metalness: 1.0,
            specular: 1000.0,
            shininess: 100.0,
            refractive_index: 1.0,
            transmission: 0.0,
        }
    };

    let light = Light {
        position: Vector3::new(0.0, 0.0, 0.0),
        intensity: 10.25,
        color: Vector4::new(1.0, 1.0, 1.0, 1.0),
    };
    let width = 1024*2*2*2*2*2*2;
    let height = 1024*2*2*2*2*2*2;
    let aspect_ratio = width as f32 / height as f32;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    let camera = Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical,
    };
    let scene = Scene {
        objects: vec![Box::new(sphere), Box::new(sphere2), Box::new(sphere3), Box::new(sphere4)],
        lights: vec![light],
    };

    render(&scene, &camera, width, height).save("/Users/mojo/IdeaProjects/Raytracer/rendered/0.png").expect("Couldnt save.");

}

pub(crate) fn render(scene: &Scene, camera: &Camera, width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image = ImageBuffer::new(width, height);
    let mut pixels: i128 = 0;
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        pixels += 1;
        let u = (x as f32) / (width as f32 - 1.0);
        let v = (y as f32) / (height as f32 - 1.0);
        let ray = camera.get_ray(u, v);
        let color = trace_ray(&ray, scene, camera, 0);
        *pixel = Rgb([
            (255.99 * color.x) as u8,
            (255.99 * color.y) as u8,
            (255.99 * color.z) as u8,
        ]);
    }
    println!("Rendered {} pixels.", pixels);
    image
}



