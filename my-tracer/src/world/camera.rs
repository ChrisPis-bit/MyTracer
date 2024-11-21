use cgmath::*;

use super::ray::Ray;

pub struct Camera{
    pub position: Point3<f32>,
    pub target: Vector3<f32>,
    pub top_left: Vector3<f32>, 
    pub top_right: Vector3<f32>, 
    pub bottom_left: Vector3<f32>,
}

impl Camera{
    pub fn new(aspect: f32) -> Camera{
        Camera{
        position : Point3::new(0.0,0.0,-5.0),
        target : Vector3::new(0.0,0.0,-1.0),
        top_left : Vector3::new(-aspect, 1.0, 0.0),
        top_right : Vector3::new(aspect, 1.0, 0.0),
        bottom_left : Vector3::new(-aspect, -1.0, 0.0)
        }       
    }



    pub fn calculate_primary_ray(&self, x: f32, y: f32) -> Ray {
        let p = self.top_left + x * (self.top_right - self.top_left) + y * (self.bottom_left - self.top_left);
        Ray::new(self.position, (p - Point3::to_vec(self.position)).normalize(), f32::max_value())
    }
}