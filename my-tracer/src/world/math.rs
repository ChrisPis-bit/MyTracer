use cgmath::*;

pub struct Math;

static mut SEED : u32 = 0x11112222;

impl Math{
    pub fn reflect(incident: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
        incident - 2.0 * incident.dot(normal) * normal
    }

    pub fn random_seed_uint() -> u32{
        unsafe {
            SEED ^= SEED << 13;
            SEED ^= SEED >> 17;
            SEED ^= SEED << 5;
            return SEED;    
        }
    }

    pub fn random_uint(seed : &mut u32) -> u32 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 17;
        *seed ^= *seed << 5;
        return *seed;
    }

    pub fn random_f32(seed : &mut u32) -> f32{
        Math::random_uint(seed) as f32 * 2.3283064365387e-10
    }

    pub fn random_uniform_vectorf32(seed : &mut u32) -> Vector3<f32>{
        let x = Math::random_f32(seed) * 2.0 - 1.0;
        let y = Math::random_f32(seed) * 2.0 - 1.0;
        let z = Math::random_f32(seed) * 2.0 - 1.0;

        vec3(x,y,z).normalize()
    }

    pub fn random_uniform_hemisphere_vectorf32(seed : &mut u32, normal: Vector3<f32>) -> Vector3<f32>{
        let v = Math::random_uniform_vectorf32(seed);
        if normal.dot(v) < 0.0 { return -v }
        return v;
    }

    pub fn random_range_f32(seed : &mut u32, min:f32, max:f32) -> f32 {
        Math::random_f32(seed) * (max - min) 
    }

    pub fn random_range_u32(seed : &mut u32, min:u32, max:u32) -> u32{
        Math::random_range_f32(seed, min as f32, max as f32).round() as u32
    }
}