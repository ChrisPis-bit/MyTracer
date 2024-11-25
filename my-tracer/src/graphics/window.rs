use glfw::{Action, Context, Key, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Window{
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,
}

impl Window{
    pub fn new(width: u32, height: u32, title: &str) -> Window{
        let mut glfw = glfw::init_no_callbacks().unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window{
            glfw,
            window_handle: window,
            events,
        }
    }

    pub fn init_gl(&mut self){
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool{
        self.window_handle.should_close()
    }

    pub fn update(&mut self){
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    pub fn process_events(&mut self, callback: fn(WindowEvent)){
        for (_, event) in glfw::flush_messages(&self.events) {
            match event{
                glfw::WindowEvent::FramebufferSize(width, height) =>{
                    unsafe{gl::Viewport(0, 0, width, height)}
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) =>{
                    self.window_handle.set_should_close(true)
                }
                _ => {}
            }

            callback(event);
        }
    }
}