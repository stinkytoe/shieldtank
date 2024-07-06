use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::iid::IidMap;

use crate::assets::entity::LdtkEntity;
use crate::assets::entity::LdtkEntityError;
use crate::assets::layer::LdtkLayer;
use crate::assets::layer::LdtkLayerError;
use crate::assets::level::LdtkLevel;
use crate::assets::level::LdtkLevelError;
use crate::assets::world::LdtkWorld;
use crate::assets::world::LdtkWorldError;

#[derive(Debug, Error)]
pub enum LdtkProjectError {}

#[derive(Debug, Deserialize, Reflect, Serialize)]
pub struct LdtkProjectSettings {}

impl Default for LdtkProjectSettings {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Asset, Reflect)]
pub struct LdtkProject {
    settings: LdtkProjectSettings,
    worlds: IidMap<Handle<LdtkWorld>>,
    levels: IidMap<Handle<LdtkLevel>>,
    layers: IidMap<Handle<LdtkLayer>>,
    entities: IidMap<Handle<LdtkEntity>>,
}

impl LdtkProject {
    pub(crate) fn asset_event_system(
        mut asset_events: EventReader<AssetEvent<LdtkProject>>,
    ) -> Result<(), LdtkProjectError> {
        Ok(())
    }
}
