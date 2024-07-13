use bevy::{prelude::*, reflect::Map};
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

impl LdtkComponent<LdtkLevel, ()> for LdtkLevelComponent {
    fn iid(&self) -> Iid {
        self.iid
    }

    fn project_entity(&self) -> Entity {
        self.project_entity
    }

    fn asset_handle_from_project(
        project: &crate::prelude::LdtkProject,
        iid: Iid,
    ) -> Option<Handle<LdtkLevel>> {
        project.levels.get(&iid).cloned()
    }

    fn on_spawn(
        commands: &mut Commands,
        entity: Entity,
        project: &crate::prelude::LdtkProject,
        asset: &LdtkLevel,
        component: &Self,
        component_set_query: &Query<()>,
    ) -> Result<(), super::traits::LdtkComponentError<LdtkLevel>> {
        Ok(())
    }
}
