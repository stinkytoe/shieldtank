use bevy::{prelude::*, reflect::Map};
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

impl LdtkComponent<LdtkEntity, ()> for LdtkEntityComponent {
    fn iid(&self) -> Iid {
        self.iid
    }

    fn project_entity(&self) -> Entity {
        self.project_entity
    }

    fn asset_handle_from_project(
        project: &crate::prelude::LdtkProject,
        iid: Iid,
    ) -> Option<Handle<LdtkEntity>> {
        project.entities.get(&iid).cloned()
    }

    fn on_spawn(
        commands: &mut Commands,
        entity: Entity,
        project: &crate::prelude::LdtkProject,
        asset: &LdtkEntity,
        component: &Self,
        component_set_query: &Query<()>,
    ) -> Result<(), super::traits::LdtkComponentError<LdtkEntity>> {
        Ok(())
    }
}
