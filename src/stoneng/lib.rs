#![allow(unused_imports, unused_variables)]

extern crate nalgebra_glm as glm;
extern crate stb_image as stb;

pub mod sprite;
pub mod event;
mod shader;
mod error;
mod renderer;

use event::*;
use gl::types::*;
use glfw::{Glfw, Window, Context, WindowEvent};
use std::sync::mpsc::Receiver;

// Aliases
pub type EngineError = error::EngineError;
pub type Renderer = renderer::Renderer;

/// Provides interfacing functions for the engine.
///
/// These functions are used as an API to the engine and serve as the 
/// basis for interaction with the main loop and rendering.
pub trait EngineCore {
    // Engine Cycle
    /// Called once, after context creation, before initial draw. 
    fn init(&mut self){}
    /// Called once per engine update.
    fn tick(&mut self, dt: f64){}
    /// Called just before the renderer draws
    fn pre_render(&mut self) {}
    /// Called right after the renderer draws
    fn post_render(&mut self) {}

    // Rendering
    /// Called by the engine when the renderer is required.
    /// 
    /// This should simply return a stored renderer used to draw.
    fn get_renderer(&mut self) -> &mut Renderer;

    // Input
    /// Called on a key being released.
    fn key_up(&mut self, key: Key, modifiers: Modifiers){} 
    /// Called on a key being pressed.
    fn key_down(&mut self, key: Key, modifiers: Modifiers){}
    /// Called on a mouse button being released.
    fn mouse_btn_up(&mut self, button: MouseButton, modifiers: Modifiers){}
    /// Called on a mouse button being pressed.
    fn mouse_btn_down(&mut self, button: MouseButton, modifiers: Modifiers){}

    fn cursor_pos(&mut self, x: f64, y: f64) {}
}


pub struct Config {
    pub dimensions: (u32, u32),
    pub title: String,
    pub fullscreen: bool,
    pub resizable: bool,
}


impl Config {
    pub fn default() -> Self {
        Self {
            dimensions: (800, 600),
            title: "rustylantern".into(),
            fullscreen: false,
            resizable: false,
        }
    }
}


struct Engine<'a> {
    config: &'a Config,
    glfw:   &'a mut Glfw,
    window: &'a mut Window,
    events: Receiver<(f64, WindowEvent)>,
    game:   &'a mut dyn EngineCore,
}


impl<'a> Engine<'a> {
    /// Initialize the engine's graphics context and call the game's init code.
    ///
    /// This sets the sets the window to current and runs all neccessary OpenGL
    /// initialization code.
    /// Finally the game's init function is called.
    fn init(&mut self) {
        // Enable window events and make it the current context
        self.window.set_all_polling(true);
        self.window.make_current();
        
        // OpenGL setup (Rendering)
        gl::load_with(|s| self.window.get_proc_address(s) as *const _);
        //gl::Viewport::load_with(|s| self.window.get_proc_address(s) as *const _);
        
        // Unsafe OpenGL setup
        unsafe {
            // Enable transparency
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
            
            // Set clear color
            //gl::ClearColor(0.4, 0.35, 0.45, 1.0);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);

            // Enable depth testing during render
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            
            // Set the viewport's dimensions. This should match the window.
            gl::Viewport( 0, 0,
                self.config.dimensions.0 as i32, 
                self.config.dimensions.1 as i32);

            gl::PointSize(10.0);
        }
       
        // Initialize the game
        self.game.get_renderer().init().unwrap();
        self.game.init();
    }
}


pub fn start<F>(config: Config, game: &mut F) where
    F: EngineCore {

    // GLFW setup (Window/Context) 
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    // OpenGL Version 4.3
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    // Build the glfw window
    let (mut window, events) = 
        glfw.create_window(
            config.dimensions.0,
            config.dimensions.1, 
            &config.title, 
            glfw::WindowMode::Windowed).unwrap();
    
    // Build the engine 
    let mut engine = Engine {
        config: &config,
        glfw:   &mut glfw, 
        window: &mut window, 
        events, 
        game
    };
   
    engine.init();


    // Main Loop
    let mut last_tick = engine.glfw.get_time();
    while !engine.window.should_close() {
        // Handle window events
        engine.handle_events();

        // Just tick every frame for now
        let now = engine.glfw.get_time();
        engine.game.tick(now-last_tick);
        last_tick = now;
        
        engine.game.pre_render();
        engine.game.get_renderer().render();
        engine.game.post_render();

        // Display rendered buffer
        engine.window.swap_buffers();
    }
}
