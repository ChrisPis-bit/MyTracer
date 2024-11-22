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
        let pos = Point3::new(0.0,0.0,-5.0);
        let target = Vector3::new(0.0,0.0,-1.0);

        let ahead = (target - Point3::to_vec(pos)).normalize();
        let right = (Vector3::cross(vec3(0.0, 1.0, 0.0), ahead));
        let up = vec3(0.0, 1.0, 0.0);

        let t_left = Point3::to_vec(pos) + 2.0 * ahead - aspect * right + up;
        let t_right = Point3::to_vec(pos) + 2.0 * ahead + aspect * right + up;
        let b_left = Point3::to_vec(pos) + 2.0 * ahead - aspect * right - up;

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
        Ray::new(self.position, (p - Point3::to_vec(self.position)).normalize(), f32::max_value())
    }
}