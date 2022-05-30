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
    path:               String,
    /// Pixel width of the sprite sheet.
    pub sheet_width:    u32,
    /// Pixel width of a single sprite tile.
    pub tile_width:     u32,
    /// A map containing all the sprite definitions
    pub sprites:        HashMap<String, Arc<SpriteSchema>>,
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
    pub loops:          bool,
    
    /// If, on the final frame, the animation should play in reverse
    /// 
    /// This can be used in conjunction with looping.
    #[serde(default)]
    pub reverses:       bool,
    
    /// How many seconds between each frame 
    #[serde(default)]
    pub frame_time:     f32,
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
    pub dimensions:       (u8, u8),
    
    /// A map of animation schema that the sprite can use.
    #[serde(default)]
    pub animations:      HashMap<String, Arc<AnimationSchema>>,
}
impl SpriteSchema {
    pub fn idle_anim(&self) -> Option<Arc<AnimationSchema>> {
        match self.animations.get("idle") {
            Some(idle) => Some(idle.clone()),
            None => None,
        }
    }
}

