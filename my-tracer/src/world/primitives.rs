use cgmath::*;
use super::ray::Ray;

pub enum Object {
    Sphere(Sphere),
    Cube(Cube)
}

impl Object{
    pub fn idx(&self) -> i32{
        match self {
            Object::Cube(c) => c.idx,
            Object::Sphere(s) => s.idx
        }
    }

    pub fn intersect(&self, ray: &mut Ray){
        match self {
            Object::Cube(c) => c.intersect(ray),
            Object::Sphere(s) => s.intersect(ray),
        }
    }
    
    pub fn get_albedo(&self, p:Point3<f32>) -> Vector3<f32> {
        match self {
            Object::Cube(c) => vec3(1.0, 1.0, 0.0),
            Object::Sphere(s) => s.get_albedo(p)
        }
    }

    pub fn get_normal(&self, p:Point3<f32>) -> Vector3<f32> {
        match self {
            Object::Cube(c) => vec3(1.0, 1.0, 0.0),
            Object::Sphere(s) => s.get_normal(p)
        }
    }
}

pub struct Sphere{
    idx: i32,
    m: Matrix4<f32>,
    inv_m: Matrix4<f32>,
    r: f32,
    r2: f32

}

impl Sphere {
    pub fn new(id: i32, m: Matrix4<f32>, size: f32) -> Sphere {
        Sphere {
            idx : id,
            m : m,
            inv_m : m.invert().unwrap(),
            r : size,
            r2 : size * size,
        }
    }

    pub fn intersect(&self, ray: &mut Ray) {
        let O = Point3::to_vec(self.inv_m.transform_point(ray.origin));
        let D = self.inv_m.transform_vector(ray.dir);

        let b = dot(O, D);
        let c = dot(O, O) - self.r2;

        let mut d = b * b - c;
        if d <= 0.0 { return; }

        d = f32::sqrt(d);
        let mut t = -b - d;

        let mut hit : bool = t < ray.dist && t > 0.0;
        if hit {
            ray.dist = t;
            ray.objIdx = self.idx;
            return;
        }

        if c > 0.0 { return; }
        t = d - b;
        hit = t < ray.dist && t > 0.0;
        if hit {
            ray.dist = t;
            ray.objIdx = self.idx;
        }
    }

    pub fn get_normal(&self, p:Point3<f32>) -> Vector3<f32> {
        Point3::to_vec(self.inv_m.transform_point(p)) * self.r2
    }

    pub fn get_albedo(&self, p:Point3<f32>) -> Vector3<f32> {
        vec3(1.0, 0.0, 0.0)
    }
}

pub struct Cube{
    idx: i32,
    m: Matrix4<f32>,
    inv_m: Matrix4<f32>,
    size: f32
}

impl Cube {
    pub fn new(id: i32, m: Matrix4<f32>, size: f32) -> Cube {
        Cube {
            idx : id,
            m : m,
            inv_m : m.invert().unwrap(),
            size : size
        }
    }

    pub fn intersect(&self, ray: &mut Ray){
        // Intersect stuff
    }
}