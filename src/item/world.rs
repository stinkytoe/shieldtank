use bevy_ldtk_asset::world::World as WorldAsset;

use crate::component::world::WorldComponentQueryData;
use crate::item::entity::EntityItem;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::macros::get_parent;
use crate::item::project::ProjectItem;
use crate::item::Item;

use super::macros::iter_descendents;

pub type WorldItem<'w, 's> = Item<'w, 's, WorldAsset, WorldComponentQueryData<'w>>;

impl WorldItem<'_, '_> {
    pub fn iter_entities(&self) -> impl Iterator<Item = EntityItem> {
        iter_descendents!(self, iter_entities, get_world)
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = LayerItem> {
        iter_descendents!(self, iter_layers, get_world)
    }

    pub fn iter_levels(&self) -> impl Iterator<Item = LevelItem> {
        iter_descendents!(self, iter_levels, get_world)
    }

    pub fn get_project(&self) -> Option<ProjectItem> {
        get_parent!(self, get_project)
    }
}
