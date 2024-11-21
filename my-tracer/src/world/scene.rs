use cgmath::{vec3, Matrix4, SquareMatrix, Vector3};

use super::{camera::Camera, primitives::{Object, Sphere}, ray::Ray};

pub struct Scene {
    camera : Camera,
    primitives : Vec<Object>,
    pub pixels: Vec<u32>,
    accumulated: i32,
    width: i32,
    height: i32
}

impl Scene{
    pub fn new(width: i32, height:i32) -> Scene{
        Scene{
            camera: Camera::new((height as f32) / (width as f32)),
            primitives: vec![Object::Sphere(Sphere::new(0, Matrix4::identity(), 1.0))],
            pixels: vec![0; 1080 * 720],
            accumulated: 0,
            width: width,
            height: height
        }
    }

    pub fn update(&mut self){
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                let mut primary_ray = self.camera.calculate_primary_ray((x as f32)/(self.width as f32), (y as f32)/(self.height as f32));

                let color = self.ray_color(&mut primary_ray);
                self.pixels[x + y * self.width as usize] = Scene::rgbf32_to_rgb8(color);
            }
        }
    }

    pub fn intersect_ray(&self, ray: &mut Ray) {
        for prim in &self.primitives{
            prim.intersect(ray);
        }
    }

    pub fn ray_color(&self, ray: &mut Ray) -> Vector3<f32>{
        self.intersect_ray(ray);

        if ray.objIdx < 0 { 
            return Vector3::new(0.0, 0.0, 0.0);
        }

        self.primitives[ray.objIdx as usize].get_albedo(ray.origin + ray.dir * ray.dist)
    }

    pub fn rgbf32_to_rgb8(color: Vector3<f32>) -> u32{
        let r: u32 = (255.0 * f32::min(1.0, color.x)) as u32;
        let g: u32 = (255.0 * f32::min(1.0, color.y)) as u32;
        let b: u32 = (255.0 * f32::min(1.0, color.z)) as u32;
        
        return (r << 16) + (g << 8) + b;
    }
}