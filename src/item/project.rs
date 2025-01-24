use bevy_ldtk_asset::project::Project as ProjectAsset;

use crate::component::project::ProjectComponentQueryData;
use crate::item::entity::EntityItem;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::world::WorldItem;
use crate::item::Item;

pub type ProjectItem<'w, 's> = Item<'w, 's, ProjectAsset, ProjectComponentQueryData<'w>>;

impl ProjectItem<'_, '_> {
    pub fn iter_entities(&self) -> impl Iterator<Item = EntityItem> {
        self.get_query()
            .iter_entities()
            .filter(|item| item.get_project().as_ref() == Some(self))
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = LayerItem> {
        self.get_query()
            .iter_layers()
            .filter(|item| item.get_project().as_ref() == Some(self))
    }

    pub fn iter_levels(&self) -> impl Iterator<Item = LevelItem> {
        self.get_query()
            .iter_levels()
            .filter(|item| item.get_project().as_ref() == Some(self))
    }

    pub fn iter_worlds(&self) -> impl Iterator<Item = WorldItem> {
        self.get_query()
            .iter_worlds()
            .filter(|item| item.get_project().as_ref() == Some(self))
    }
}
