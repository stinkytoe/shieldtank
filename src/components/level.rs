use bevy::prelude::*;
use thiserror::Error;

use crate::assets::level::LdtkLevel;

#[derive(Debug, Error)]
pub(crate) enum LdtkLevelComponentError {}

#[derive(Component, Debug)]
pub(crate) struct LdtkLevelComponent {}

impl LdtkLevelComponent {
    pub(crate) fn new(asset: &LdtkLevel) -> Result<Self, LdtkLevelComponentError> {
        Ok(Self {})
    }
}
