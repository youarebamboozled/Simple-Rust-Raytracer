use crate::structs::intersection::Intersection;
use crate::structs::material::Material;
use crate::structs::object::Object;
use crate::structs::ray::Ray;
use crate::structs::vector3::Vector3;

pub struct Sphere {
    pub(crate)  center: Vector3,
    pub(crate)  radius: f32,
    pub(crate)  material: Material,
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin.subtract(&self.center);
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp > 0.0 {
                let point = ray.point_at_parameter(temp);
                let normal = point.subtract(&self.center).divide(self.radius);
                let distance = point.subtract(&ray.origin).length();
                return Some(Intersection { point, normal, material: self.material.clone(), distance });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp > 0.0 {
                let point = ray.point_at_parameter(temp);
                let normal = point.subtract(&self.center).divide(self.radius);
                let distance = point.subtract(&ray.origin).length();
                return Some(Intersection { point, normal, material: self.material.clone(), distance });
            }
        }
        None
    }

    fn material(&self) -> &Material {
        &self.material
    }
}