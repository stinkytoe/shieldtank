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

impl LdtkComponent<LdtkLayer, ()> for LdtkLayerComponent {
    fn iid(&self) -> Iid {
        self.iid
    }

    fn project_entity(&self) -> Entity {
        self.project_entity
    }

    fn asset_handle_from_project(
        project: &crate::prelude::LdtkProject,
        iid: Iid,
    ) -> Option<Handle<LdtkLayer>> {
        project.layers.get(&iid).cloned()
    }

    fn on_spawn(
        commands: &mut Commands,
        entity: Entity,
        project: &crate::prelude::LdtkProject,
        asset: &LdtkLayer,
        component: &Self,
        component_set_query: &Query<()>,
    ) -> Result<(), super::traits::LdtkComponentError<LdtkLayer>> {
        Ok(())
    }
}
