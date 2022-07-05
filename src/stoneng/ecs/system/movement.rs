use specs::{ReadStorage, WriteStorage, System, Join, Read, SystemData};
use specs::prelude::*;
use crate::ecs::{
    component::{Position, Velocity},
    resource::DeltaTime,
};

#[derive(Default)]
pub struct MovementSys;
impl<'a> System<'a> for MovementSys {
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
