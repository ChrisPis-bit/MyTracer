use core::f32;
use cgmath::*;
use image::{ImageBuffer, Rgb};
use num_traits::clamp;
use rayon::prelude::*;

use super::{camera::Camera, math::Math, primitives::{Object, Plane, Sphere}, ray::Ray};

const EPSILON : f32 = 0.0001;

pub struct Scene {
    camera : Camera,
    primitives : Vec<Object>,
    lights: Vec<i32>,
    accumulated: f32,
    width: u32,
    height: u32,
    skybox: ImageBuffer<Rgb<f32>, Vec<f32>>,
    sky_width: f32,
    sky_height: f32,
    aspect: f32
}

impl Scene{
    pub fn new(width: u32, height:u32, skybox_path : &str) -> Scene{
        let mut texture = image::open(&skybox_path).unwrap().into_rgb32f();
        
        let sky_width = texture.width() as f32;
        let sky_height = texture.height() as f32;

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
            primitives: Vec::new(),
            lights: Vec::new(),
            accumulated: 0.0,
            width: width,
            height: height,
            skybox: texture,
            sky_width: sky_width,
            sky_height: sky_height,
            aspect: (width as f32) / (height as f32)
        }
    }

    pub fn add_object(&mut self, mut obj: Object){
        obj.set_idx(self.primitives.len() as i32);
        obj.set_light(false);
        self.primitives.push(obj);
    }

    pub fn add_light(&mut self, mut obj: Object){
        obj.set_idx(self.primitives.len() as i32);
        obj.set_light(true);
        self.primitives.push(obj);
        self.lights.push(obj.idx());
    }

    pub fn build(&mut self){
        self.add_object(Object::Plane(Plane::new(1.0, vec3(0.0, 1.0, 0.0), vec3(0.8, 0.8, 0.8)))); // Ground
        // self.add_object(Object::Plane(Plane::new(5.0, vec3(0.0, -1.0, 0.0), vec3(0.8, 0.8, 0.8))));
        // self.add_object(Object::Plane(Plane::new(5.0, vec3(-1.0, 0.0, 0.0), vec3(0.8, 0.8, 0.8)))); 
        // self.add_object(Object::Plane(Plane::new(5.0, vec3(1.0, 0.0, 0.0), vec3(0.8, 0.8, 0.8)))); 
        // self.add_object(Object::Plane(Plane::new(10.0, vec3(0.0, 0.0, -1.0), vec3(0.8, 0.8, 0.8)))); 
        // self.add_object(Object::Plane(Plane::new(10.0, vec3(0.0, 0.0, 1.0), vec3(0.8, 0.8, 0.8)))); 

        self.add_object(Object::Sphere(Sphere::new(vec3(-2.5, 0.0, 8.0), 1.0, vec3(0.1, 0.75, 0.75))));
        self.add_object(Object::Sphere(Sphere::new(vec3(0.0, 0.0, 8.0), 1.0, vec3(0.75, 0.1, 0.75))));
        self.add_object(Object::Sphere(Sphere::new(vec3(2.5, 0.0, 8.0), 1.0, vec3(0.75, 0.75, 0.1))));


        self.add_light(Object::Sphere(Sphere::new(vec3(-3.8, 2.0, 8.0), 0.5, vec3(15.0, 3.0, 2.0))));
        self.add_light(Object::Sphere(Sphere::new(vec3(3.8, 2.0, 8.0), 0.5, vec3(2.0, 3.0, 15.0))));

    }

    pub fn update(&mut self, delta_time: f32, pixels: &mut Vec<Vector3<f32>>, pixels_rgb8: &mut Vec<u32>){
        if self.camera.update(delta_time, self.aspect) {self.accumulated = 0.0;}


        let base_seed = Math::random_seed_uint();

        let accum = self.accumulated;

        let f_width = self.width as f32;
        let f_height = self.height as f32;

        pixels.par_iter_mut().zip(pixels_rgb8.par_iter_mut()).enumerate().for_each(|(i, (pixel, pixel_rgb8))| {
            let mut seed =  (i as u32).wrapping_add(base_seed).wrapping_mul(17).wrapping_add(1);
            seed = Math::wang_hash(seed);

            let x = (i as f32 % f_width) + Math::random_f32(&mut seed) - 0.5;
            let y = (i as f32 / f_width) + Math::random_f32(&mut seed) - 0.5;

            let mut primary_ray = self.camera.calculate_primary_ray(x / f_width, y / f_height);

            let color = self.ray_color(&mut primary_ray, &mut seed);
            *pixel = (vec3(color.z, color.y, color.x) + *pixel * accum) / (accum + 1.0);
            *pixel_rgb8 = Math::rgbf32_to_rgb8(*pixel);
        });
        self.accumulated += 1.0;
    }

    fn intersect_ray(&self, ray: &mut Ray) {
        for prim in &self.primitives{
            prim.intersect(ray);
        }
    }

    fn ray_color(&self, ray: &mut Ray, seed: &mut u32) -> Vector3<f32>{
        let mut depth = 0;

        let mut T = vec3(1.0,1.0,1.0);
        let mut E = vec3(0.0, 0.0, 0.0);

        while true {
            self.intersect_ray(ray);

            if ray.obj_idx < 0 { 
                E += T.mul_element_wise(self.sample_skybox(ray.dir));
                break;
            }

            // Intersection data
            let primitive = self.primitives[ray.obj_idx as usize];
            let I = ray.origin + ray.dir * ray.dist;
            let albedo = primitive.get_albedo(I);

            // If intersecting with light, simply return light color
            if primitive.is_light() && depth == 0{
                if depth == 0{
                    E += albedo;
                    break;
                }
                else {
                    break;
                }
            }

            let normal = primitive.get_normal(I);
            let R = Math::random_uniform_hemisphere_vectorf32(seed, normal);

            // Light data
            let light = self.sample_random_light(seed);
            let mut L = light.get_random_position(normal, seed) - I;
            let light_normal = light.get_normal(I);
            let light_pdf = light.get_light_pdf();
            let dist_to_light = L.magnitude();
            L = L.normalize();


            let BRDF = albedo * f32::consts::FRAC_1_PI;

            // Check light direction
            let cos_o = light_normal.dot(-L);
            let cos_i = normal.dot(L);

            if cos_o > 0.0 && cos_i > 0.0 {
                // Shadows
                let mut shadow_ray = Ray::new(I + L * EPSILON, L, dist_to_light - EPSILON * 2.0);
                self.intersect_ray(&mut shadow_ray);

                // NEE
                if shadow_ray.obj_idx == -1 {   
                    let light_color = light.get_albedo(I);
                    let light_area = light.get_area();
                    let solid_angle = (light_area * cos_o) / (dist_to_light * dist_to_light);

                    E += T.mul_element_wise(light_color.mul_element_wise(BRDF * solid_angle * cos_i * self.lights.len() as f32 * light_pdf)) ;
                }
            }
   
            // Russian Roulette
            let p = Scene::ray_survival_probability(T);
            if p < Math::random_f32(seed) { break; }

            // Indirect bounces
            *ray = Ray::new(I + R * EPSILON, R, f32::MAX);

            T = T.mul_element_wise(BRDF.mul_element_wise(normal.dot(R) * 2.0 * f32::consts::PI)) / p;

            depth += 1;
        }

        return E;
    }

    fn ray_survival_probability(color: Vector3<f32>) -> f32{
        clamp(f32::max(color.x, f32::max(color.y, color.z)), 0.0, 1.0)
    }

    fn sample_random_light(&self, seed: &mut u32) -> Object {
        let random_index = Math::random_range_u32(seed, 0,self.lights.len() as u32 - 1);
        self.primitives[self.lights[random_index as usize] as usize]
    }

    fn sample_skybox(&self, dir :Vector3<f32>) -> Vector3<f32>{
        let phi = f32::atan2(dir.z, dir.x);
        let u = (self.sky_width * (if phi > 0.0 { phi } else {(phi + 2.0 * f32::consts::PI)}) * (f32::consts::FRAC_1_PI / 2.0) - 0.5) as u32;
        let v = (self.sky_height * f32::acos(dir.y) * f32::consts::FRAC_1_PI - 0.5) as u32;
        //let skyIdx = (u + v * self.skybox.width()) % (self.skybox.width() * self.skybox.height());
        let color = self.skybox[(u, v)];
        vec3(color[0],color[1], color[2])
    }
}