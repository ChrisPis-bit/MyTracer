use std::collections::HashMap;
use std::ffi::CString;
use std::io::Read;
use std::mem;
use std::os::raw::*;
use std::path::Path;
use std::ptr;
use std::fs::File;

use cgmath::Matrix;
use gl::types::*;
use image::*;

pub struct Vao{
    id: gl::types::GLuint,
}

impl Vao{
    pub fn new() -> Vao{
        let mut id = 0;
        unsafe{
            gl::GenVertexArrays(1, &mut id);
        }

        Vao{ id }
    }

    pub fn bind(&self){
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self){
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub struct BufferObject{
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
    usage: gl::types::GLenum,
}

impl BufferObject{
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> BufferObject{
        let mut id = 0;
        unsafe{
            gl::GenBuffers(1, &mut id);
        }

        BufferObject { id, r#type, usage }
    }

    pub fn bind(&self){
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }

    pub fn unbind(&self){
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn store_f32_data(&self, data: &[f32]){
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const f32 as *const c_void,
                self.usage,
            );
        }
    }

    pub fn store_i32_data(&self, data: &[i32]){
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const i32 as *const c_void,
                self.usage,
            );
        }
    }
}

pub struct VertexAttribute{
    index: GLuint
}

impl VertexAttribute{
    pub fn new(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexAttribute{
        unsafe{
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
        }

        VertexAttribute{ index }
    }

    pub  fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self){
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

pub struct ShaderProgram {
    pub program_handle: u32,
    uniform_ids: HashMap<String, GLint>
}

#[allow(temporary_cstring_as_ptr)]
impl ShaderProgram{
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> ShaderProgram {
        let mut vertex_shader_file = File::open(vertex_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertex_shader_path));
        let mut fragment_shader_file = File::open(fragment_shader_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_shader_path));

        let mut vertex_shader_source = String::new();
        let mut fragment_shader_source = String::new();

        vertex_shader_file
            .read_to_string(&mut vertex_shader_source)
            .expect("Failed to read vertex shader");

        
        fragment_shader_file
            .read_to_string(&mut fragment_shader_source)
            .expect("Failed to read fragment shader");

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);

            let program_handle = gl::CreateProgram();
            gl::AttachShader(program_handle, vertex_shader);
            gl::AttachShader(program_handle, fragment_shader);
            gl::LinkProgram(program_handle);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            ShaderProgram{
                program_handle,
                uniform_ids: HashMap::new()
            }
        }
    }

    pub fn bind(&self){
        unsafe {
            gl::UseProgram(self.program_handle);
        }
    }

    pub fn unbind(&self){
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn create_uniform(&mut self, uniform_name: &str){
        let uniform_location = unsafe {
            gl::GetUniformLocation(
                self.program_handle,
                CString::new(uniform_name).unwrap().as_ptr(),
            )
        };

        if uniform_location < 0 {
            panic!("Cannot locate uniform: {}", uniform_location);
        } else {
            self.uniform_ids.insert(uniform_name.to_string(), uniform_location);
        }
    }

    pub fn set_matrix4fv_uniform(&self, uniform_name: &str, matrix: &cgmath::Matrix4<f32>){
        unsafe {
            gl::UniformMatrix4fv(
                self.uniform_ids[uniform_name],
                1,
                gl::FALSE,
                matrix.as_ptr()
            )
        }
    }
}

pub struct Texture{
    pub id: GLuint
}

impl Texture{
    pub fn new() -> Self{
        unsafe{
            let mut id: GLuint = 0;
            gl::GenTextures(1, &mut id);
            Self { id }
        }
    }

    pub fn delete(&self){
        unsafe{
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }

    pub fn bind(&self){
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self){
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn load(&self, path: &str) -> Result<(), ImageError>{
        self.bind();

        let img = image::open(&path)?.into_rgba8();
        unsafe{
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const _,
            );

            
        // Set texture parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

        // Generate mipmaps (optional)
        gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        Ok(())
    }

    pub fn set(&self, width: i32, height: i32, data: *const u32){
        self.bind();

        unsafe{
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data as *const _,
            );

            
        // Set texture parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

        // Generate mipmaps (optional)
        gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }
}