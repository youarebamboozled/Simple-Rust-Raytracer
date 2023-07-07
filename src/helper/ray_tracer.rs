#![allow(dead_code)]

use crate::structs::camera::Camera;
use crate::structs::intersection::Intersection;
use crate::structs::ray::Ray;
use crate::structs::scene::Scene;
use crate::structs::vector3::Vector3;
use crate::structs::vector4::Vector4;

pub(crate) fn trace_ray(ray: &Ray, scene: &Scene, camera: &Camera) -> Vector4 {
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
            //let specular = light.intensity * view_direction.dot(&reflection).max(0.0).powf(i.material.shininess);

            color += diffuse;
        }
    }

    color
}

fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    *v - 2.0 * v.dot(n) * *n
}
