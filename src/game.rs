#![allow(unused_variables)]

use glm::{Vec2, Vec3, Vec4};
use crate::engine::{self, Renderer, Sprite};
pub struct RustyLantern {
    renderer: Renderer,
}

impl RustyLantern {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::new(), 
        }
    }
}


impl engine::EngineCore for RustyLantern {
    fn init(&mut self){ 
        let r = self.get_renderer();
        r.sprites.push(Sprite::default());
        
        let s = r.sprites.get_mut(0).unwrap();
        s.pos = Vec3::new(0.0, 0.0, 0.0);
        s.sprite_id = 2;

        let s = s.clone(); 
        for i in 1..10 {
            let mut sc = s.clone();
            sc.pos = Vec3::new(32.0 * i as f32, 0.0, 0.0);

            if i >= 3 && i <= 5 {
                sc.sprite_id = 9
            }

            r.sprites.push(sc)

        }
    }

    fn tick(&mut self){}
    
    fn get_renderer(&mut self) -> &mut engine::Renderer {
        &mut self.renderer
    }

    fn key_up(&mut self, key: engine::Key, modifiers: engine::Modifiers){}
    fn key_down(&mut self, key: engine::Key, modifiers: engine::Modifiers){}
    fn mouse_btn_up(&mut self, button: engine::MouseButton, modifiers: engine::Modifiers){}
    fn mouse_btn_down(&mut self, button: engine::MouseButton, modifiers: engine::Modifiers){}
}

