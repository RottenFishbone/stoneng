#![allow(unused_variables, unused_imports, dead_code)]

use std::rc::Rc;
use nalgebra_glm::{Vec2, Vec3, Vec4};
use stoneng::{
    self, 
    sprite::{Sprite, SpriteSheet},
    event,
}; 
pub struct RustyLantern {
    renderer:       stoneng::Renderer,
    spritesheet:    SpriteSheet,
}

impl RustyLantern {
    pub fn new() -> Self {
        Self {
            renderer: stoneng::Renderer::new(),
            spritesheet: SpriteSheet::from_layout("assets/textures/atlas.ron".into()).unwrap()
        }
    }
}


impl stoneng::EngineCore for RustyLantern {
    fn init(&mut self){ 
        let schema = self.spritesheet.sprites["man"].clone();
        let mut sprite = Sprite::new(
                              Vec3::from([128.0, 128.0, 1.0]), 
                              Vec4::from([1.0, 1.0, 1.0, 1.0]), 
                              Vec2::from([1.0, 1.0]),
                              0.0,
                              schema.clone());
        
        sprite.to_animation(Some(schema.animations["walk"].clone()));
        self.renderer.sprites.push(sprite);
    }

    fn tick(&mut self, dt: f64){
        for sprite in self.renderer.sprites.iter_mut() {
            sprite.advance_animation(dt);
        }
    }

    fn pre_render(&mut self) {
    }
    
    fn post_render(&mut self) {}

    fn get_renderer(&mut self) -> &mut stoneng::Renderer {
        &mut self.renderer
    }
    fn key_up(&mut self, key: event::Key, modifiers: event::Modifiers){}
    fn key_down(&mut self, key: event::Key, modifiers: event::Modifiers){}
    fn mouse_btn_up(&mut self, button: event::MouseButton, modifiers: event::Modifiers){}

    fn mouse_btn_down(&mut self, button: event::MouseButton, modifiers: event::Modifiers){}
}

