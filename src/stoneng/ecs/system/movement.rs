use specs::{ReadStorage, WriteStorage, System, Join, Read, SystemData};
use specs::prelude::*;
use crate::ecs::{
    component::{Position, Velocity},
    resource::DeltaTime,
};

/// A system to simulate entity velocity.
/// 
/// (Velocity, Position, resource::DeltaTime)
///
/// This system takes the velocity of a component and applies it to the position
/// component of each relevant entity.
#[derive(Default)]
pub struct VelocitySys;
impl<'a> System<'a> for VelocitySys {
    type SystemData = (ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>,
                       Read<'a, DeltaTime>);

    fn run(&mut self, data: Self::SystemData) {
        let (vels, mut positions, dt) = data;
        let dt = dt.0 as f32;

        for (vel, pos) in (&vels, &mut positions).join() {
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;
        }
    }
}

/// A system to provide a variable acceleration towards a target position.
///
/// (PositionTarget, Velocity, Position, resource::DeltaTime)
///
/// The system uses the PositionTarget component to variably alter the velocity component in order
/// to move the object towards a target position.
#[derive(Default)]
pub struct PositionTargetSys;
impl<'a> System<'a> for PositionTargetSys {
    type SystemData = ();

    fn run(&mut self, data: Self::SystemData) {
        todo!()
    }
}
