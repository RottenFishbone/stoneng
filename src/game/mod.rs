#![allow(unused_variables, unused_imports, dead_code)]
use std::rc::Rc;
use std::collections::HashMap;
use nalgebra_glm::{Vec2, Vec3, Vec4, vec2};

use rand::Rng;
use specs::{Builder, World, WorldExt, Entity, RunNow, DispatcherBuilder, Dispatcher};
use stoneng::ecs::component::Scale;
use stoneng::ecs::{
    resource,
    system,
    component,
};
use stoneng::event::{KeyEvent, KeyCode, ElementState, MouseBtnEvent, MouseButton};
use stoneng::{
    self, 
    model::spritesheet::SpriteSheet,
    controller::player,
    event,
};

mod animation;

// A quick macro to return from a function on a failed Option unwrap
macro_rules! unwrap_or_return {
    ($e: expr) => {
        match $e {
            Some(v) => v,
            None => return,
        }
    }
}

pub struct GameState<'a> {
    spritesheet:        SpriteSheet,
    world:              Option<World>,
    dispatcher:         Option<Dispatcher<'a, 'a>>,
    time:               std::time::Instant,
    collision_reader:   Option<shrev::ReaderId<resource::CollisionEvent>>,

    cursor:             Option<Entity>,
    cursor_pos:         (f64, f64),
    player_contr:       Option<player::PlayerController>,
}

impl<'a> GameState<'a> {
    pub fn new() -> Self {
        Self {
            spritesheet: SpriteSheet::from_layout("assets/textures/sprites.ron".into()).unwrap(),
            world: None,
            dispatcher: None,
            time: std::time::Instant::now(),
            collision_reader: None,

            cursor: None,
            cursor_pos: (0.0, 0.0),
            player_contr: None,
        }
    }
}


