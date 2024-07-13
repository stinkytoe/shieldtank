use bevy::prelude::*;
use thiserror::Error;

use crate::{
    assets::{level::LdtkLevel, traits::LdtkAsset},
    iid::Iid,
};

use super::traits::LdtkComponent;

#[derive(Debug, Error)]
pub(crate) enum LdtkLevelComponentError {}

#[derive(Component, Debug)]
pub(crate) struct LdtkLevelComponent {
    iid: Iid,
    project_entity: Entity,
}

impl LdtkLevelComponent {
    pub(crate) fn new(
        asset: &LdtkLevel,
        project_entity: Entity,
    ) -> Result<Self, LdtkLevelComponentError> {
        Ok(Self {
            iid: asset.iid(),
            project_entity,
        })
    }
}

impl LdtkComponent<LdtkLevel> for LdtkLevelComponent {
    fn iid(&self) -> Iid {
        self.iid
    }

    fn ldtk_asset_event_system(
        events: EventReader<crate::assets::event::LdtkAssetEvent<LdtkLevel>>,
        query: Query<(Entity, &Self)>,
        assets: Res<Assets<LdtkLevel>>,
    ) {
        todo!()
    }
}
