use crate::structs::light::Light;
use crate::structs::object::Object;

pub(crate) struct Scene {
    pub(crate) objects: Vec<Box<dyn Object>>,
    pub(crate) lights: Vec<Light>,
}