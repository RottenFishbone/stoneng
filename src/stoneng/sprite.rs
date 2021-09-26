#![allow(dead_code)]
use crate::EngineError;

use std::{
    path,
    rc::Rc,
    collections::HashMap,
};

use serde::Deserialize;
use glm::{Vec2, Vec3, Vec4};


// ====================== Sprite Sheet / Schema ===============================
/// Defines a sprite sheets individual sprite schemas.
#[derive(Deserialize, Debug)]
pub struct SpriteSheet {
    /// The path to the image file this data describes.
    #[serde(default)]
    path:               String,
    /// Pixel width of the sprite sheet.
    pub sheet_width:    u32,
    /// Pixel width of a single sprite tile.
    pub tile_width:     u32,
    /// A map containing all the sprite definitions
    pub sprites:        HashMap<String, Rc<SpriteSchema>>,
}

impl SpriteSheet {
    /// Takes a spritesheet layout in Rusty Object Notation, as well as a path to 
    /// the image that the layout describes.
    /// 
    /// The path to the image is not tested and will result in a read error on shader
    /// compilation if an invalid path is used.
    ///
    /// Note: the max sheet resolution is 255 tiles x 255 tiles
    ///
    /// # Example 
    /// ```
    /// # use stoneng::sprite::*;
    /// let layout = r#"
    /// SpriteSheet ( 
    ///     sheet_width:    256,
    ///     tile_width:     32,
    ///
    ///     // Note that sprites is a map (curly braces, string keys)
    ///     sprites: {
    ///         // Each sprite is a struct, root is required, other params
    ///         // use defaults. Additional parameters can be found under
    ///         // the struct SpriteSchema
    ///         "grass": (
    ///             root: 0,
    ///         ),
    ///         "arch": (
    ///             root: 9,
    ///             dimensions: (2,2),
    ///         ),
    ///         "water": (
    ///             root: 3,
    ///             // Note that animations is a map
    ///             animations: {
    ///                 // Each animation is the struct AnimationSchema
    ///                 "idle": (
    ///                     root:       3,
    ///                     frames:     3,
    ///                     fps:        5,
    ///                     loops:    true,
    ///                     reverses:   true,
    ///                 ),
    ///             }
    ///         )
    ///     }
    /// )
    /// "#;
    /// let sheet = SpriteSheet::from_string(
    ///                 layout.into(), 
    ///                 "path/to/img.png".into()
    ///             ).unwrap(); 
    ///
    /// // Check the sheet contains valid entries
    /// assert_eq!(sheet.sheet_width, 256);
    /// assert_eq!(sheet.sprites["arch"].root, 9);
    /// assert!(sheet.sprites["water"].animations.contains_key("idle"));
    /// ```
    pub fn from_string(layout: String, path_to_img: String) -> Result<Self, EngineError> {
        // Deserialize the layout
        let mut sheet = ron::from_str::<SpriteSheet>(&layout)?;
        // Store the img path
        sheet.path = path_to_img;
        if sheet.sheet_width / sheet.tile_width > 255 {
            return Err(EngineError::SheetSizeError("Maximum tiles per row is 255".into()));
        }
        Ok(sheet)   
    }

    /// Takes a path to a sprite sheet layout file and deserializes it into a SpriteSheet.
    ///
    /// The layout file should be saved alongside the spritesheet image. It should be in Rusty 
    /// Object Notation. An example can be found in SpriteSheet::from_string().
    ///
    /// Note: the max sheet resolution is 255 tiles x 255 tiles
    pub fn from_layout(path_to_layout: String) -> Result<Self, EngineError> {
        // Load the layout file
        let filepath = path::PathBuf::from(&path_to_layout);
        let layout_string = std::fs::read_to_string(&filepath)?; 

        // Find the image's path
        let mut img_path = filepath.clone();
        img_path.set_extension("png");
        let img_path_str = img_path.to_string_lossy().to_string();
        
        // Parse the layout_string
        SpriteSheet::from_string(layout_string, img_path_str)
    }
    
