use bevy::prelude::*;

use crate::ldtk;

#[derive(Debug, Default, Reflect)]
pub struct EnumTagValue {
    pub(crate) enum_value_id: String,
    pub(crate) tile_ids: Vec<i64>,
}

impl EnumTagValue {
    pub(crate) fn new(value: &ldtk::EnumTagValue) -> Self {
        Self {
            enum_value_id: value.enum_value_id.clone(),
            tile_ids: value.tile_ids.clone(),
        }
    }
}
