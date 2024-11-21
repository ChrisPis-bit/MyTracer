use cgmath::*;

pub struct Ray{
    pub dir: Vector3<f32>,
    pub rdir: Vector3<f32>,
    pub origin: Point3<f32>,
    pub dist: f32,
    pub objIdx: i32
}

impl Ray{
    pub fn new(origin: Point3<f32>, dir: Vector3<f32>, dist: f32) -> Ray{
        Ray {
            dir : dir,
            rdir : Vector3::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z),
            origin : origin,
            dist : dist,
            objIdx : -1
        }
    }
}