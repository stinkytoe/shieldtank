use bevy::prelude::*;
use thiserror::Error;

use crate::ldtk;
use crate::util::bevy_color_from_ldtk;
use crate::util::ColorParseError;

#[derive(Debug, Default, Reflect)]
pub struct IntGridValueGroup {
    pub color: Option<Color>,
    pub identifier: Option<String>,
    pub uid: i64,
}

#[derive(Debug, Error)]
pub enum IntGridValueGroupFromError {
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
}

impl IntGridValueGroup {
    pub(crate) fn new(
        value: &ldtk::IntGridValueGroupDefinition,
    ) -> Result<Self, IntGridValueGroupFromError> {
        Ok(Self {
            color: match value.color.as_ref() {
                Some(color) => Some(bevy_color_from_ldtk(color)?),
                None => None,
            },
            identifier: value.identifier.clone(),
            uid: value.uid,
        })
    }
}
