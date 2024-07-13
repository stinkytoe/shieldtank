use bevy::prelude::*;
use thiserror::Error;

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

impl LdtkComponent<LdtkWorld, ()> for LdtkWorldComponent {
    fn iid(&self) -> Iid {
        self.iid
    }

    fn project_entity(&self) -> Entity {
        self.project_entity
    }

    fn asset_handle_from_project(
        project: &crate::prelude::LdtkProject,
        iid: Iid,
    ) -> Option<Handle<LdtkWorld>> {
        project.worlds.get(&iid).cloned()
    }

    fn on_spawn(
        commands: &mut Commands,
        entity: Entity,
        project: &crate::prelude::LdtkProject,
        asset: &LdtkWorld,
        component: &Self,
        component_set_query: &Query<()>,
    ) -> Result<(), super::traits::LdtkComponentError<LdtkWorld>> {
        Ok(())
    }
}
