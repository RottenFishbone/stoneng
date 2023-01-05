#![allow(dead_code)]
use crate::EngineError;

use std::{
    path,
    sync::Arc,
    rc::Rc,
    collections::HashMap,
};

use serde::Deserialize;
use glm::{Vec2, Vec3, Vec4};

/// Defines a sprite sheet's individual sprite schemas.
#[derive(Deserialize, Debug)]
pub struct SpriteSheet {
    /// The path to the image file this data describes.
    #[serde(default)]
    img_ref:            Option<&'static [u8]>,
    /// Pixel width of the sprite sheet.
    pub sheet_width:    u32,
    /// Pixel width of a single sprite tile.
    pub tile_width:     u32,
    /// A map containing all the sprite definitions
    pub sprites:        HashMap<String, Arc<SpriteSchema>>,
}

impl SpriteSheet {

    /// Loads a spritesheet image and parses its Rusty Object Notation layout file.
    ///
    /// Note: the max sheet resolution is 255 tiles x 255 *tiles*
    ///
    /// # Example 
    /// ```
    /// # use stoneng::{model::spritesheet::SpriteSheet};
    /// // Using this file as example, normally an image would go here
    /// let img_data = include_bytes!("./spritesheet.rs");
    /// // i.e. let img_data = include_bytes("example.png");
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
    ///                 // `mode` being:
    ///                 // Once (self-deletes), OncePersist, Loop, LoopReverse, Reverse 
    ///                 "idle": (
    ///                     root:       3,
    ///                     frames:     3,
    ///                     fps:        5,
    ///                     mode:       Loop,
    ///                 ),
    ///             }
    ///         )
    ///     }
    /// )
    /// "#;
    /// let sheet = SpriteSheet::new(layout, img_data).unwrap();
    ///
    /// // Check the sheet contains valid entries
    /// assert_eq!(sheet.sheet_width, 256);
    /// assert_eq!(sheet.sprites["arch"].root, 9);
    /// assert!(sheet.sprites["water"].animations.contains_key("idle"));
    /// ```
    pub fn new(layout: &'static str, img_ref: &'static [u8]) -> Result<Self, EngineError> {
        let mut sheet = ron::from_str::<SpriteSheet>(layout)?;
        sheet.img_ref = Some(img_ref);

        if sheet.sheet_width / sheet.tile_width > 255 {
            return Err(EngineError::SheetSizeError("Maximum tiles per row is 255".into()));
        }
        return Ok(sheet);
    }

    pub fn img_ref(&self) -> &'static [u8] { self.img_ref.unwrap() }
}

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq)]
pub enum AnimMode {
    Once,
    OncePersist,
    Loop,
    LoopReverse,
    Reverse,
}
impl AnimMode { pub fn lowest() -> Self { AnimMode::OncePersist } }

/// A description of a sprite animation.
///
/// This is used to describe an animation when the renderer is handling
/// sprite animations.
#[derive(Deserialize, Debug, Clone)]
pub struct AnimationSchema {
    /// The position of the sprite animation's root tile.
    pub root:           u32,
    
    /// The number of unique tiles in the sprite's animation
    #[serde(default)]
    pub frames:         u8,
    
    #[serde(default = "AnimMode::lowest")]
    pub mode:          AnimMode,
    
    /// How many seconds between each frame 
    #[serde(default)]
    pub frame_time:     f32,
}
impl PartialEq for AnimationSchema {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && 
        self.frames == other.frames &&
        self.mode == other.mode
    }
}

/// A description of a specific sprite.
///
/// This is used as a reference for the renderer when it needs to render
/// a particular sprite.
#[derive(Deserialize, Debug, Clone)]
pub struct SpriteSchema {
    /// The bottom left SINGLE tile of the sprite
    pub root:           u32,

    /// Variants of the sprite, e.g. Brick, Mossy brick, Cracked Brick..
    ///
    /// At the moment these aren't compatible with animations.
    #[serde(default)]
    pub variants:       HashMap<String, Arc<SpriteSchema>>,
    
    /// How many tiles in each direction the sprite uses. The origin is the
    /// bottom left tile.
    #[serde(default)]
    pub dimensions:     (u8, u8),
    
    /// A map of animation schema that the sprite can use.
    #[serde(default)]
    pub animations:      HashMap<String, Arc<AnimationSchema>>,
}