impl<'a> stoneng::EngineCore for GameState<'a> {
    fn init(&mut self){
        // Setup ECS
        let mut world = World::new();
        world.insert(resource::SpritesheetPath(self.spritesheet.get_img_path().into()));
        world.insert(resource::DeltaTime(0.0));
        world.insert(resource::WindowSize(800.0, 600.0));
        world.insert(resource::View(0.0 ,0.0, 0.0));
        let mut collision_events = resource::CollisionEvents::new();
        self.collision_reader = Some(collision_events.register_reader());
        world.insert(collision_events);

        // Creates the system dispatcher, the order here is important
        let mut dispatcher = DispatcherBuilder::new()
            .with(system::collision::CollisionSys, "collision", &[])
            .with(system::particle::ParticleSys, "particle", &[])
            .with(system::movement::VelocitySys, "velocity", &[])
            .with(system::sprite::AnimSpriteSys, "anim_sprite", &[])
            // thread_local must be used with OpenGL systems as OpenGL only runs on main thread
            .with_thread_local(system::RenderSys::default())
            .with_thread_local(system::sprite::SpriteRenderSys::default())
            .with_thread_local(system::text::TextRenderSys::default())
            .with_thread_local(system::sprite::TileRenderSys::default())
            .with_thread_local(system::light::LightRenderSys::default())
            .build();
 
        dispatcher.setup(&mut world);
        let tile = self.spritesheet.sprites
                .get("human").unwrap()
                .variants.get("unarmed").unwrap()
                .clone();
        let mut pos = component::Position { x: 32.0, y: 32.0, z: -5.0 };
        let scale = component::Scale { x: 5.0, y: 5.0 };
        world.create_entity()
            .with(pos.clone())
            .with(scale.clone())
            .with(component::Color::default())
            .with(component::Sprite::from(tile.clone())) 
            .build();

        pos.x = 100.0;
        pos.y = 100.0;
        let player_anim = tile.animations.get("idle"); 
        let player_size = (self.spritesheet.tile_width-3) as f32 * scale.x;
        let player_entity = world.create_entity()
                .with(pos)
                .with(scale)
                .with(component::Color::default())
                .with(component::Sprite::from(tile.clone()))
                .with(component::Animation::from(player_anim))
                .with(component::PointLight::new(400.0))
                .with(component::Velocity { x: 0.0, y: 0.0 })
                .with(component::Collider::new(player_size, player_size))
                .with(component::Text{ 
                    content: String::from("Bobert"), size: 2.0, offset: (-25.0, 45.0) 
                })
                .build();

        self.player_contr = Some(
            player::PlayerController::new(
                player_entity, 
                player::MovementType::Instant(250.0)
                )
            );

        let cursor_sprite = self.spritesheet.sprites.get("crosshair")
                .expect(r#""crosshair" sprite could not be found"#)
                .clone();

        let cursor_size = (self.spritesheet.tile_width-2) as f32 * 3.0;
        self.cursor = Some(
            world.create_entity()
                .with(component::Position { x:0.0, y:0.0, z:1.0 })
                .with(component::Scale { x: 3.0, y: 3.0 })
                .with(component::Collider::new(cursor_size, cursor_size))
                .with(component::Sprite::from(cursor_sprite))
                .with(component::Color::default())
                .build()
        );
        
       
        let grass_sprite = self.spritesheet.sprites.get("grass").unwrap().clone();
        let mut rng = rand::thread_rng();
        for i in -25..25 {
            for j in -25..25 {
                let var;
                if rng.gen_bool(0.1) {
                    var = rng.gen_range(1..8);
                } else {
                    var = 0;
                }
                world.create_entity()
                    .with(component::Tile { pos: (i,j) })
                    .with(component::Floor { 
                        schema: grass_sprite.variants.get(&var.to_string()[..]).unwrap().clone()
                    })
                    .with(component::Color::default())
                    .build();
            }
        }
        
        world.maintain();

        self.world = Some(world);
        self.dispatcher = Some(dispatcher);
    }

    fn tick(&mut self, dt: f64){
        // Unwrap and maintain the world first
        let world = unwrap_or_return!(&mut self.world);
        world.maintain();
        // Read world resources
            // view
        let view = world.read_resource::<resource::View>();
        let (view_x,view_y,view_z) = (view.0, view.1, view.2);
            // dt
        let mut dt_res = world.write_resource::<resource::DeltaTime>();
        *dt_res = resource::DeltaTime(dt);
            // windowsize
        let win = world.read_resource::<resource::WindowSize>();
        let (win_x, win_y) = (win.0, win.1);
            // collision_events
        let collisions = world.read_resource::<resource::CollisionEvents>();
        let collision_events = collisions.read(&mut self.collision_reader.as_mut().unwrap());
        

        // Unwrap relevant entities       
        let player_contr = unwrap_or_return!(&mut self.player_contr);
        let cursor = unwrap_or_return!(&mut self.cursor);

        let mut positions = world.write_storage::<component::Position>();

        // Update player
        player_contr.tick(dt, world);
        
        let cursor_vec: Vec2; 
        {
            // Update cursor
            let mut cursor_pos = positions.get_mut(*cursor).unwrap();
            cursor_pos.x = self.cursor_pos.0 as f32 + view_x;
            cursor_pos.y = win_y - self.cursor_pos.1 as f32 + view_y;
            cursor_vec = vec2(cursor_pos.x, cursor_pos.y);
        }
        
        // Compute the aim direction as the direction from player to cursor
        let player_pos = positions.get(player_contr.player).unwrap();
        let player_vec = vec2(player_pos.x, player_pos.y);
        let aim_dir = (cursor_vec-player_vec).normalize();
        
        // Determine walking/idle
        let vels = world.read_component::<component::Velocity>();
        let player_vel = unwrap_or_return!(vels.get(player_contr.player));
        let player_speed = vec2(player_vel.x, player_vel.y).norm();
        let player_anim_state = if player_speed > 10.0 {
                animation::PlayerState::Walking
            } else {
                animation::PlayerState::Idle
            };
        // Build the required animation name based on movement/direction
        let anim_name: String = animation::PlayerAnim {
            state: player_anim_state,
            direction: animation::PlayerDirection::from(&aim_dir),
        }.into();

        // Load animation on player entity
        system::sprite::AnimSpriteSys::entity_to_anim(
            &player_contr.player, 
            &anim_name[..],
            world
        ).unwrap();
        
        // Flip the player sprite as needed
        let mut scales = world.write_component::<component::Scale>();
        let player_scale = unwrap_or_return!(scales.get_mut(player_contr.player));

        if player_scale.x < 0.0 && aim_dir.x > 0.0 || player_scale.x > 0.0 && aim_dir.x < 0.0 {
            player_scale.x *= -1.0
        }
        
        
        let mut texts = world.write_component::<component::Text>();
        let player_name = unwrap_or_return!(texts.get_mut(player_contr.player));
        player_name.content = "Bobert".into();

        for collision in collision_events {
            if collision.collider_a == player_contr.player {
                player_name.content = "Boop".into();
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
        let world = unwrap_or_return!(&mut self.world);
        let player = unwrap_or_return!(&mut self.player_contr);

        let state = match event.state {
            ElementState::Pressed => true,
            ElementState::Released => false,
        };

        if let Some(key) = event.key {
            let dir = match key {
                KeyCode::D => Some(player::MovementDirection::Right),
                KeyCode::A => Some(player::MovementDirection::Left),
                KeyCode::W => Some(player::MovementDirection::Up),
                KeyCode::S => Some(player::MovementDirection::Down),
                _ => None,
            };
        
            if let Some(dir) = dir {
                player.update_move_input(dir, state, world, None);
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

    fn mouse_btn(&mut self, event: event::MouseBtnEvent){
        match event.button {
            MouseButton::Left => {
                if event.state == ElementState::Pressed {
                    let world = unwrap_or_return!(&mut self.world);
                    let view = *world.read_resource::<resource::View>();
                    let cursor = unwrap_or_return!(&self.cursor);
                    let positions = world.read_component::<component::Position>();
                    let pos = positions.get(*cursor).unwrap().clone();
                    std::mem::drop(positions);

                    let tile = self.spritesheet.sprites
                        .get("crosshair").unwrap()
                        .clone();

                    world.create_entity()
                        .with(pos)
                        .with(component::Color::default())
                        .with(component::Scale { x: 5.0, y: 5.0 })
                        .with(component::Scaling::with_thresh(0.99, 0.25))
                        .with(component::Sprite::from(tile.clone()))
                        .with(component::PointLight::new_scaled(25.0))
                        .build();
                }
            },
            MouseButton::Right => {},
            MouseButton::Middle => {},
            MouseButton::Other(_) => {},
        }
    }

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

