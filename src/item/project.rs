use bevy_ldtk_asset::project::Project as ProjectAsset;

use crate::component::project::ProjectComponentQueryData;
use crate::item::entity::EntityItem;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::world::WorldItem;
use crate::item::Item;

use super::macros::iter_descendents;

pub type ProjectItem<'w, 's> = Item<'w, 's, ProjectAsset, ProjectComponentQueryData<'w>>;

impl ProjectItem<'_, '_> {
    pub fn iter_entities(&self) -> impl Iterator<Item = EntityItem> {
        iter_descendents!(self, iter_entities, get_project)
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = LayerItem> {
        iter_descendents!(self, iter_layers, get_project)
    }

    pub fn iter_levels(&self) -> impl Iterator<Item = LevelItem> {
        iter_descendents!(self, iter_levels, get_project)
    }

    pub fn iter_worlds(&self) -> impl Iterator<Item = WorldItem> {
        iter_descendents!(self, iter_worlds, get_project)
    }
}
