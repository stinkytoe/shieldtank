use bevy::prelude::*;
use thiserror::Error;

use crate::assets::layer::LdtkLayer;

#[derive(Debug, Error)]
pub(crate) enum LdtkLayerComponentError {}

#[derive(Component, Debug)]
pub(crate) struct LdtkLayerComponent {}

impl LdtkLayerComponent {
    pub(crate) fn new(asset: &LdtkLayer) -> Result<Self, LdtkLayerComponentError> {
        Ok(Self {})
    }
}
