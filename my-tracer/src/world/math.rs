use cgmath::*;

pub struct Math;

impl Math{
    pub fn reflect(incident: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
        incident - 2.0 * incident.dot(normal) * normal
    }
}