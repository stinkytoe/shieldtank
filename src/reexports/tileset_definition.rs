use bevy::math::I64Vec2;
use bevy::prelude::*;

use crate::ldtk;
use crate::reexports::enum_tag_value::EnumTagValue;
use crate::reexports::tile_custom_metadata::TileCustomMetadata;

#[derive(Debug, Default, Reflect)]
pub struct TilesetDefinition {
    pub(crate) grid_size: I64Vec2,
    pub(crate) custom_data: Vec<TileCustomMetadata>,
    // FIXME: embedAtlas not currently supported!
    pub(crate) enum_tags: Vec<EnumTagValue>,
    pub(crate) identifier: String,
    pub(crate) padding: i64,
    pub(crate) size: Vec2,
    pub(crate) rel_path: Option<String>,
    pub(crate) spacing: i64,
    pub(crate) tags: Vec<String>,
    pub(crate) tile_grid_size: i64,
    pub(crate) uid: i64,
}

impl TilesetDefinition {
    pub(crate) fn new(value: &ldtk::TilesetDefinition) -> Self {
        Self {
            grid_size: (value.c_wid, value.c_hei).into(),
            custom_data: value
                .custom_data
                .iter()
                .map(TileCustomMetadata::new)
                .collect(),
            enum_tags: value.enum_tags.iter().map(EnumTagValue::new).collect(),
            identifier: value.identifier.clone(),
            padding: value.padding,
            size: (value.px_wid as f32, value.px_hei as f32).into(),
            rel_path: value.rel_path.clone(),
            spacing: value.spacing,
            tags: value.tags.clone(),
            tile_grid_size: value.tile_grid_size,
            uid: value.uid,
        }
    }
}
