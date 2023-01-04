use specs::{ReadStorage, WriteStorage, System, Join, Read, SystemData};
use specs::prelude::*;
use std::sync::Arc;
use crate::ecs::component;
use crate::error::EngineError;
use crate::{
    model::spritesheet::{SpriteSheet, AnimationSchema, AnimMode},
    ecs::resource::{DeltaTime, WindowSize, View, SpritesheetPath},
    ecs::component::{Color, Sprite, Position, Scale, Animation, tile::*},
    renderer::sprite::{RenderSprite, SpriteRenderer},
    renderer::light::{RenderLight, LightRenderer},
};

use std::{
    fs::File,
    path::PathBuf,
    io::Read as FileRead,
};


#[derive(Default)]
pub struct AnimSpriteSys;
impl AnimSpriteSys {
    /// A helper function to allow for easier setting of animations, by name.
    ///
    /// Note, this mutates the passed world and relevant components.
    pub fn entity_to_anim(entity: &Entity, anim_name: &str, world: &World) 
            -> Result<(), EngineError> {
        
        let sprites = world.read_component::<component::Sprite>();
        let sprite = match sprites.get(*entity) {
            Some(s) => s,
            None => return Err(
                EngineError::AnimationError(
                    format!("Entity {:?} does not have Sprite component", entity)
                )    
            ),
        };

        let mut anims = world.write_component::<component::Animation>();
        let anim = match anims.get_mut(*entity) {
            Some(a) => a,
            None => return Err(
                EngineError::AnimationError(
                    format!("Entity {:?} does not have Animation component", entity)
                )    
            ),
        };
        
        let new_anim = component::Animation::from_name(anim_name, sprite);
        if anim.schema.clone().unwrap() != new_anim.schema.unwrap() {
            *anim = component::Animation::from_name(anim_name, sprite);
        }

        Ok(())
    }
    
    fn sprite_to_idle(sprite: &mut Sprite, anim: &mut Animation) {
        anim.frame = 0;
        anim.frame_progress = 0.0;
        anim.is_reversing = false;        
        anim.schema = sprite.schema.animations.get("idle").map(|s| s.clone());
    }
    
