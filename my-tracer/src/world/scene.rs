use core::f32;
use std::vec;

use cgmath::*;
use image::{ImageBuffer, Rgb};
use num_traits::zero;
use rand::{rngs::ThreadRng, Rng};

use super::{camera::Camera, light::{Light, PointLight}, math::Math, primitives::{Object, Plane, Sphere}, ray::Ray};

const EPSILON : f32 = 0.0001;

pub struct Scene {
    camera : Camera,
    primitives : Vec<Object>,
    lights: Vec<Light>,
    pub pixels: Vec<u32>,
    accumulated: i32,
    width: i32,
    height: i32,
    skybox: ImageBuffer<Rgb<f32>, Vec<f32>>,
}

impl Scene{
    pub fn new(width: i32, height:i32, skybox_path : &str) -> Scene{
        let mut texture = image::open(&skybox_path).unwrap().into_rgb32f();
        
        // Adjusts HDR values
        for x in 0..texture.width(){
            for y in 0..texture.height(){
                for i in 0..3  {
                    texture[(x,y)][i] = f32::sqrt(texture[(x,y)][i]);
                }
            }
        }
        
        Scene{
            camera: Camera::new((width as f32) / (height as f32)),
            primitives: vec![
                Object::Sphere(Sphere::new(0, Point3::new(0.0, 0.0, 7.0), 1.0)), 
                Object::Plane(Plane::new(1, 1.0, vec3(0.0, 1.0, 0.0)))],
            lights: vec![
                Light::Point(PointLight::new(0, vec3(1000.0, 1000.0, 1000.0), point3(0.0, 30.0, 7.0)))
            ],
            pixels: vec![0; (width * height) as usize],
            accumulated: 0,
            width: width,
            height: height,
            skybox: texture,
        }
    }

    pub fn update(&mut self){
        let base_seed = Math::random_seed_uint();

        for x  in 0..self.width as u32 {
            let mut seed = (x + 1) * 17;
            seed = u32::wrapping_add(base_seed, seed);

            for y in 0..self.height  as u32{
                let mut primary_ray = self.camera.calculate_primary_ray((x as f32)/(self.width as f32), (y as f32)/(self.height as f32));

                let color = self.ray_color(&mut primary_ray, &mut seed, 0);
                self.pixels[(x + y * self.width as u32) as usize] = Scene::rgbf32_to_rgb8((color + Scene::rgb8_to_rgbf32(self.pixels[(x + y * self.width as u32) as usize]) * (self.accumulated as f32)) / (self.accumulated as f32 + 1.0));
                //self.pixels[x + y * self.width as usize] = Scene::rgbf32_to_rgb8(color );
            }
        }

        self.accumulated += 1;
    }

    pub fn intersect_ray(&self, ray: &mut Ray) {
        for prim in &self.primitives{
            prim.intersect(ray);
        }
    }

    pub fn ray_color(&mut self, ray: &mut Ray, seed: &mut u32, depth: u32) -> Vector3<f32>{
        self.intersect_ray(ray);

        if ray.objIdx < 0 { 
            return self.sample_skybox(ray.dir);
        }

        if depth >= 2 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        let I = ray.origin + ray.dir * ray.dist;
        let primitive = &self.primitives[ray.objIdx as usize];
        let albedo = primitive.get_albedo(I);
        let normal = primitive.get_normal(I);
        let R = Math::reflect(ray.dir, normal);

        let light = self.sample_random_light(seed);
        let light_col = light.get_albedo();
        let mut L = light.get_position(seed) - I;
        let dist_to_light = L.magnitude();
        L = L.normalize();

        let mut shadow_ray = Ray::new(I + L * EPSILON, L, dist_to_light - EPSILON * 2.0);
        self.intersect_ray(&mut shadow_ray);
        if shadow_ray.objIdx != -1 {
            return Vector3::zero();
        }
        
        albedo.mul_element_wise((light_col / (dist_to_light * dist_to_light)) * L.dot(normal))
    }

    pub fn rgbf32_to_rgb8(color: Vector3<f32>) -> u32{
        let r: u32 = (255.0 * f32::min(1.0, color.x)) as u32;
        let g: u32 = (255.0 * f32::min(1.0, color.y)) as u32;
        let b: u32 = (255.0 * f32::min(1.0, color.z)) as u32;
        
        return (r << 16) + (g << 8) + b;
    }

    pub fn rgb8_to_rgbf32(color: u32) -> Vector3<f32> {
        let r = ((color >> 16) & 0xFF) as f32 / 255.0; 
        let g = ((color >> 8) & 0xFF) as f32 / 255.0; 
        let b = (color & 0xFF) as f32 / 255.0; 
    
        Vector3::new(r, g, b)
    }

    pub fn sample_random_light(&mut self, seed: &mut u32) -> &Light {
        let random_index = Math::random_range_u32(seed, 0,self.lights.len() as u32 - 1);
        &self.lights[random_index as usize]
    }

    pub fn sample_skybox(&self, dir :Vector3<f32>) -> Vector3<f32>{
        let phi = f32::atan2(dir.z, dir.x);
        let u = (self.skybox.width() as f32 * (if phi > 0.0 { phi } else {(phi + 2.0 * f32::consts::PI)}) * (f32::consts::FRAC_1_PI / 2.0) - 0.5) as u32;
        let v = (self.skybox.height() as f32 * f32::acos(dir.y) * f32::consts::FRAC_1_PI - 0.5) as u32;
        //let skyIdx = (u + v * self.skybox.width()) % (self.skybox.width() * self.skybox.height());
        let color = self.skybox[(u, v)];
        vec3(color[2],color[1], color[0])
    }
}