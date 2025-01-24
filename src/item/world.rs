use bevy_ldtk_asset::world::World as WorldAsset;

use crate::component::world::WorldComponentQueryData;

use super::{project::ProjectItem, Item};

pub type WorldItem<'w, 's> = Item<'w, 's, WorldAsset, WorldComponentQueryData<'w>>;

impl WorldItem<'_, '_> {}

impl WorldItem<'_, '_> {
    pub fn get_project(&self) -> Option<ProjectItem> {
        self.get_parent_component()
            .as_ref()
            .and_then(|parent| self.get_query().get_project(parent.get()).ok())
    }
}
