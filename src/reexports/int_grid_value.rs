use bevy::prelude::*;
use thiserror::Error;

use crate::ldtk;
use crate::reexports::tileset_rectangle::TilesetRectangle;
use crate::util::bevy_color_from_ldtk;
use crate::util::ColorParseError;

#[derive(Clone, Debug, Default, Reflect)]
pub struct IntGridValue {
    pub color: Color,
    pub group_uid: i64,
    pub identifier: Option<String>,
    pub tile: Option<TilesetRectangle>,
    pub value: i64,
}

#[derive(Debug, Error)]
pub enum IntGridValueFromError {
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
}

impl IntGridValue {
    pub(crate) fn new(value: &ldtk::IntGridValueDefinition) -> Result<Self, IntGridValueFromError> {
        Ok(Self {
            color: bevy_color_from_ldtk(&value.color)?,
            group_uid: value.group_uid,
            identifier: value.identifier.clone(),
            tile: value.tile.as_ref().map(TilesetRectangle::new),
            value: value.value,
        })
    }
}
