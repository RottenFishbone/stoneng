mod event;
mod shader;
mod error;
//mod renderer;


use glfw::{Glfw, Window, Context, WindowEvent};
use std::sync::mpsc::Receiver;

pub type EngineError = error::EngineError;
pub type Key = glfw::Key;
pub type MouseButton = glfw::MouseButton;
pub type Modifiers = glfw::Modifiers;

pub trait GameCore {
    fn tick(&mut self);
    fn key_up(&mut self, key: Key, modifiers: Modifiers); 
    fn key_down(&mut self, key: Key, modifiers: Modifiers); 
    fn click_up(&mut self, button: MouseButton, modifiers: Modifiers); 
    fn click_down(&mut self, button: MouseButton, modifiers: Modifiers); 
}

pub struct Config {
    pub dimensions: (u32, u32),
    pub title: String,
}

struct Engine<'a> {
    pub glfw:   &'a mut Glfw,
    pub window: &'a mut Window,
    pub events: Receiver<(f64, WindowEvent)>,
    pub game:   &'a mut dyn GameCore
}

impl<'a> Engine<'a> {
    pub fn draw(&self){
    
    }
}


pub fn start<F>(config: Config, game: &mut F) where
    F: GameCore {

    // GLFW setup (Window/Context) 
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    
    let (mut window, events) = glfw.create_window(config.dimensions.0,
                                                  config.dimensions.1, 
                                                  &config.title, 
                                                  glfw::WindowMode::Windowed).unwrap();
    
    // Build an engine struct
    let mut engine = Engine { 
        glfw:   &mut glfw, 
        window: &mut window, 
        events, 
        game
    };
    
    // Enable window events and make it the current context
    engine.window.set_all_polling(true);
    engine.window.make_current();
    
    // OpenGL setup (Rendering)
    gl::load_with(|s| engine.window.get_proc_address(s) as *const _);
    gl::Viewport::load_with(|s| engine.window.get_proc_address(s) as *const _);
    
    unsafe {
        // Enable transparency
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
        
        // Set clear color
        gl::ClearColor(1.0, 0.5, 0.5, 1.0);

        // Enable depth testing during render
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
        
        gl::Viewport(0, 0,  config.dimensions.0 as i32, config.dimensions.1 as i32);
    }
    
    // Compile shader program from sources
    let program = shader::program_from_sources(
            include_str!("../../assets/shaders/vert.glsl").into(),
            include_str!("../../assets/shaders/frag.glsl").into(),
            None).unwrap();


    // Main Loop
    while !engine.window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        engine.game.tick();
        engine.draw();
        engine.handle_events();
        engine.window.swap_buffers();
    }
}
