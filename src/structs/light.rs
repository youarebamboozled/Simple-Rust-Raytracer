#![allow(dead_code)]
use crate::structs::vector3::Vector3;

pub(crate) struct Light {
    pub(crate) position: Vector3,
    pub(crate) intensity: f32,
}
