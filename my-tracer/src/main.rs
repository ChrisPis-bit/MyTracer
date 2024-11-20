use std::ptr;

use gl::types::{GLfloat, GLsizei};
use my_tracer::graphics::window::Window;
use my_tracer::graphics::gl_wrapper::*;

fn main() {
    println!("Hello, world!");

    let mut window = Window::new(1080, 720, "Hello World");

    let vertices: [f32; 18] = [
        -1.0, -1.0, 0.0,
        1.0,-1.0,0.0,
        1.0,1.0,0.0,
        -1.0, -1.0, 0.0,
        -1.0,1.0,0.0,
        1.0,1.0,0.0
    ];

    window.init_gl();

    let vao = Vao::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(&vertices);

    let pos_attrib = VertexAttribute::new(0, 3, gl::FLOAT, gl::FALSE, 
        3 * size_of::<GLfloat>() as GLsizei, ptr::null());

    pos_attrib.enable();
    
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        window.update();
    }

}
