#![allow(dead_code)]

use nalgebra_glm::{Vec2, vec2};
use specs::{Entity, World, WorldExt};
use crate::ecs::{resource,system,component};

pub enum MovementDirection {
    Up      = 0,
    Down    = 1,
    Left    = 2,
    Right   = 3,
}

pub enum MovementType {
    /// Instantly affects player direction up to the passed max speed float.
    Instant(f32),
    /// Affects player movement using defined physical parameters.
    Velocity(MovementInfo),
}

/// A container for defining player movement
pub struct MovementInfo {
    /// That max value a player's velocity vector can reach via movement
    max_speed:  f32,

    accel:      f32,
    deccel:     f32,
}

/// PlayerController are used to translate raw input into player actions such
/// as movement.
pub struct PlayerController {
    /// The type of movement to experience, based on player input
    pub move_type:  MovementType,
    
    /// The currently pressed movement keys: [up,down,left,right]
    move_input: [bool; 4],
    /// The 2D vector representation of proposed movement
    move_vec:   Vec2,
    
    /// The ECS entity representing the player
    pub player: Entity

}
impl PlayerController {
    pub fn new(player: Entity, move_type: MovementType) -> Self {
        Self { move_type, move_input: [false; 4], move_vec: vec2(0.0, 0.0), player }
    }
    
    /// Update the control state of the player based on time.
    ///
    /// This should be called every game tick and updates the players velocity 
    /// based on input.
    pub fn tick(&mut self, dt: f64, world: &World) {
        // Grab the player's velocity component from the world
        let mut vels = world.write_component::<component::Velocity>();
        let player_vel = match vels.get_mut(self.player) {
            Some(vel) => vel,
            None => return,
        };
        
        match &self.move_type {
            // Instantly set the players velocity to defined max_speed in relevant direction
            MovementType::Instant(max_speed) => {
                player_vel.x = self.move_vec.x * max_speed;
                player_vel.y = self.move_vec.y * max_speed;
            },
            MovementType::Velocity(move_info) => todo!(),
        }
    }
    
    /// Change the state of an input direction
    ///
    /// Primarily, the purpose of this is to update the state of the PlayerController
    /// to reflect player intention.
    ///
    /// Optionally an animate function can be passed to handle an animation change on
    /// movement.
    pub fn update_move_input(
            &mut self, 
            direction: MovementDirection, 
            pressed: bool, 
            world: &World,
            animate: Option<fn(player: &Entity, move_dir: &Vec2, world: &World)>) {
        let dir_value = direction as usize;

        // Ignore repeated move input
        if self.move_input[dir_value] == pressed { return; }
        self.move_input[dir_value] = pressed;

        // Build a move direction tuple out of currently pressed keys
        self.move_vec = { 
            let y = {
                // Up - Down
                (if self.move_input[MovementDirection::Up as usize] { 1 } else { 0 }) -
                (if self.move_input[MovementDirection::Down as usize] { 1 } else { 0 })
            };

            let x = {
                // Right - Left
                (if self.move_input[MovementDirection::Right as usize] { 1 } else { 0 }) -
                (if self.move_input[MovementDirection::Left as usize] { 1 } else { 0 })
            };

            // Return a normalized move direction
            {
                let mut move_vec = vec2(x as f32, y as f32).normalize();
                if f32::is_nan(move_vec.x) { move_vec.x = 0.0 };
                if f32::is_nan(move_vec.y) { move_vec.y = 0.0 };

                move_vec
            }
        };
        
        // Call passed 'animate' function, if possible
        if let Some(animate) = animate {
            animate(&self.player, &self.move_vec, world);
        }
    }
}
