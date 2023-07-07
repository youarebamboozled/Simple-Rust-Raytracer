#![allow(dead_code)]
use crate::structs::vector3::Vector3;

pub(crate) struct Camera {
    pub(crate) position: Vector3,
    pub(crate) direction: Vector3,
    pub(crate) fov: f32,
}
