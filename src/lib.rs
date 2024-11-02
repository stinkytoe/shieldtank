#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub mod automations;
pub mod children_spawn;
pub mod entity;
pub mod error;
pub mod int_grid;
pub mod layer;
pub mod level;
pub mod level_background;
pub mod load_pattern;
pub mod plugin;
pub mod project;
pub mod project_config;
pub mod world;

pub use error::{Error, Result};

//
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
