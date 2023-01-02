use specs::{ReadStorage, WriteStorage, System, Join, Write, Read, SystemData};
use specs::prelude::*;
use crate::model::Rect;
use crate::ecs::{
    component::{Position, Velocity, Collider},
    resource::{DeltaTime, CollisionEvents, CollisionEvent},
};


// TODO implement ncollide
#[derive(Default)]
pub struct CollisionSys;
impl<'a> System<'a> for CollisionSys {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, Position>,
                       WriteStorage<'a, Collider>,
                       Write<'a, CollisionEvents>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, positions, colliders, mut collision_events) = data;

        let collision_group = (&entities, &positions, &colliders).join();
        for (ent, pos, coll) in collision_group.clone() {
            for (ent_b, pos_b, coll_b) in collision_group.clone() {
                if ent == ent_b { continue; }
                
                // Build collision boxes using the bounds and position
                let (x,y,w,h) = (pos.x, pos.y, coll.width/2.0, coll.height/2.0);
                let coll_rect = Rect::new((x-w, y+h), (x+w, y-h));

                let (x,y,w,h) = (pos_b.x, pos_b.y, coll_b.width/2.0, coll_b.height/2.0);
                let coll_rect_b = Rect::new((x-w, y+h), (x+w, y-h));
                
                // Test for collision
                if coll_rect.intersects(&coll_rect_b) {
                    // Emit a new collision event
                    let coll_event = CollisionEvent::new(ent, ent_b);
                    collision_events.single_write(coll_event);
                }
            }
        }

    }
}
