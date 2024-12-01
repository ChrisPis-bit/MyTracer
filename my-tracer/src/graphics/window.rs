use egui_gl_glfw::EguiInputState;
use glfw::{Action, Context, Key, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Window{
    pub glfw: glfw::Glfw,
    pub window_handle: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, WindowEvent)>,
}

impl Window{
    pub fn new(width: u32, height: u32, title: &str) -> Window{
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(false));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.set_char_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);


        Window{
            glfw,
            window_handle: window,
            events,
        }
    }

    pub fn init_gl(&mut self){
        self.window_handle.make_current();
        self.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);

    }

    pub fn should_close(&self) -> bool{
        self.window_handle.should_close()
    }

    pub fn update(&mut self){
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    pub fn process_events(&mut self, egui_input_state: &mut EguiInputState, callback: fn(&WindowEvent)){
        for (_, event) in glfw::flush_messages(&self.events) {
            match event{
                glfw::WindowEvent::FramebufferSize(width, height) =>{
                    unsafe{gl::Viewport(0, 0, width, height)}
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) =>{
                    self.window_handle.set_should_close(true)
                }
                _ => { 
                    egui_gl_glfw::handle_event(event, egui_input_state)
                }
            }
        }
    }
}