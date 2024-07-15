use bevy::prelude::*;

use crate::ldtk;

#[derive(Clone, Component, Debug, Reflect)]
pub struct TilesetRectangle {
    pub location: Vec2,
    pub size: Vec2,
    pub tileset_uid: i64,
}

impl TilesetRectangle {
    pub(crate) fn new(value: &ldtk::TilesetRectangle) -> Self {
        Self {
            location: (value.x as f32, value.y as f32).into(),
            size: (value.w as f32, value.h as f32).into(),
            tileset_uid: value.tileset_uid,
        }
    }
}
