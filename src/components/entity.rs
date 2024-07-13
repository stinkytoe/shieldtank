use bevy::prelude::*;
use thiserror::Error;

use crate::{
    assets::{entity::LdtkEntity, traits::LdtkAsset},
    iid::Iid,
};

use super::traits::LdtkComponent;

#[derive(Debug, Error)]
pub(crate) enum LdtkEntityComponentError {}

#[derive(Component, Debug)]
pub(crate) struct LdtkEntityComponent {
    iid: Iid,
    project_entity: Entity,
}

impl LdtkEntityComponent {
    pub(crate) fn new(
        asset: &LdtkEntity,
        project_entity: Entity,
    ) -> Result<Self, LdtkEntityComponentError> {
        Ok(Self {
            iid: asset.iid(),
            project_entity,
        })
    }
}

impl LdtkComponent<LdtkEntity> for LdtkEntityComponent {
    fn iid(&self) -> Iid {
        self.iid
    }

    fn ldtk_asset_event_system(
        events: EventReader<crate::assets::event::LdtkAssetEvent<LdtkEntity>>,
        query: Query<(Entity, &Self)>,
        assets: Res<Assets<LdtkEntity>>,
    ) {
        todo!()
    }
}
