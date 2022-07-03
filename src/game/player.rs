use std::collections::HashMap;
use nalgebra_glm::vec2;
use specs::{World, WorldExt, Entity};
use stoneng::ecs::{
    resource,
    system,
    component,
};

#[derive(Copy, Clone)]
pub enum MoveDir {
    Up      = 0,
    Down    = 1,
    Left    = 2,
    Right   = 3,
}

pub enum PlayerAnim {
    Idle,  
    WalkDown,  
    WalkUp,  
    WalkSide,  
    WalkDiagUp,  
    WalkDiagDown,  
}

/// PlayerEntity stores state information about a player entity as well
/// as provides simple management of that state.
pub struct PlayerEntity {
    entity:     Entity,
    max_speed:  f32,
    accel:      f32,
    deccel:     f32,
}
impl PlayerEntity {
    pub fn new(entity: Entity, max_speed: f32, accel: f32, deccel: f32) -> Self {
        Self { entity, max_speed, accel, deccel }
    }
}

/// PlayerController provides an interface between input and a Player entity's 
/// state.
///
/// This can be used to provide user input or automated input for AI.
pub struct PlayerController {
    player: PlayerEntity,
    cur_move_input: [bool; 4],

    move_vec: (f32, f32),
}
impl PlayerController {
    pub fn new(player: PlayerEntity) -> Self {
        Self { player, cur_move_input: [false; 4], move_vec: (0.0, 0.0) }
    }

    pub fn tick(&mut self, dt: f64, world: &World){
        let mut world_vels = world.write_component::<component::Velocity>();
        let mut player_vel = match world_vels.get_mut(self.player.entity) {
            Some(vel) => vel,
            None => return,
        };
        
        let player = &self.player;
        let move_dir = nalgebra_glm::vec2(self.move_vec.0, self.move_vec.1);
        let mut move_vec = move_dir;
        let mut vel = nalgebra_glm::vec2(player_vel.x, player_vel.y);
        
        let accel_frac = (
            (player.max_speed * player.accel) as f64 * dt
        ) as f32;

        move_vec += accel_frac * move_dir;

        // Handle decceleration
        let mut deccel_amt = (-vel * player.deccel) * (dt as f32);
        if vel.x.abs() > 0.0 && deccel_amt.x.abs() > 0.01 {
            let deccel_dir = vec2(deccel_amt.x, 0.0).normalize();
            if f32::abs(move_dir.x - deccel_dir.x) > 1.0 {
                deccel_amt.x = 0.0;
            }
        }
        if vel.y.abs() > 0.0 && deccel_amt.y.abs() > 0.01 {
            let deccel_dir = vec2(0.0, deccel_amt.y).normalize();
            if f32::abs(move_dir.y - deccel_dir.y) > 1.0 {
                deccel_amt.y = 0.0;
            }
        }
        
        vel += deccel_amt;
        vel += move_vec;

       
        if vel.magnitude() > player.max_speed {
            vel = vel.normalize() * player.max_speed;
        }
        else if vel.magnitude() <= 0.01 {
            vel = vec2(0.0, 0.0);
        }
        
        player_vel.x = vel.x;
        player_vel.y = vel.y;
    }

    pub fn set_move_input(&mut self, direction: MoveDir, pressed: bool, world: &World) {
        let dir_id = direction as usize;
 
        // Ignore repeated inputs in the same direction
        if self.cur_move_input[dir_id] == pressed { return }

        self.cur_move_input[dir_id] = pressed;

        let mut xforms  = world.write_component::<component::Transform>();
        let mut player_xform = match xforms.get_mut(self.player.entity) {
            Some(xform) => xform,
            None => return,
        };
        
        // Get the y_movement as a 1, 0 or -1 (up, neutral, down)
        let y_movement = {
            let (mut up, mut down) = (0, 0);
            if self.cur_move_input[MoveDir::Up as usize]   { up = 1; }
            if self.cur_move_input[MoveDir::Down as usize] { down = 1; }

            up-down
        };
        // Get the x_movement as a 1, 0 or -1 (right, neutral, left)
        let x_movement = {
            let (mut right, mut left) = (0, 0);
            if self.cur_move_input[MoveDir::Right as usize] { right = 1; }
            if self.cur_move_input[MoveDir::Left as usize]  { left = 1; }

            right-left
        };
        
        // Check if the animation should be flipped, given the current xform and movement
        if (x_movement == -1 && player_xform.scale.x > 0.0) ||
           (x_movement ==  1 && player_xform.scale.x < 0.0) {
            player_xform.scale.x *= -1.0;
        }

        let anim = match y_movement {
             1 => {
                if x_movement != 0 { PlayerAnim::WalkDiagUp }
                else { PlayerAnim::WalkUp }
             },
             0 => {
                if x_movement != 0 { PlayerAnim::WalkSide }
                else { PlayerAnim::Idle }
             },
            -1 => {
                if x_movement != 0 { PlayerAnim::WalkDiagDown }
                else { PlayerAnim::WalkDown }
            },
             _ => PlayerAnim::Idle,
        };
    
        // Change the animation to reflect player's inputted movement
        self.select_anim(anim, world);

        // Normalize and store the movement direction vector
        let move_vec = nalgebra_glm::vec2(x_movement as f32, y_movement as f32).normalize();
        self.move_vec = (
            if !f32::is_nan(move_vec.x) { move_vec.x } else { 0.0 }, 
            if !f32::is_nan(move_vec.y) { move_vec.y } else { 0.0 }
        );
    }
    
    pub fn select_anim(&mut self, new_anim: PlayerAnim, world: &World) {
        let sprites = world.read_component::<component::Sprite>();
        let anims = match sprites.get(self.player.entity) {
            Some(sprite) => &sprite.schema.animations,
            None => return,
        };

        let mut world_anims = world.write_component::<component::Animation>();
        let player_anim = match world_anims.get_mut(self.player.entity) {
            Some(anim) => anim,
            None => return,
        };

        let anim_name = match new_anim {
            PlayerAnim::Idle => "idle",
            PlayerAnim::WalkDown => "walk-down",
            PlayerAnim::WalkUp => "walk-up",
            PlayerAnim::WalkSide => "walk-side",
            PlayerAnim::WalkDiagUp => "walk-diag-up",
            PlayerAnim::WalkDiagDown => "walk-diag-down",
        };
        
        *player_anim = component::Animation::from(anims.get(anim_name));
    }
}
impl From<Entity> for PlayerController {
    fn from(entity: Entity) -> Self {
        Self {
            player:         PlayerEntity::new(entity, 300.0, 4.0, 10.0),
            move_vec:       (0.0, 0.0),
            cur_move_input: [false; 4],
        }
    }
}
