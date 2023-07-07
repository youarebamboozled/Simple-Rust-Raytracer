#![allow(dead_code)]

use crate::structs::ray::Ray;
use crate::structs::vector3::Vector3;

pub(crate) struct Camera {
    pub(crate) origin: Vector3,
    pub(crate) lower_left_corner: Vector3,
    pub(crate) horizontal: Vector3,
    pub(crate) vertical: Vector3,
}

impl Camera {
    pub fn new(origin: Vector3, lower_left_corner: Vector3, horizontal: Vector3, vertical: Vector3) -> Camera {
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}

