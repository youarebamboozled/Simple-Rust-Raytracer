#![allow(dead_code)]
use std::fmt::{Debug, Display, Formatter};

#[derive(Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 { x, y, z, w }
    }

    pub fn multiply(&self, scalar: f32) -> Vector4 {
        Vector4::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }

    pub fn divide(&self, scalar: f32) -> Vector4 {
        Vector4::new(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar)
    }

    pub fn elementwise_multiply(&self, other: &Vector4) -> Vector4 {
        Vector4::new(self.x * other.x, self.y * other.y, self.z * other.z, self.w * other.w)
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Vector4 {
        let magnitude = self.magnitude();
        Vector4::new(self.x / magnitude, self.y / magnitude, self.z / magnitude, self.w / magnitude)
    }

    pub fn add(&self, other: &Vector4) -> Vector4 {
        Vector4::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }

    pub fn subtract(&self, other: &Vector4) -> Vector4 {
        Vector4::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }

    pub fn dot(&self, other: &Vector4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl Debug for Vector4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x: {:?}, y: {:?}, z: {:?}, w: {:?}}}", self.x, self.y, self.z, self.w)
    }
}

impl Display for Vector4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x: {}, y: {}, z: {}, w: {}}}", self.x, self.y, self.z, self.w)
    }
}