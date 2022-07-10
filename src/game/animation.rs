use std::collections::HashMap;
use nalgebra_glm::{Vec2, vec2};
use lazy_static::lazy_static;

/// The animation state of the player
pub enum PlayerState {
    Idle, 
    Walking,
}
impl From<PlayerState> for &str {
    fn from(val: PlayerState) -> Self {
        match val {
            PlayerState::Idle       => "idle",
            PlayerState::Walking    => "walk",
        }
    }
}

// Define a set of vectors mapped to a direction for fast vector lookup
lazy_static! {
    static ref ANIM_DIRS: [(Vec2, PlayerDirection); 5] = [
        (vec2(0.0,  1.0), PlayerDirection::Up),
        (vec2(0.0, -1.0), PlayerDirection::Down),
        (vec2(1.0,  0.0), PlayerDirection::Side),
        (vec2(1.0,  1.0).normalize(), PlayerDirection::DiagUp),
        (vec2(1.0, -1.0).normalize(), PlayerDirection::DiagDown),
    ];
}
/// The sprite direction the player is travelling
#[derive(Copy, Clone, Debug)]
pub enum PlayerDirection {
    Up,
    Down,
    Side,
    DiagUp,
    DiagDown,
}
impl From<PlayerDirection> for &str {
    fn from(val: PlayerDirection) -> Self {
        match val {
            PlayerDirection::Up         => "up",
            PlayerDirection::Down       => "down",
            PlayerDirection::Side       => "side",
            PlayerDirection::DiagUp     => "diag-up",
            PlayerDirection::DiagDown   => "diag-down",
        }
    }
}
impl From<&Vec2> for PlayerDirection {
    fn from(v: &Vec2) -> Self {
        let tar_v = vec2(f32::abs(v.x), v.y);
        let (min_id, _) = ANIM_DIRS.iter().enumerate()
            .fold((usize::MAX, f32::INFINITY), |(min_id, min_dist), x| { 
                let (id, val) = x;
                let (val, _) = val;
                let dist = val.metric_distance(&tar_v);
               
                if dist < min_dist {
                    (id, dist)
                } else {
                    (min_id, min_dist)
                }
            });
        
        ANIM_DIRS[min_id].1.clone()
    }
}


/// An animation definition of the player. This is directly mapped to a string
/// that can be indexed in the spritesheet.
pub struct PlayerAnim {
    pub state:      PlayerState,
    pub direction:  PlayerDirection
}
impl Into<String> for PlayerAnim {
    fn into(self) -> String {
        let state: &str = self.state.into();
        let dir: &str = self.direction.into();

        format!("{}-{}", state, dir)
    }
}
