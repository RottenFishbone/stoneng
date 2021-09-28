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
    renderer:       stoneng::Renderer,
    spritesheet:    SpriteSheet,
    time:           f64,
    held:           HashMap<char, bool>,
}

impl RustyLantern {
    pub fn new() -> Self {
        Self {
            renderer: stoneng::Renderer::new(),
            spritesheet: SpriteSheet::from_layout("assets/textures/atlas.ron".into()).unwrap(),
            time: 0.0,
            held: HashMap::new(),
        }
    }
}


impl stoneng::EngineCore for RustyLantern {
    fn init(&mut self){ 
        self.renderer.lights.push(Vec3::from([400.0, 300.0, 100.0]));
        
        let mut schema = self.spritesheet.sprites["man"].clone();
        let mut sprite = Sprite::new(
                              Vec3::from([450.0, 320.0, 1.0]), 
                              Vec4::from([1.0, 1.0, 1.0, 1.0]), 
                              Vec2::from([1.0, 1.0]),
                              0.0,
                              schema.clone());
        sprite.to_animation(Some(schema.animations["walk"].clone()));
        self.renderer.sprites.push(sprite);


        schema = self.spritesheet.sprites["brick"].clone();
        sprite = Sprite::new(
                    Vec3::from([0.0, 0.0, 0.1]),
                    Vec4::from([1.0, 1.0, 1.0, 1.0]),
                    Vec2::from([1.0, 1.0]),
                    0.0,
                    schema.clone());
        for i in 0..1000{
            let mut s = sprite.clone();
            s.pos = Vec3::from([
                ((i*32)%800) as f32, 
                (((i*32)/800)*32) as f32, 
                1.0
            ]);
            self.renderer.sprites.push(s);
        }

        sprite = self.renderer.sprites.remove(0);
        self.renderer.sprites.push(sprite);

        self.held.insert('A', false);
        self.held.insert('D', false);
    }

    fn tick(&mut self, dt: f64){
        self.time += dt;
        for sprite in self.renderer.sprites.iter_mut() {
            sprite.advance_animation(dt);
        }

        for light in self.renderer.lights.iter_mut() {
            light.z = 150.0 + (self.time.sin() as f32) * 10.0;
        }

        if *self.held.get(&'A').unwrap() {
            let ply = self.renderer.sprites.last_mut().unwrap();
            ply.pos.x -= 2.0;
            ply.scale.x = -1.0; 
        } 
        if *self.held.get(&'D').unwrap() {
            let ply = self.renderer.sprites.last_mut().unwrap();
            ply.pos.x += 2.0;
            ply.scale.x = 1.0; 
        }
    }

    fn pre_render(&mut self) {}
    

    fn post_render(&mut self) {}

    fn get_renderer(&mut self) -> &mut stoneng::Renderer {
        &mut self.renderer
    }

    fn key_up(&mut self, key: event::Key, modifiers: event::Modifiers){
        match key {
            event::Key::A => {
                self.held.insert('A', false);
            },
            event::Key::D => {
                self.held.insert('D', false);
            },
            _ => {}
        }
    

    }

    fn key_down(&mut self, key: event::Key, modifiers: event::Modifiers){
        match key {
            event::Key::A => {

                self.held.insert('A', true);
            },
            event::Key::D => {
                self.held.insert('D', true);
            },
            _ => {}
        }
    }

    fn mouse_btn_up(&mut self, button: event::MouseButton, modifiers: event::Modifiers){}

    fn mouse_btn_down(&mut self, button: event::MouseButton, modifiers: event::Modifiers){
        let l = self.renderer.lights.get(0).unwrap().clone();
        self.renderer.lights.push(l);

    }

    fn cursor_pos(&mut self, x: f64, y: f64) {
        let l = self.renderer.lights.get_mut(0).unwrap();
        let x = x as f32; let y = y as f32;
        *l = Vec3::from([x, 600.0-y, l.z]);
    }
}

