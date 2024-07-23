use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use thiserror::Error;

use crate::assets::layer::LdtkLayerAsset;

#[derive(Debug, Error)]
pub enum LdtkEntityError {
    #[error(transparent)]
    QueryEntityError(#[from] QueryEntityError),
    #[error("bad layer handle? {0:?}")]
    BadLayerHandle(Handle<LdtkLayerAsset>),
    #[error("could not query parent as a layer")]
    NoLayerParent,
    #[error("could not query parent as a level")]
    NoLevelParent,
}
