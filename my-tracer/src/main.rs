use std::ptr;
use std::time::Instant;
use cgmath::{Vector3, Zero};
use egui::{vec2, Pos2, Rect};
use gl::types::{GLfloat, GLsizei};
use my_tracer::world::camera;
use my_tracer::world::math::Math;
use my_tracer::{graphics::window::Window, world::scene::Scene};
use my_tracer::graphics::gl_wrapper::*;
use glfw::{Action, Key, WindowEvent};

fn main() {
    let mut window = Window::new(1080, 720, "Hello World");
    window.init_gl();

    let vertices: [f32; 12] = [
        1.0, 1.0, 0.0,
        1.0,-1.0,0.0,
        -1.0,-1.0,0.0,
        -1.0,1.0,0.0,
    ];

    let indices = [0,1,3,1,2,3];


    let mut egui_painter = egui_gl_glfw::Painter::new(&mut window.window_handle);
    let egui_ctx = egui::Context::default();

    let (width, height) = window.window_handle.get_framebuffer_size();
    let native_pixels_per_point = window.window_handle.get_content_scale().0;

    let mut egui_input_state = egui_gl_glfw::EguiInputState::new(
        egui::RawInput{
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0f32, 0f32),
                vec2(width as f32, height as f32) / native_pixels_per_point,

            )),
            ..Default::default()
        }, native_pixels_per_point);

    // Create fullscreen quad to display resulting render
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
    
    // Setup scene
    let mut scene = Scene::new(1080, 720, "src/textures/qwantani_dusk_1_4k.hdr");
    scene.build();

    texture.bind();

    // Initialize pixel arrays
    let mut pixels: Vec<Vector3<f32>> = vec![Vector3::zero(); 1080 * 720];
    let mut pixels_rgb8 = vec![0; 1080 * 720];

    let start_time = Instant::now();

    let mut a = 1.0;
    let mut time_last_frame = 0.0;

    while !window.should_close() {
        let start_elapsed = start_time.elapsed();
        egui_input_state.input.time = Some(start_elapsed.as_secs_f64());
        egui_ctx.begin_pass(egui_input_state.input.take());
        egui_input_state.pixels_per_point = native_pixels_per_point;

        unsafe {
            gl::ClearColor(0.3, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        // Update scene anc convert render results into rgb8
        scene.update(start_elapsed.as_secs_f32() - time_last_frame, &mut pixels, &mut pixels_rgb8);
        time_last_frame = start_elapsed.as_secs_f32();

        unsafe {          
            gl::Disable(gl::BLEND);
        }
        texture.set(1080, 720, pixels_rgb8.as_ptr());
        vao.bind();
        vbo.bind();
        ibo.bind();
        shader.bind();

        // Draw fullscreen quad     
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        let end_elapsed = start_time.elapsed();
        egui::Window::new("Egui with GLFW").show(&egui_ctx, |ui| {
            ui.label(format!("Elapsed: {}", 1.0 / (end_elapsed - start_elapsed).as_secs_f32()));
        });

        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output: _,
        } = egui_ctx.end_pass();

        //Handle cut, copy text from egui
        if !platform_output.copied_text.is_empty() {
            egui_gl_glfw::copy_to_clipboard(&mut egui_input_state, platform_output.copied_text);
        }

        let clipped_shapes = egui_ctx.tessellate(shapes, pixels_per_point);
        egui_painter.paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);

        // Events, TODO: add camera movement
        window.process_events(&mut egui_input_state,|event : &WindowEvent| {
        }
        );
        
        window.update();
    }

}
