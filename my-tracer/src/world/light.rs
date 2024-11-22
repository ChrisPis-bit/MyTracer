use cgmath::{vec3, Point3, Vector3};

use super::primitives::Sphere;

pub enum Light {
    Point(PointLight),
    //Sphere(Sphere)
}

impl Light {
    pub fn idx(&self) -> i32 {
        match self {
            Light::Point(p) => p.idx,
            //Light::Sphere(s) => s.idx
        }
    }

    pub fn get_position(&self) -> Point3<f32> {
        match self {
            Light::Point(p) => p.get_position(),
            //Light::Sphere(s) => s.idx
        }
    }

    pub fn get_albedo(&self) -> Vector3<f32> {
        match self {
            Light::Point(p) => p.get_albedo(),
            //Light::Sphere(s) => s.idx
        }
    }
}

pub struct PointLight{
    idx: i32,
    color : Vector3<f32>,
    position: Point3<f32>, 
}

impl PointLight{
    pub fn new(idx: i32, color: Vector3<f32>, position: Point3<f32>) -> PointLight {
        PointLight{
            idx,
            color,
            position
        }
    }
    pub fn get_position(&self) -> Point3<f32> { self.position }
    pub fn get_albedo(&self) -> Vector3<f32> { self.color }
}