    pub fn get_img_path(&self) -> &str { &self.path[..] }
}


/// A description of a sprite animation.
///
/// This is used to describe an animation when the renderer is handling
/// sprite animations.
#[derive(Deserialize, Debug, Copy, Clone)]
pub struct AnimationSchema {
    /// The position of the sprite animation's root tile.
    pub root:           u16,
    
    /// The number of unique tiles in the sprite's animation
    #[serde(default)]
    pub frames:         u8,
    
    /// If the animation should loop on completion
    #[serde(default)]
    pub loops:        bool,
    
    /// If, on the final frame, the animation should play in reverse
    /// 
    /// This can be used in conjunction with looping.
    #[serde(default)]
    pub reverses:       bool,
    
    /// How fast to play the animation
    #[serde(default)]
    pub fps:            u32,
}

/// A description of a specific sprite.
///
/// This is used as a reference for the renderer when it needs to render
/// a particular sprite.
#[derive(Deserialize, Debug, Clone)]
pub struct SpriteSchema {
    /// The bottom left SINGLE tile of the sprite
    pub root:       u16,

    /// Variants of the sprite, e.g. Brick, Mossy brick, Cracked Brick..
    ///
    /// At the moment these aren't compatible with animations.
    #[serde(default)]
    pub variants:   u16,
    
    /// How many tiles in each direction the sprite uses. The origin is the
    /// bottom left tile.
    #[serde(default)]
    pub dimensions:       (u32, u32),
    
    /// A map of animation schema that the sprite can use.
    #[serde(default)]
    pub animations:      HashMap<String, Rc<AnimationSchema>>,
}

// ==========================/SpriteSheet======================================


// ========================== Sprite ==========================================
/// Contains all the data needed to render a sprite. Note, that if the schema
/// does not exist the sprite will render as the first sprite in the spritesheet.
///
/// Note, the anim_state and schema are boxed to allow for faster traversal over
/// the sprite_data when sending to the GPU.
#[repr(C)]
pub struct Sprite {
    // ======== Render Data =========
    // This data is sent to the GPU on a render call
    // DO NOT ADJUST ORDERING OR SIZES
    pub pos:        glm::Vec3,
    pub color:      glm::Vec4,
    pub scale:      glm::Vec2,
    pub rotation:   f32,
   
    /// These are set by the schema
    sprite_id:      u16,
    sprite_dims:    u8,

    sprite_flags:   u8,

    // ====== Logic Data ======== 
    // This data is not sent the GPU, merely jumped over
    schema:             Rc<SpriteSchema>,
    anim_state:         Option<Box<AnimState>>,
}

impl Sprite {
    /// Builds a sprite using provided attributes and specific sprite schema.
    ///
    /// # Example
    /// ```
    /// 
    /// ```
    pub fn new(pos: Vec3, color: Vec4, scale: Vec2, 
               rotation: f32, schema: Rc<SpriteSchema>) -> Self {
         
        Self {
            pos, color, scale, rotation,
            sprite_id:      schema.root as u16,
            sprite_flags:   0,
            sprite_dims:    (schema.dimensions.0 | (schema.dimensions.1 << 4)) as u8,
            anim_state:     None,
            schema:         schema.clone(),
        }
    }
    
    /// Return the dimensions of the sprite in tiles, NOT pixels
    pub fn get_dimensions(&self) -> (u8,u8) { 
        (self.sprite_dims & 0xF, self.sprite_dims >> 4)
    }
    
    /// Return the pointer to the schema that the sprite is using.
    pub fn get_schema(&self) -> Rc<SpriteSchema> { self.schema.clone() } 
    /// Assign a new schema to the sprite.
    ///
    /// Note, this resets the animation state.
    pub fn set_schema(&mut self, schema: Rc<SpriteSchema>) -> &mut Self {
        self.schema = schema.clone();
        self.anim_state = None;
        self.sprite_id = schema.root as u16;
        self.sprite_dims = (schema.dimensions.0 | 
                           (schema.dimensions.1 << 4)) as u8;
        self
    }

