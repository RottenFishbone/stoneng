#![allow(unused_imports, unused_variables)]

extern crate nalgebra_glm as glm;
extern crate stb_image as stb;

pub mod sprite;
pub mod event;
mod shader;
mod error;
mod renderer;

use event::*;
use std::time::Instant;
use gl::types::*;
use std::sync::mpsc::{self, Sender, Receiver};
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop},
    window::{self, Window, WindowBuilder},
    dpi::PhysicalSize,
};

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
    //fn key_up(&mut self, key: Key, modifiers: Modifiers){} 
    /// Called on a key being pressed.
    //fn key_down(&mut self, key: Key, modifiers: Modifiers){}
    /// Called on a mouse button being released.
    //fn mouse_btn_up(&mut self, button: MouseButton, modifiers: Modifiers){}
    /// Called on a mouse button being pressed.
    //fn mouse_btn_down(&mut self, button: MouseButton, modifiers: Modifiers){}

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

pub fn start<F, G>(config: Config, game: F) where
    G: 'static + EngineCore,
    F: 'static + FnOnce() -> G {
    
    let mut game = game();

    // Spawn the event loop thread and build the context
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("rustylantern")
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_resizable(false);
    let ctx = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 1)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();
    let ctx = unsafe { ctx.make_current().unwrap() };
    
    gl::load_with(|ptr| ctx.context().get_proc_address(ptr) as *const _);
    
    init_gl();
    game.init();
    game.get_renderer().init().unwrap();
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; },
                _ => {}
            },
            Event::MainEventsCleared => { 
                game.get_renderer().render();

                ctx.swap_buffers().unwrap();
            }

            _ => {},
        }
    });


}


fn init_gl(){
    unsafe {
        // Enable transparency
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); 
        
        // Set clear color
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);

        // Enable depth testing during render
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
        
        // Set the viewport's dimensions. This should match the window.
        gl::Viewport( 0, 0, 800, 600);

        gl::PointSize(10.0);
    }
}
