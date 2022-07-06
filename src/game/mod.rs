#![allow(unused_variables, unused_imports, dead_code)]
mod player;

use std::rc::Rc;
use std::collections::HashMap;
use glutin::event::ElementState;
use nalgebra_glm::{Vec2, Vec3, Vec4};

use specs::{Builder, World, WorldExt, Entity, RunNow, DispatcherBuilder, Dispatcher};
use stoneng::ecs::component::Scale;
use stoneng::ecs::{
    resource,
    system,
    component,
};
use stoneng::event::{KeyEvent, KeyCode};
use stoneng::{
    self, 
    model::spritesheet::SpriteSheet,
    event,
};

use crate::game::player::*;

pub struct RustyLantern<'a> {
    spritesheet:        SpriteSheet,
    world:              Option<World>,
    dispatcher:         Option<Dispatcher<'a, 'a>>,
    time:               std::time::Instant,

    cursor:             Option<Entity>,
    cursor_pos:         (f64, f64),
    player:             Option<PlayerController>,
}

impl<'a> RustyLantern<'a> {
    pub fn new() -> Self {
        Self {
            spritesheet: SpriteSheet::from_layout("assets/textures/sprites.ron".into()).unwrap(),
            world: None,
            dispatcher: None,
            time: std::time::Instant::now(),

            cursor: None,
            cursor_pos: (0.0, 0.0),
            player: None,
        }
    }
}


impl<'a> stoneng::EngineCore for RustyLantern<'a> {
    fn init(&mut self){
        // Setup ECS
        let mut world = World::new();
        world.insert(resource::DeltaTime(0.0));
        world.insert(resource::WindowSize(800.0, 600.0));
        world.insert(resource::View(0.0 ,0.0, 0.0));

        let mut dispatcher = DispatcherBuilder::new()
            .with(system::movement::MovementSys, "move_sys", &[])
            .with(system::sprite::StaticSpriteSys, "static_sprite", &[])
            .with(system::sprite::AnimSpriteSys, "anim_sprite", &["static_sprite"])
            .with_thread_local(system::RenderSys::default())
            .with_thread_local(system::sprite::SpriteRenderSys::default())
            .with_thread_local(system::light::LightRenderSys::default())
            .with_thread_local(system::text::TextRenderSys::default())
            .build();
 
        dispatcher.setup(&mut world);
        let tile = self.spritesheet.sprites.get("human-unarmed").unwrap().clone();
        let mut pos = component::Position { x: 32.0, y: 32.0, z: -5.0 };
        let scale = component::Scale { x: 6.0, y: 6.0 };
        world.create_entity()
            .with(pos.clone())
            .with(scale.clone())
            .with(component::Color::default())
            .with(component::Sprite::from(tile.clone())) 
            .build();

        pos.x = 100.0;
        pos.y = 100.0;
        let player_anim = tile.animations.get("walk-side"); 
        let player_entity = world.create_entity()
                .with(pos)
                .with(scale)
                .with(component::Color::default())
                .with(component::Sprite::from(tile.clone()))
                .with(component::Animation::from(player_anim))
                .with(component::PointLight { intensity: 250.0 })
                .with(component::Velocity { x: 0.0, y: 0.0 })
                .with(component::Text{ 
                    content: String::from("Bobert"), size: 2.0, offset: (-25.0, 45.0) 
                })
                .build();

        self.player = Some(PlayerController::from(player_entity));

        let cursor_sprite = self.spritesheet.sprites.get("crosshair")
                .expect(r#""crosshair" sprite could not be found"#)
                .clone();

        self.cursor = Some(
            world.create_entity()
                .with(component::Position { x:0.0, y:0.0, z:1.0 })
                .with(component::Scale { x: 3.0, y: 3.0 })
                .with(component::Sprite::from(cursor_sprite))
                .with(component::Color::default())
                .build()
        );
        
        world.maintain();

        self.world = Some(world);
        self.dispatcher = Some(dispatcher);
    }

    fn tick(&mut self, dt: f64){
        if let Some(world) = &mut self.world {
            world.maintain();
        }

        if let Some(world) = &mut self.world {
            let mut dt_res = world.write_resource::<resource::DeltaTime>();
            *dt_res = resource::DeltaTime(dt);
            
            if let Some(player) = &mut self.player {
                player.tick(dt, world);
            }
    
            if let Some(cursor) = self.cursor {
                let view = world.read_resource::<resource::View>();
                let (view_x,view_y,view_z) = (view.0, view.1, view.2);
                let win = world.read_resource::<resource::WindowSize>();
                let (win_x, win_y) = (win.0, win.1);

                let mut positions = world.write_storage::<component::Position>();
                let mut cursor_pos = positions.get_mut(cursor).unwrap();
                cursor_pos.x = self.cursor_pos.0 as f32 + view_x;
                cursor_pos.y = win_y - self.cursor_pos.1 as f32 + view_y;
            }
        }
    }

    fn render(&mut self) {
        if let Some(dispatcher) = &mut self.dispatcher {
            if let Some(world) = &mut self.world {
                dispatcher.dispatch(world);
            }
        }              
    }
    fn post_render(&mut self) {

    }

    fn key_input(&mut self, event: event::KeyEvent){
        let world = match &self.world {
            Some(world) => world,
            None => return,
        };
        let player = match &mut self.player {
            Some(player) => player,
            None => return,
        };


        let state = match event.state {
            ElementState::Pressed => true,
            ElementState::Released => false,
        };

        if let Some(key) = event.key {
            let dir = match key {
                KeyCode::D => Some(MoveDir::Right),
                KeyCode::A => Some(MoveDir::Left),
                KeyCode::W => Some(MoveDir::Up),
                KeyCode::S => Some(MoveDir::Down),
                _ => None,
            };
        
            if let Some(dir) = dir {
                player.set_move_input(dir, state, world);
                return;
            }
            
            if state {
                let dv = match key {
                    KeyCode::Right => (10.0, 0.0),
                    KeyCode::Left => (-10.0, 0.0),
                    KeyCode::Up => (0.0, 10.0),
                    KeyCode::Down => (0.0, -10.0),
                    _ => (0.0, 0.0),
                };

                let mut view_res = world.write_resource::<resource::View>();
                let (x,y,z) = (view_res.0, view_res.1, view_res.2);
                *view_res = resource::View(x + dv.0, y + dv.1, z);
            }
        }
    }

    fn mouse_btn(&mut self, event: event::MouseBtnEvent){}

    fn cursor_moved(&mut self, x: f64, y: f64) {
        self.cursor_pos = (x, y);
    }

    fn resized(&mut self, x: u32, y: u32) {
        if let Some(world) = &self.world {
            let mut win = world.write_resource::<resource::WindowSize>();
            *win = resource::WindowSize(x as f32, y as f32);
        }
    }
}