    /// Moves an animation component's frames forward based in its frame_time and
    /// the time passed since the last update.
    fn advance_animation(s: &mut Sprite, a: &mut Animation, dt: f64){
        let schema = match &a.schema {
            Some(schema) => schema.clone(),
            None => return,
        };

        let loops = schema.mode == AnimMode::Loop || schema.mode == AnimMode::LoopReverse;
        let reverses = schema.mode == AnimMode::Reverse || schema.mode == AnimMode::LoopReverse;

        // Calculate how many frames need to be played ( as a float )
        a.frame_progress += (dt/(schema.frame_time as f64)) as f32;

        // Figure out how many WHOLE frames to advance ( as an int )
        let frames_to_adv = a.frame_progress.floor();
        // Remove those from frame progress
        a.frame_progress -= frames_to_adv;
        
        // Prevent overflow
        if frames_to_adv > 255.0 { 
            s.id_offset = schema.root as i32 + (a.frame as i32);
            return; 
        } 

        let mut adv_frames = frames_to_adv as u8;

        while adv_frames > 0 {
             // Try to play forwards
            if !a.is_reversing {
                // Advance all frames, if possible
                if (a.frame as u32 + adv_frames as u32) < schema.frames as u32 {
                    a.frame += adv_frames;
                    break;
                } 
                // Not all frames could be advanced
                else{
                    // Calculate the number of valid frames and advance those
                    let valid_frames = (a.frame + adv_frames) - schema.frames;
                    adv_frames -= valid_frames;
                    a.frame += valid_frames;
                    // Note, there is AT LEAST one frame left now.

                    if reverses { a.is_reversing = true; }
                    else if loops {
                        // Return to the first frame again, costing one frame.
                        adv_frames -= 1;
                        a.frame = 0;
                    }
                    // Return to idle
                    else if schema.mode == AnimMode::OncePersist {
                        return;
                    }
                    else {
                        a.is_done = true;
                        return;
                    }
                }
            }

            // Otherwise, play in reverse
            else {
                // Reverse all the frames, if possible
                if a.frame >= adv_frames {
                    a.frame -= adv_frames;
                    s.id_offset -= adv_frames as i32;
                    break;
                }
                // Not all frames could be reversed
                else {
                    // Reverse back to 0
                    adv_frames -= a.frame;
                    a.frame = 0;
    
                    // Continue back in forward animation
                    if loops {
                        a.is_reversing = false;
                    }
                    // TODO ReversePersist?
                    else {
                        a.is_done = true;
                        return;
                    }
                }
            }
        }
    }
}
impl<'a> System<'a> for AnimSpriteSys {
    type SystemData = (Entities<'a>,
                       WriteStorage<'a, Sprite>,
                       WriteStorage<'a, Animation>,
                       Read<'a, DeltaTime>);
    // TODO allow for providing a string to the animation component and
    // have the system simply consume it to replace it with an animation
    fn run(&mut self, data: Self::SystemData) {
        let (ents, mut sprites, mut anims, dt) = data;
        let dt = dt.0;

        for (ent, mut s, mut a) in (&ents, &mut sprites, &mut anims).join() {
            if a.is_done { 
                let _ = ents.delete(ent); 
                continue; 
            }

            Self::advance_animation(&mut s, &mut a, dt); 
            // Apply the animation's offset, if it exists
            s.id_offset = match &a.schema {
                None => 0,
                // Calculate the difference between the animation root+frame and the schema root
                Some(schema) => (schema.root+(a.frame as u32)) as i32 - s.schema.root as i32,
            };
        }
    }
}

/// A system for rendering Sprites to the screen.
///
/// As this is an OpenGL System it must be called on the main thread via `with_tread_local`
#[derive(Default)]
pub struct SpriteRenderSys {
    renderer: SpriteRenderer,
}
impl<'a> System<'a> for SpriteRenderSys {
    type SystemData = (ReadStorage<'a, Sprite>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Scale>,
                       ReadStorage<'a, Color>,
                       Read<'a, WindowSize>,
                       Read<'a, View>);

    fn run(&mut self, data: Self::SystemData) {
        let (sprites, positions, scales, colors, window, view) = data;
        let window = (window.0, window.1); 
        let view = (view.0, view.1, view.2);
        // Build the RenderSprite Vec from the components
        let sprites: Vec<RenderSprite> = 
            (&sprites, &positions, &scales, &colors).join()
                .map(|data| data.into())
                .collect();
        self.renderer.render(&sprites, window, view);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.renderer = SpriteRenderer::new();
        let sheet = world.fetch::<SpritesheetPath>();
        let atlas_path = PathBuf::from(&sheet.0[..]);
        let mut atlas_file = std::fs::File::open(atlas_path)
            .expect("Spritesheet image could not be found alongside its definition");
        let mut atlas_data = Vec::new();
        atlas_file.read_to_end(&mut atlas_data).unwrap();
        self.renderer.init(&atlas_data[..]).unwrap();
    }
}

//TODO join renderers into a common resource (potentially using the resource system?)
pub struct TileRenderSys {
    renderer: SpriteRenderer,
    scale:  (f32, f32),
}
impl Default for TileRenderSys {
    fn default() -> Self {
        Self {
            renderer: SpriteRenderer::default(),
            scale: (5.0, 5.0),
        }
    }
}
impl<'a> System<'a> for TileRenderSys {
    type SystemData = (ReadStorage<'a, Tile>,
                       ReadStorage<'a, Floor>,
                       ReadStorage<'a, Wall>,
                       ReadStorage<'a, Color>,
                       Read<'a, WindowSize>,
                       Read<'a, View>);

    fn run(&mut self, data: Self::SystemData) {
        // Unpack system data
        let (tiles, floors, walls, colors, window, view) = data;
        let window = (window.0, window.1);
        let view = (view.0, view.1, view.2);
        let scale = self.scale.clone();
        let sprites: Vec<RenderSprite> = 
            (&tiles, &floors, &colors).join()
                .map(|data| {
                    let (tile, floor, color) = data;
                    RenderSprite::from((tile, color, floor.schema.clone(), scale, -10.1))
                })
                .collect();
        self.renderer.render(&sprites, window, view);

        let sprites: Vec<RenderSprite> = 
            (&tiles, &walls, &colors).join()
                .map(|data| {
                    let (tile, wall, color) = data;
                    RenderSprite::from((tile, color, wall.schema.clone(), scale, -10.0))
                })
                .collect();
        self.renderer.render(&sprites, window, view);
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.renderer = SpriteRenderer::new();
        self.renderer.init(include_bytes!("../../../../assets/textures/sprites.png")).unwrap();
    }
}
