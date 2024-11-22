use cgmath::*;
use super::ray::Ray;

pub enum Object {
    Sphere(Sphere),
    Cube(Cube),
    Plane(Plane)
}

impl Object{
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
    
    pub fn get_albedo(&self, pos:Point3<f32>) -> Vector3<f32> {
        match self {
            Object::Cube(c) => vec3(1.0, 1.0, 0.0),
            Object::Sphere(s) => s.get_albedo(pos),
            Object::Plane(p) => p.get_albedo(pos),
        }
    }

    pub fn get_normal(&self, pos:Point3<f32>) -> Vector3<f32> {
        match self {
            Object::Cube(c) => vec3(1.0, 1.0, 0.0),
            Object::Sphere(s) => s.get_normal(pos),
            Object::Plane(p) => p.get_normal(pos),
        }
    }
}

pub struct Sphere{
    pub idx: i32,
    position: Point3<f32>,
    r: f32,
    r2: f32

}

impl Sphere {
    pub fn new(id: i32, position: Point3<f32>, size: f32) -> Sphere {
        Sphere {
            idx : id,
            position : position,
            r : size,
            r2 : size * size,
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
        (p - self.position) * self.r2
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

pub struct Plane{
    idx: i32,
    dist: f32,
    direction: Vector3<f32>,
}

impl Plane{
    pub fn new(id: i32, dist: f32, direction: Vector3<f32>) -> Plane {
        Plane {
            idx : id,
            dist : dist,
            direction : direction,
        }
    }

    pub fn intersect(&self, ray: &mut Ray) {
        let t = -(Vector3::dot(Point3::to_vec(ray.origin), self.direction) + self.dist) / dot(ray.dir, self.direction);
        if t < ray.dist && t > 0.0 {
            ray.dist = t;
            ray.objIdx = self.idx;
        }
    }

    pub fn get_normal(&self, p:Point3<f32>) -> Vector3<f32> {
        self.direction
    }

    pub fn get_albedo(&self, p:Point3<f32>) -> Vector3<f32> {
        vec3(0.5, 0.5, 0.5)
    }
}