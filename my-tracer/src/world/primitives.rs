use core::f32;
use cgmath::*;
use super::{math::Math, ray::Ray};

#[derive(Copy, Clone)]
pub enum Object {
    Sphere(Sphere),
    Cube(Cube),
    Plane(Plane)
}

impl Object{
    pub fn set_light(&mut self, is_light: bool){
        match self {
            Object::Cube(ref mut c) => {},
            Object::Sphere(ref mut s) => s.is_light = is_light,
            Object::Plane(ref mut p) => p.is_light = is_light,
        }
    }

    pub fn is_light(&self) -> bool {
        match self {
            Object::Cube(c) => false,
            Object::Sphere(s) => s.is_light,
            Object::Plane(p) => p.is_light,
        }
    }

    pub fn set_idx(&mut self, idx: i32){
        match self {
            Object::Cube(ref mut c) => c.idx = idx,
            Object::Sphere(ref mut s) => s.idx = idx,
            Object::Plane(ref mut p) => p.idx = idx,
        }
    }

    pub fn idx(&self) -> i32{
        match self {
            Object::Cube(c) => c.idx,
            Object::Sphere(s) => s.idx,
            Object::Plane(p) => p.idx,
        }
    }

    pub fn intersect(&self, ray: &mut Ray){
        match self {
            Object::Cube(c) => c.intersect(ray),
            Object::Sphere(s) => s.intersect(ray),
            Object::Plane(p) => p.intersect(ray),

        }
    }
    
    pub fn get_albedo(&self, pos:Vector3<f32>) -> Vector3<f32> {
        match self {
            Object::Cube(c) => vec3(1.0, 1.0, 0.0),
            Object::Sphere(s) => s.get_albedo(pos),
            Object::Plane(p) => p.get_albedo(pos),
        }
    }

    pub fn get_normal(&self, pos:Vector3<f32>) -> Vector3<f32> {
        match self {
            Object::Cube(c) => vec3(1.0, 1.0, 0.0),
            Object::Sphere(s) => s.get_normal(pos),
            Object::Plane(p) => p.get_normal(pos),
        }
    }

    pub fn get_random_position(&self, normal: Vector3<f32>, seed: &mut u32) -> Vector3<f32>{
        match self {
            Object::Cube(c) => Vector3::zero(),
            Object::Sphere(s) => s.get_random_position(normal, seed),
            Object::Plane(p) => Vector3::zero(),
        }
    }

    pub fn get_area(&self) -> f32{
        match self {
            Object::Cube(c) => 1.0,
            Object::Sphere(s) => s.get_area(),
            Object::Plane(p) => p.get_area(),
        }
    }

    pub fn get_light_pdf(&self) -> f32{
        match self {
            Object::Cube(c) => 1.0,
            Object::Sphere(s) => 0.5, // We only sample hemisphere facing the surface
            Object::Plane(p) =>  1.0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Sphere{
    pub idx: i32,
    position: Vector3<f32>,
    r: f32,
    r2: f32,
    color: Vector3<f32>,
    pub is_light: bool,
}

impl Sphere {
    pub fn new(position: Vector3<f32>, size: f32, color: Vector3<f32>) -> Sphere {
        Sphere {
            idx : 0,
            position : position,
            r : size,
            r2 : size * size,
            color: color,
            is_light: false,
        }
    }

    pub fn intersect(&self, ray: &mut Ray) {
        // let O = Point3::to_vec(self.inv_m.transform_point(ray.origin));
        // let D = self.inv_m.transform_vector(ray.dir);
        let O = ray.origin - self.position;
        let D = ray.dir;

        let b = dot(O, D);
        let c = dot(O, O) - self.r2;

        let mut d = b * b - c;
        if d <= 0.0 { return; }

        d = f32::sqrt(d);
        let mut t = -b - d;

        let mut hit : bool = t < ray.dist && t > 0.0;
        if hit {
            ray.dist = t;
            ray.obj_idx = self.idx;
            return;
        }

        if c > 0.0 { return; }
        t = d - b;
        hit = t < ray.dist && t > 0.0;
        if hit {
            ray.dist = t;
            ray.obj_idx = self.idx;
        }
    }

    pub fn get_normal(&self, p:Vector3<f32>) -> Vector3<f32> {
        (p - self.position) * self.r2
    }

    pub fn get_albedo(&self, p:Vector3<f32>) -> Vector3<f32> {
        self.color
    }

    pub fn get_random_position(&self, normal: Vector3<f32>, seed: &mut u32) -> Vector3<f32>{
        Math::random_uniform_hemisphere_vectorf32(seed, -normal) * self.r + self.position
    }

    pub fn get_area(&self) -> f32{
        self.r2 * f32::consts::PI * 4.0
    }
}

#[derive(Copy, Clone)]
pub struct Cube{
    idx: i32,
    m: Matrix4<f32>,
    inv_m: Matrix4<f32>,
    size: f32,
}

impl Cube {
    pub fn new(m: Matrix4<f32>, size: f32) -> Cube {
        Cube {
            idx : 0,
            m : m,
            inv_m : m.invert().unwrap(),
            size : size
        }
    }

    pub fn intersect(&self, ray: &mut Ray){
        // Intersect stuff
    }
}

#[derive(Copy, Clone)]
pub struct Plane{
    idx: i32,
    dist: f32,
    direction: Vector3<f32>,
    color: Vector3<f32>,
    pub is_light: bool,
}

impl Plane{
    pub fn new(dist: f32, direction: Vector3<f32>, color: Vector3<f32>) -> Plane {
        Plane {
            idx : 0,
            dist : dist,
            direction : direction,
            color: color,
            is_light: false
        }
    }

    pub fn intersect(&self, ray: &mut Ray) {
        let t = -(Vector3::dot(ray.origin, self.direction) + self.dist) / dot(ray.dir, self.direction);
        if t < ray.dist && t > 0.0 {
            ray.dist = t;
            ray.obj_idx = self.idx;
        }
    }

    pub fn get_normal(&self, p:Vector3<f32>) -> Vector3<f32> {
        self.direction
    }

    pub fn get_albedo(&self, p:Vector3<f32>) -> Vector3<f32> {
        self.color
    }

    pub fn get_area(&self) -> f32{
        f32::MAX
    }
}