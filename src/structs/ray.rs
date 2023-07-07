#![allow(dead_code)]
use crate::structs::vector3::Vector3;

pub(crate) struct Ray {
    pub(crate) origin: Vector3,
    pub(crate) direction: Vector3,
}

impl Ray {
    fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction: direction.normalize() }
    }

    pub(crate) fn point_at_parameter(&self, t: f32) -> Vector3 {
        self.origin.add(&self.direction.multiply(t))
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::ray::Ray;
    use crate::structs::vector3::Vector3;

    #[test]
    fn test_point_at_parameter() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0));
        let point = ray.point_at_parameter(2.0);
        assert_eq!(point.x, 2.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 2.0);
    }
}
