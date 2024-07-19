use bevy::prelude::*;

use crate::ldtk;

#[derive(Debug, Default, Reflect)]
pub struct TileCustomMetadata {
    pub(crate) data: String,
    pub(crate) tile_id: i64,
}

impl TileCustomMetadata {
    pub(crate) fn new(value: &ldtk::TileCustomMetadata) -> Self {
        Self {
            data: value.data.clone(),
            tile_id: value.tile_id,
        }
    }
}
