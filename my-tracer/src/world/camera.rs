use cgmath::*;
use glfw::{Key};
use num_traits::clamp;
use crate::graphics::window::{get_mouse_delta, key_held};

use super::ray::Ray;

pub struct Camera{
    pub position: Vector3<f32>,
    pub top_left: Vector3<f32>, 
    pub top_right: Vector3<f32>, 
    pub bottom_left: Vector3<f32>,
    pitch: f32,
    yaw: f32
}

impl Camera{
    pub fn new(aspect: f32) -> Camera{
        let pos = Vector3::new(0.0,0.0,-5.0);

        let ahead = Vector3::new(0.0,0.0,1.0);
        let right = Vector3::cross(vec3(0.0, 1.0, 0.0), ahead);
        let up = vec3(0.0, 1.0, 0.0);

        let t_left = pos + 2.0 * ahead - aspect * right + up;
        let t_right = pos + 2.0 * ahead + aspect * right + up;
        let b_left = pos + 2.0 * ahead - aspect * right - up;

        Camera{
        position : pos,
        top_left : t_left,
        top_right : t_right,
        bottom_left : b_left,
        pitch: 0.0,
        yaw: 0.0
        }       
    }

    pub fn calculate_primary_ray(&self, x: f32, y: f32) -> Ray {
        let p = self.top_left + x * (self.top_right - self.top_left) + y * (self.bottom_left - self.top_left);
        Ray::new(self.position, (p - self.position).normalize(), f32::max_value())
    }

    pub fn update(&mut self, delta_time: f32, aspect: f32) -> bool {
        let mut changed = false;

        let sensitivity = 0.1;
        let (x, y) = get_mouse_delta();

        if x != 0.0 || y != 0.0 {changed = true;}

        self.yaw += x * sensitivity;
        self.pitch -= y * sensitivity;
        self.pitch = self.pitch.clamp(-89.0, 89.0);


        let ahead = self.get_direction();
        let right = Vector3::normalize(Vector3::cross(Vector3::unit_y(), ahead));
        let up = Vector3::normalize(Vector3::cross(ahead, right));

        let speed = 3.0 * delta_time;

        if  key_held(Key::W){ self.position += speed * ahead; changed = true; }
        if  key_held(Key::A){ self.position -= speed * right; changed = true; }
        if  key_held(Key::S){ self.position -= speed * ahead; changed = true; }
        if  key_held(Key::D){ self.position += speed * right; changed = true; }

        if  key_held(Key::Space){ self.position += speed * Vector3::unit_y(); changed = true; }
        if  key_held(Key::LeftControl){ self.position -= speed * Vector3::unit_y(); changed = true; }

        
        self.top_left = self.position + 2.0 * ahead - aspect * right + up;
        self.top_right = self.position + 2.0 * ahead + aspect * right + up;
        self.bottom_left = self.position + 2.0 * ahead - aspect * right - up;

        changed
    }

    fn get_direction(&self) -> Vector3<f32> {
        let pitch_rad = Rad(self.pitch * std::f32::consts::PI / 180.0);
        let yaw_rad = Rad(self.yaw * std::f32::consts::PI / 180.0);

        let x = pitch_rad.cos() * yaw_rad.sin();
        let y = pitch_rad.sin();
        let z = pitch_rad.cos() * yaw_rad.cos();

        Vector3::new(x, y, z).normalize()
    }
}