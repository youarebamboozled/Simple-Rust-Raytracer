#![allow(dead_code)]
use crate::structs::vector3::Vector3;
use crate::structs::vector4::Vector4;

pub(crate) struct Light {
    pub(crate) position: Vector3,
    pub(crate) intensity: f32,
    pub(crate) color: Vector4,
}
