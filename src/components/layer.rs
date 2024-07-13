use bevy::prelude::*;
use thiserror::Error;

use crate::{
    assets::{layer::LdtkLayer, traits::LdtkAsset},
    iid::Iid,
};

use super::traits::LdtkComponent;

#[derive(Debug, Error)]
pub(crate) enum LdtkLayerComponentError {}

#[derive(Component, Debug)]
pub(crate) struct LdtkLayerComponent {
    iid: Iid,
    project_entity: Entity,
}

impl LdtkLayerComponent {
    pub(crate) fn new(
        asset: &LdtkLayer,
        project_entity: Entity,
    ) -> Result<Self, LdtkLayerComponentError> {
        Ok(Self {
            iid: asset.iid(),
            project_entity,
        })
    }
}

impl LdtkComponent<LdtkLayer> for LdtkLayerComponent {
    fn iid(&self) -> Iid {
        self.iid
    }

    fn ldtk_asset_event_system(
        events: EventReader<crate::assets::event::LdtkAssetEvent<LdtkLayer>>,
        query: Query<(Entity, &Self)>,
        assets: Res<Assets<LdtkLayer>>,
    ) {
        todo!()
    }
}
