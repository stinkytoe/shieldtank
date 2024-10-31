#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub mod children_spawn;
pub mod entity;
pub mod error;
pub mod layer;
pub mod level;
pub mod load_pattern;
pub mod plugin;
pub mod project;
pub mod project_config;
pub mod world;

pub use error::{Error, Result};

// ## Level
//  - Name
//  -- from identifier
//  -- Only on new, and if not present
//  -- if changed, then asset path changed also and is now a different asset
//
//  - Visibility
//  -- always visible
//  -- Only on new, and if not present
//
//  - Transform
//  -- Depends on WorldLayout
//  --- Free or GridVania: from asset: location, world_depth TODO: We need to establish a scale factor for calculating z
//  --- LinearHorizontal or LinearVertical: TODO: What to do here?
//  -- Only on new, and if not present
//
//  - LevelBackground
//  -- from asset
//  -- always update
//  -- systems use this to draw background
//
// ## Layer
//  - Name
//  -- from identifier
//  -- Only on new, and if not present
//  -- if changed, then asset path changed also and is now a different asset
//
//  - Visibility
//  -- always visible
//  -- Only on new, and if not present
//
//  - Transform
//  -- always translation (0,0,0)
//  -- Only on new, and if not present
//
//  - Tiles
//  -- only for layers with tiles
//  -- delete if no tiles/changed to entity layer
//  -- from asset
//  -- always update
//  -- systems use this to draw layer
//
// ## Entity
//  - Name
//  -- from identifier
//  -- Only on new, and if not present
//  -- if changed, then asset path changed also and is now a different asset
//
//  - Visibility
//  -- always visible
//  -- Only on new, and if not present
//
//  - Transform
//  -- Use location from asset;
//  -- Only on new, and if not present
//
//  - TilesetRectangle
//  -- from asset, if present
//  -- always update
//  -- systems use this to draw entity
