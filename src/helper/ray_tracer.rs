#![allow(dead_code)]

use crate::structs::camera::Camera;
use crate::structs::intersection::Intersection;
use crate::structs::ray::Ray;
use crate::structs::scene::Scene;
use crate::structs::vector3::Vector3;
use crate::structs::vector4::Vector4;

const MAX_DEPTH: u32 = 100;

pub(crate) fn trace_ray(ray: &Ray, scene: &Scene, camera: &Camera, depth: u32) -> Vector4 {
    let mut color = Vector4::new(0.0, 0.0, 0.0, 0.0);
    let mut min_distance = f32::MAX;
    let mut intersection: Option<Intersection> = None;

    for object in &scene.objects {
        if let Some(i) = object.intersect(ray) {
            if i.distance < min_distance {
                min_distance = i.distance;
                intersection = Some(i);
            }
        }
    }

    if let Some(i) = intersection {
        // Calculate lighting
        for light in &scene.lights {
            let light_direction = (light.position - i.point).normalize();
            let normal = i.normal;

            // Diffuse component
            let diffuse = i.material.albedo * normal.dot(&light_direction).max(0.0);

            // Specular component
            let reflection = reflect(&-light_direction, &normal);
            let view_direction = (camera.origin - i.point).normalize();
            let specular_color = i.material.albedo * i.material.metalness + Vector4::new(1.0, 1.0, 1.0, 1.0) * (1.0 - i.material.metalness);
            let specular = specular_color * i.material.specular * reflection.dot(&view_direction).max(0.0).powf(i.material.shininess / i.material.roughness);
            color += diffuse + specular;
        }

        // Handle transmission
        if i.material.transmission > 0.0 && depth < MAX_DEPTH {
            let refracted_ray = refract(ray, &i.normal, i.material.refractive_index);
            let refracted_color = trace_ray(&refracted_ray, scene, camera, depth + 1);
            color = color * (1.0 - i.material.transmission) + refracted_color * i.material.transmission;
        }
    }

    color
}

fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    *v - 2.0 * v.dot(n) * *n
}

fn refract(ray: &Ray, normal: &Vector3, refractive_index: f32) -> Ray {
    let cos_theta = (-ray.direction).dot(normal).min(1.0);
    let r_out_perp = refractive_index * (ray.direction + cos_theta * *normal);
    let r_out_parallel = -((1.0 - r_out_perp.squared_length()).abs().sqrt()) * *normal;
    let direction = r_out_perp + r_out_parallel;
    Ray::new(ray.origin, direction)
}


