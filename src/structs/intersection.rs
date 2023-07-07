use crate::structs::material::Material;
use crate::structs::vector3::Vector3;

pub struct Intersection {
    pub point: Vector3,
    pub normal: Vector3,
    pub material: Material,
}
