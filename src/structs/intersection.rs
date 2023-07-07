#![allow(dead_code)]
use crate::structs::material::Material;
use crate::structs::vector3::Vector3;

pub struct Intersection {
    pub(crate)  point: Vector3,
    pub(crate)  normal: Vector3,
    pub(crate)  material: Material,
}
