use egui_gl_glfw::EguiInputState;
use glfw::{Action, Context, CursorMode, Key, MouseButton, WindowEvent};
use std::sync::{Arc, Mutex};

struct InputState {
    key_states: [bool; 350], // Array to store the state of each key
}

impl InputState {
    fn new() -> Self {
        InputState {
            key_states: [false; 350],
        }
    }

    fn set_key_state(&mut self, key: Key, action: Action) {
        let key_idx = key as usize;
        self.key_states[key_idx] = match action {
            Action::Press | Action::Repeat => true,
            Action::Release => false,
        };
    }

    fn is_key_held(&self, key: Key) -> bool {
        self.key_states[key as usize]
    }
}

lazy_static::lazy_static! {
    static ref INPUT_STATE: Arc<Mutex<InputState>> = Arc::new(Mutex::new(InputState::new()));
}

pub fn key_held(key: Key) -> bool{
    let input_state = INPUT_STATE.lock().unwrap();
    input_state.is_key_held(key)
}

fn handle_key_input(key: Key, action: Action) {
    let mut input_state = INPUT_STATE.lock().unwrap();
    input_state.set_key_state(key, action);
}


// Struct to track mouse state
struct MouseState {
    last_x: f32,
    last_y: f32,
    delta_x: f32,
    delta_y: f32,
    initialized: bool,
}

impl MouseState {
    fn new() -> Self {
        MouseState {
            last_x: 0.0,
            last_y: 0.0,
            delta_x: 0.0,
            delta_y: 0.0,
            initialized: false
        }
    }

    fn refresh(&mut self, x: f32, y: f32){
        self.initialized = true;
        self.last_x = x;
        self.last_y = y;
    }

    fn update(&mut self, current_x: f32, current_y: f32) {
        if !self.initialized {self.refresh(current_x, current_y);}

        self.delta_x = current_x - self.last_x;
        self.delta_y = current_y - self.last_y;
        
        self.last_x = current_x;
        self.last_y = current_y;
    }

    fn get_delta(&self) -> (f32, f32) {
        (self.delta_x, self.delta_y)
    }

    fn get_position(&self) -> (f32, f32) {
        (self.last_x, self.last_y)
    }
}

lazy_static::lazy_static! {
    static ref MOUSE_STATE: Arc<Mutex<MouseState>> = Arc::new(Mutex::new(MouseState::new()));
}

fn handle_mouse_input(window: &glfw::Window) {
    let (current_x, current_y) = window.get_cursor_pos();

    let mut mouse_state = MOUSE_STATE.lock().unwrap();
    mouse_state.update(current_x as f32, current_y as f32);
}

fn refresh_mouse_input(window: &glfw::Window) {
    let (current_x, current_y) = window.get_cursor_pos();

    let mut mouse_state = MOUSE_STATE.lock().unwrap();
    mouse_state.refresh(current_x as f32, current_y as f32);
}

pub fn get_mouse_delta() -> (f32, f32) {
    let mouse_state = MOUSE_STATE.lock().unwrap();
    mouse_state.get_delta()
}

pub fn get_mouse_position() -> (f32, f32) {
    let mouse_state = MOUSE_STATE.lock().unwrap();
    mouse_state.get_position()
}



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
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_cursor_mode(glfw::CursorMode::Normal);
                    refresh_mouse_input(&self.window_handle);
                },
                glfw::WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
                    self.window_handle.set_cursor_mode(glfw::CursorMode::Disabled);
                    refresh_mouse_input(&self.window_handle);
                },
                glfw::WindowEvent::Key(key, _, action, _) => {
                    handle_key_input(key, action);
                },
                glfw::WindowEvent::FramebufferSize(width, height) =>{
                    unsafe{gl::Viewport(0, 0, width, height)}
                }
                _ => { 
                    egui_gl_glfw::handle_event(event, egui_input_state)
                }
            }
        }
        handle_mouse_input(&self.window_handle);
    }
}