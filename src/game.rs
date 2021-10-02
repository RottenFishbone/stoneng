#![allow(unused_variables, unused_imports, dead_code)]
use std::rc::Rc;
use std::collections::HashMap;
use nalgebra_glm::{Vec2, Vec3, Vec4};

use stoneng::{
    self, 
    sprite::{Sprite, SpriteSheet},
    event,
}; 
pub struct RustyLantern {
    spritesheet:    SpriteSheet,
}

impl RustyLantern {
    pub fn new() -> Self {
        Self {
            spritesheet: SpriteSheet::from_layout("assets/textures/atlas.ron".into()).unwrap(),
        }
    }
}


impl stoneng::EngineCore for RustyLantern {
    fn init(&mut self){}

    fn tick(&mut self, dt: f64){}

    fn render(&mut self) {}
    fn post_render(&mut self) {}

    fn cursor_moved(&mut self, x: f64, y: f64) {}

    fn key_input(&mut self, event: event::KeyEvent){}

    fn mouse_btn(&mut self, event: event::MouseBtnEvent){}
}

