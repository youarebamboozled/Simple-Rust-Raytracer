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
use minifb::{Key, Window, WindowOptions};


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
    let width = 1024;
    let height = 1024;
    let aspect_ratio = width as f32 / height as f32;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    let mut camera = Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical,
    };
    let scene = Scene {
        objects: vec![Box::new(sphere), Box::new(sphere2), Box::new(sphere3), Box::new(sphere4)],
        lights: vec![light],
    };

    //render_image(&scene, &camera, width, height).save("/Users/mojo/IdeaProjects/Raytracer/rendered/0.png").expect("Couldnt save.");

    render_on_screen(&scene, &mut camera, width, height);
}

pub(crate) fn render_image(scene: &Scene, camera: &Camera, width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
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

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;

pub(crate) fn render_on_screen(scene: &Scene, camera: &mut Camera, width: u32, height: u32){
    let mut buffer: Vec<u32> = vec![0; (width * height) as usize];

    let mut window = Window::new(
        "Render View",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut fov: f32 = 90.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let x = (i % width as usize) as f32;
            let y = (i / width as usize) as f32;

            let u = x / ((width - 1) as f32);
            let v = y / ((height - 1) as f32);
            let ray = camera.get_ray(u, v);

            //set fov of camera
            //let fov_adjustment = (fov.to_radians() / 2.0).tan();
            //let aspect_ratio = (width as f32) / (height as f32);
            //let sensor_x = (((x as f32 + 0.5) / width as f32) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
            //let sensor_y = (1.0 - ((y as f32 + 0.5) / height as f32) * 2.0) * fov_adjustment;
            //camera.horizontal = Vector3::new(sensor_x, 0.0, 0.0);
            //camera.vertical = Vector3::new(0.0, sensor_y, 0.0);
            let color = trace_ray(&ray, scene, camera, 0);
            let scaled_color = color * 255.99;
            *pixel = from_u8_rgb(
                scaled_color.x as u8,
                scaled_color.y as u8,
                scaled_color.z as u8
            );
        }

        if (window.is_key_down(Key::W)) {
            camera.origin = Vector3::new(0.0, 0.0, 0.5) + camera.origin;
        }
        if (window.is_key_down(Key::S)) {
            camera.origin = Vector3::new(0.0, 0.0, -0.5) + camera.origin;
        }
        if (window.is_key_down(Key::A)) {
            camera.origin = Vector3::new(0.5, 0.0, 0.0) + camera.origin;
        }
        if (window.is_key_down(Key::D)) {
            camera.origin = Vector3::new(-0.5, 0.0, 0.0) + camera.origin;
        }
        if (window.is_key_down(Key::Space)) {
            camera.origin = Vector3::new(0.0, 0.5, 0.0) + camera.origin;
        }
        if (window.is_key_down(Key::LeftShift)) {
            camera.origin = Vector3::new(0.0, -0.5, 0.0) + camera.origin;
        }
        if (window.is_key_down(Key::Up)) {
            fov = fov + 1.0;
        }
        if (window.is_key_down(Key::Down)) {
            fov = fov - 1.0;
        }


        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .unwrap();
    }
}
