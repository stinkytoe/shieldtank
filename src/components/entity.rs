use bevy::prelude::*;
use thiserror::Error;

use crate::assets::entity::LdtkEntity;

#[derive(Debug, Error)]
pub(crate) enum LdtkEntityComponentError {}

#[derive(Component, Debug)]
pub(crate) struct LdtkEntityComponent {}

impl LdtkEntityComponent {
    pub(crate) fn new(asset: &LdtkEntity) -> Result<Self, LdtkEntityComponentError> {
        Ok(Self {})
    }
}
