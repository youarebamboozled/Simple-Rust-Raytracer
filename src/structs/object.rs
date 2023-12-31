use crate::structs::intersection::Intersection;
use crate::structs::material::Material;
use crate::structs::ray::Ray;

pub(crate) trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn material(&self) -> &Material;
}
