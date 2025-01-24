use bevy_ldtk_asset::world::World as WorldAsset;

use crate::component::world::WorldComponentQueryData;
use crate::item::entity::EntityItem;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::project::ProjectItem;
use crate::item::Item;

pub type WorldItem<'w, 's> = Item<'w, 's, WorldAsset, WorldComponentQueryData<'w>>;

impl WorldItem<'_, '_> {
    pub fn iter_entities(&self) -> impl Iterator<Item = EntityItem> {
        self.get_query()
            .iter_entities()
            .filter(|item| item.get_world().as_ref() == Some(self))
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = LayerItem> {
        self.get_query()
            .iter_layers()
            .filter(|item| item.get_world().as_ref() == Some(self))
    }

    pub fn iter_levels(&self) -> impl Iterator<Item = LevelItem> {
        self.get_query()
            .iter_levels()
            .filter(|item| item.get_world().as_ref() == Some(self))
    }

    pub fn get_project(&self) -> Option<ProjectItem> {
        self.get_parent_component()
            .as_ref()
            .and_then(|parent| self.get_query().get_project(parent.get()).ok())
    }
}
