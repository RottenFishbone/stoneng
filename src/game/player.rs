use std::collections::HashMap;
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
    entity: Entity,
}
impl PlayerEntity {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

/// PlayerController provides an interface between input and a Player entity's 
/// state.
///
/// This can be used to provide user input or automated input for AI.
pub struct PlayerController {
    player: PlayerEntity,
    cur_movement: [bool; 4],
}
impl PlayerController {
    pub fn new(player: PlayerEntity) -> Self {
        Self { player, cur_movement: [false; 4] }
    }
    
    pub fn set_move(&mut self, direction: MoveDir, pressed: bool, world: &World) {
        let dir_id = direction as usize;
 
        // Ignore repeated inputs in the same direction
        if self.cur_movement[dir_id] == pressed { return }

        self.cur_movement[dir_id] = pressed;

        // Adjust velocity
        let mut vels    = world.write_component::<component::Velocity>();
        let mut player_vel = match vels.get_mut(self.player.entity) {
            Some(vel) => vel,
            None => return,
        };
        let vel_sign = if pressed { 1.0 } else { -1.0 };
        match direction {
            MoveDir::Up => player_vel.y += 250.0 * vel_sign,
            MoveDir::Down => player_vel.y -= 250.0 * vel_sign,
            MoveDir::Left => player_vel.x -= 250.0 * vel_sign,
            MoveDir::Right => player_vel.x += 250.0 * vel_sign,
        }

        let mut xforms  = world.write_component::<component::Transform>();
        let mut player_xform = match xforms.get_mut(self.player.entity) {
            Some(xform) => xform,
            None => return,
        };
        
        // Get the y_movement as a 1, 0 or -1 (up, neutral, down)
        let y_movement = {
            let (mut up, mut down) = (0, 0);
            if self.cur_movement[MoveDir::Up as usize]   { up = 1; }
            if self.cur_movement[MoveDir::Down as usize] { down = 1; }

            up-down
        };
        // Get the x_movement as a 1, 0 or -1 (right, neutral, left)
        let x_movement = {
            let (mut right, mut left) = (0, 0);
            if self.cur_movement[MoveDir::Right as usize] { right = 1; }
            if self.cur_movement[MoveDir::Left as usize]  { left = 1; }

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

        self.select_anim(anim, world);
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
            player: PlayerEntity { entity },
            cur_movement: [false; 4],
        }
    }
}
