use bevy::prelude::*;
use thiserror::Error;

use crate::assets::world::LdtkWorld;

#[derive(Debug, Error)]
pub(crate) enum LdtkWorldComponentError {}

#[derive(Component, Debug)]
pub(crate) struct LdtkWorldComponent {}

impl LdtkWorldComponent {
    pub(crate) fn new(asset: &LdtkWorld) -> Result<Self, LdtkWorldComponentError> {
        Ok(Self {})
    }
}