    pub fn to_animation(&mut self, animation: Option<Rc<AnimationSchema>>) {
        if let Some(anim_schema) = animation {
            self.sprite_id = anim_schema.root;
            self.anim_state = Some(Box::from(
                AnimState {
                    schema:             anim_schema.clone(),
                    frame_progress:     0.0,
                    frame:              0,
                    is_reversing:       false,
                }
            ));
        }
        // No animation, revert to root
        else {
            self.anim_state = None;
            self.sprite_id  = self.get_schema().root;
        }
    }

    /// Brings a sprite back to it's 'idle' animation, if it exists, otherwise 
    /// ends the animation and returns to root.
    pub fn to_idle_animation(&mut self) {
        // Try to return to idle
        let animations = &self.get_schema().animations;
        let idle = match animations.get("idle") {
            Some(anim) => Some(anim.clone()),
            None => None,
        };
        self.to_animation(idle);
    }

    /// Should be called every tick to advance the animations at the correct framerate.
    ///
    /// This is independent of the rendering framerate and as such at low FPS will
    /// result in skipped animations.
    pub fn advance_animation(&mut self, dt: f64) {
        // Ensure the sprite has an animation
        let mut anim_state = match &mut self.anim_state {
            Some(state) => state,
            None => return,
        };
        let anim_schema = anim_state.schema.clone();
        
        // Calculate how many frames need to be played this update
        anim_state.frame_progress += dt / (1.0/(anim_schema.fps as f64));
        // Separate the whole numbers
        let frames_to_adv = anim_state.frame_progress.floor();
        // Subtract the whole number to leave decimals for future progress
        anim_state.frame_progress -= frames_to_adv;      

        let mut adv_frames = frames_to_adv as u8; 
        while adv_frames > 0 {
            // Try to play forwards
            if !anim_state.is_reversing {
                // Advance all frames, if possible
                if anim_state.frame + adv_frames < anim_schema.frames {
                    anim_state.frame += adv_frames;
                    self.sprite_id += adv_frames as u16;
                    break;
                } 
                // Not all frames could be advanced
                else{
                    // Calculate the number of valid frames and advance those
                    let valid_frames = (anim_state.frame + adv_frames) - anim_schema.frames;
                    adv_frames -= valid_frames;
                    anim_state.frame += valid_frames;
                    self.sprite_id += valid_frames as u16;
                    // Note, there is AT LEAST one frame left now.

                    if anim_schema.reverses { anim_state.is_reversing = true; }
                    else if anim_schema.loops {
                        // Return to the first frame again, costing one frame.
                        adv_frames -= 1;
                        anim_state.frame = 0;
                        self.sprite_id = anim_schema.root;
                    }
                    // Return to idle
                    else {
                        self.to_idle_animation();
                        break;
                    }
                }
            }

            // Otherwise, play in reverse
            else {
                // Reverse all the frames, if possible
                if anim_state.frame >= adv_frames {
                    anim_state.frame -= adv_frames;
                    self.sprite_id -= adv_frames as u16;
                    break;
                }
                // Not all frames could be reversed
                else {
                    // Reverse back to 0
                    adv_frames -= anim_state.frame;
                    anim_state.frame -= anim_state.frame;
                    self.sprite_id -= anim_state.frame as u16;
    
                    // Continue back in forward animation
                    if anim_schema.loops {
                        anim_state.is_reversing = false;
                    }
                    // Return to idle
                    else {
                        self.to_idle_animation();
                        break;
                    }
                }
            }
        }
    }
}

pub struct AnimState {
    pub schema:             Rc<AnimationSchema>,
    pub frame_progress:     f64,
    pub frame:              u8,
    pub is_reversing:       bool,
}

