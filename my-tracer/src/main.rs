use std::ffi::CString;
use std::ptr;

use gl::types::{GLfloat, GLsizei};
use my_tracer::{graphics::window::Window, world::scene::Scene};
use my_tracer::graphics::gl_wrapper::*;
use glfw::{Action, Context, Key, WindowEvent};

fn main() {
    println!("Hello, world!");

    let mut window = Window::new(1080, 720, "Hello World");

    let vertices: [f32; 12] = [
        1.0, 1.0, 0.0,
        1.0,-1.0,0.0,
        -1.0,-1.0,0.0,
        -1.0,1.0,0.0,
    ];

    let indices = [0,1,3,1,2,3];

    window.init_gl();

    let vao = Vao::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(&vertices);

    let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ibo.bind();

    ibo.store_i32_data(&indices);

    let pos_attrib = VertexAttribute::new(0, 3, gl::FLOAT, gl::FALSE, 
        3 * size_of::<GLfloat>() as GLsizei, ptr::null());

    pos_attrib.enable();

    let index_attrib = VertexAttribute::new(1, 3, gl::FLOAT, gl::FALSE, 
        3 * size_of::<GLfloat>() as GLsizei, ptr::null());

    index_attrib.enable();

    let shader = ShaderProgram::new("src/shaders/fullscreenVert.vert", "src/shaders/fullscreenFrag.frag");
    shader.bind();

    let texture = Texture::new();
    //let res = texture.load("src/textures/grem.jpg");
    
    let mut scene = Scene::new(1080, 720, "src/textures/qwantani_dusk_1_4k.hdr");
    texture.bind();


    while !window.should_close() {
        
        window.process_events(|event| 
            match event{
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) =>{
                }
                _ => {}
            }
        );

        scene.update();
        texture.set(1080, 720, scene.pixels.as_ptr());

        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.update();
    }

}
