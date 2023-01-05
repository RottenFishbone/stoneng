#![allow(unused_imports, unused_variables)]

extern crate nalgebra_glm as glm;
extern crate stb_image as stb;

pub mod model;
pub mod event;
pub mod renderer;
pub mod ecs;
pub mod controller;

pub mod audio;

mod shader;
mod error;

use event::*;
use std::time::Instant;
use gl::types::*;
use std::sync::mpsc::{self, Sender, Receiver};
use glutin::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::{self, ControlFlow, EventLoop},
    window::{self, Window, WindowBuilder},
    dpi::PhysicalSize,
};

// Aliases
pub type EngineError = error::EngineError;

/// Provides interfacing functions for the engine.
///
/// These functions are used as an API to the engine and serve as the 
/// basis for interaction with the main loop and rendering.
pub trait EngineCore {
        // Engine Cycle
    /// Called once, after context creation, before initial draw. 
    fn init(&mut self);
    /// Called once per engine update with the number of seconds since the last draw.
    fn tick(&mut self, dt: f64);

        // Rendering
    /// Called when the context is ready for drawing
    fn render(&mut self);
    /// Called after the context has been drawn to and displayed
    fn post_render(&mut self) {}

        // Input
    /// Called on a keyboard input state being changed.
    fn key_input(&mut self, event: KeyEvent){} 
    /// Called when a mouse button has changed state.
    fn mouse_btn(&mut self, event: MouseBtnEvent){}
    /// Called when the cursor moves within the window
    fn cursor_moved(&mut self, x: f64, y: f64) {}
    
        // Window
    /// Called on a window resize, the parameters being the new window size.
    fn resized(&mut self, x: u32, y: u32) {} 
}


pub struct Config {
    /// Viewport and window dimensions.
    pub dimensions: (u32, u32),
    /// The title of the window
    pub title:      String,
    /// Hide window decorations and use native resolution for window size
    pub fullscreen: bool,
    /// Is the window resizable (this is not _always_ respected)
    pub resizable:  bool,
    
    /// The maximum number of frames per seconds, 0 for uncapped.
    pub frame_cap:  u32,

    /// The major and minor version of opengl to use 
    /// Might need to be upgraded from default for some shaders
    pub opengl_version: (u8, u8)
}


impl Config {
    pub fn default() -> Self {
        Self {
            dimensions: (800, 600),
            title: "StoneEng".into(),
            fullscreen: false,
            resizable: false,

            frame_cap: 100,

            opengl_version: (4, 2),
        }
    }
}

pub fn start<F, G>(config: Config, game: F) where
    G: 'static + EngineCore,
    F: 'static + FnOnce() -> G {
    let mut game = game();
    let window_size = PhysicalSize::new(config.dimensions.0, config.dimensions.1);
    // Spawn the event loop thread and build the context
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(config.title.clone())
        .with_inner_size(window_size)
        .with_resizable(config.resizable);
    let ctx = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, config.opengl_version))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();
    let ctx = unsafe { ctx.make_current().unwrap() };
    
    gl::load_with(|ptr| ctx.context().get_proc_address(ptr) as *const _);
    
    init_gl(&config);
    game.init();
    
    ctx.window().set_cursor_visible(false);
    
    let mut last_frame = Instant::now();
    let frametime = if config.frame_cap > 0 { 
        1.0/config.frame_cap as f64 * 1_000_000.0 
    } else { 0.0 };

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; },
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        if key == VirtualKeyCode::Escape {
                            *control_flow = ControlFlow::Exit;
                        }
                    }

                    game.key_input(input.into());
                },
                WindowEvent::MouseInput {state, button, ..} => {
                    game.mouse_btn(MouseBtnEvent { state, button });
                },
                WindowEvent::CursorMoved { position, .. } => {
                    game.cursor_moved(position.x, position.y);
                },
                WindowEvent::Resized(new_size) => {
                    game.resized(new_size.width, new_size.height); 
                },
                _ => {}
            },
            Event::MainEventsCleared => { 
                let dt = last_frame.elapsed().as_micros() as f64;
                
                // Frame limiting
                if dt < frametime { 
                    let time_to_tick = ((frametime-dt)*0.95)as u64;
                    std::thread::sleep(std::time::Duration::from_micros(time_to_tick));
                    return;
                }

                // Run game logic (converting time into seconds)
                game.tick(dt / 1_000_000.0);

                last_frame = Instant::now();
                game.render();

                // Call renderers here
                ctx.swap_buffers().unwrap();
                
                game.post_render();
            }

            _ => {},
        }
    });


}

fn init_gl(config: &Config){
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
        let (width, height) = (config.dimensions.0 as i32, config.dimensions.1 as i32);
        gl::Viewport( 0, 0, width, height);

        gl::PointSize(10.0);
    }
}
