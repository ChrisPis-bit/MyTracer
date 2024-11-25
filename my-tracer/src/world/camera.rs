use cgmath::*;

use super::ray::Ray;

pub struct Camera{
    pub position: Vector3<f32>,
    pub target: Vector3<f32>,
    pub top_left: Vector3<f32>, 
    pub top_right: Vector3<f32>, 
    pub bottom_left: Vector3<f32>,
}

impl Camera{
    pub fn new(aspect: f32) -> Camera{
        let pos = Vector3::new(0.0,0.0,-5.0);
        let target = Vector3::new(0.0,0.0,-1.0);

        let ahead = (target - pos).normalize();
        let right = Vector3::cross(vec3(0.0, 1.0, 0.0), ahead);
        let up = vec3(0.0, 1.0, 0.0);

        let t_left = pos + 2.0 * ahead - aspect * right + up;
        let t_right = pos + 2.0 * ahead + aspect * right + up;
        let b_left = pos + 2.0 * ahead - aspect * right - up;

        Camera{
        position : pos,
        target : target,
        top_left : t_left,
        top_right : t_right,
        bottom_left : b_left
        }       
    }



    pub fn calculate_primary_ray(&self, x: f32, y: f32) -> Ray {
        let p = self.top_left + x * (self.top_right - self.top_left) + y * (self.bottom_left - self.top_left);
        Ray::new(self.position, (p - self.position).normalize(), f32::max_value())
    }
}