use specs::{ReadStorage, WriteStorage, System, Join, Read, SystemData};
use specs::prelude::*;
use std::sync::Arc;
use crate::{
    model::spritesheet::{SpriteSheet, AnimationSchema},
    ecs::resource::{DeltaTime, WindowSize, View},
    ecs::component::{Position, PointLight, Scale},
    renderer::{
        sprite::{RenderSprite, SpriteRenderer}, 
        light::{RenderLight, LightRenderer},
    },
};


/// A System for rendering lights to the screen
///
/// As this is an OpenGL system it must be called on the main thread with `with_thread_local`
#[derive(Default)] 
pub struct LightRenderSys {
    renderer: LightRenderer,
}
impl<'a> System<'a> for LightRenderSys {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, PointLight>,
                       ReadStorage<'a, Scale>,
                       Read<'a, WindowSize>,
                       Read<'a, View>);

    fn run(&mut self, data: Self::SystemData) {
        let (posns, lights, scales, window, view) = data;
        let window = (window.0, window.1);
        let view = (view.0, view.1, view.2);
        let lights: Vec<RenderLight> = (&posns, &lights, scales.maybe()).join()
            .map(|(pos, light, scale)| {
                let intensity = if light.scaled && scale.is_some() { 
                    let scale = scale.unwrap();
                    light.intensity * scale.x.max(scale.y)
                } else { light.intensity };

                RenderLight { pos: (pos.x, pos.y), intensity }
            })
            .collect();

        self.renderer.render(&lights, window, view);
    }
    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.renderer = LightRenderer::new();
        self.renderer.init().unwrap();
        self.renderer.dither_scale = 2.0;
    }
}
