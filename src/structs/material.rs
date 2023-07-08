#![allow(dead_code)]
use crate::structs::vector4::Vector4;

/// # Material
/// A material is a collection of properties that define how light interacts with an object.
///
/// This is a very simple material that only has albedo, roughness, and metallic properties.
/// Albedo is the color of the material and is a `Vector4` that represents RGBA values.
/// Roughness is a value between 0 and 1 that represents how rough the material is.
/// Metallic is a value between 0 and 1 that represents how metallic the material is.
///
/// # Examples
///
/// ```
/// let material = Material {
///    albedo: Vector4::new(1.0, 0.0, 0.0, 1.0),
///    roughness: 0.5,
///    metallic: 0.5,
/// };
/// ```
///
/// See also: `Vector4`, `Vector3`, `Vector2`, `Ray`, `Camera`
#[derive(Clone, Debug)]
pub(crate) struct Material {
    /// The color of the material.
    pub(crate) albedo: Vector4,
    /// The roughness of the material.
    pub(crate) roughness: f32,
    /// The metallicness of the material.
    pub(crate) metalness: f32,
    /// The specularness of the material.
    pub(crate) specular: f32,
    /// The shininess of the material.
    pub(crate) shininess: f32,
    /// How much light is refracted through the material.
    pub(crate) transmission: f32,
    /// The refractive index of the material.
    pub(crate) refractive_index: f32,
}
