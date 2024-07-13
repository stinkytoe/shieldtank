use bevy::ecs::component::StorageType;
use bevy::prelude::*;
use thiserror::Error;

use crate::assets::event::LdtkAssetEvent;
use crate::assets::traits::LdtkAsset;
use crate::assets::world::LdtkWorld;
use crate::iid::Iid;

use super::traits::LdtkComponent;

#[derive(Debug, Error)]
pub(crate) enum LdtkWorldComponentError {}

#[derive(Component, Debug)]
pub(crate) struct LdtkWorldComponent {
    iid: Iid,
    project_entity: Entity,
}

impl LdtkWorldComponent {
    pub(crate) fn new(
        asset: &LdtkWorld,
        project_entity: Entity,
    ) -> Result<Self, LdtkWorldComponentError> {
        Ok(Self {
            iid: asset.iid(),
            project_entity,
        })
    }

    pub(crate) fn iid(&self) -> Iid {
        self.iid
    }
}

impl LdtkComponent<LdtkWorld> for LdtkWorldComponent {
    fn ldtk_asset_event_system(
        mut events: EventReader<LdtkAssetEvent<LdtkWorld>>,
        query: Query<(Entity, &Self)>,
        assets: Res<Assets<LdtkWorld>>,
    ) {
        for event in events.read() {
            debug!("LdtkAssetEvent<LdtkWorld>: {event:?}");
        }
    }

    fn iid(&self) -> Iid {
        self.iid
    }
}
