pub mod plugin;
pub mod systems;

use bevy_ecs::world::Ref;
use bevy_ldtk_asset::level::Level as LevelAsset;

use crate::component::level::LevelComponentQueryData;
use crate::item::entity::EntityItem;
use crate::item::iter::recurrent_identifier::ItemRecurrentIdentifierIteratorExt as _;
use crate::item::layer::LayerItem;
use crate::item::macros::get_parent;
use crate::item::macros::{get_ancestor, iter_descendents};
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;
use crate::level_background::LevelBackground;

pub type LevelItem<'w, 's> = Item<'w, 's, LevelAsset, LevelComponentQueryData<'w>>;

impl LevelItem<'_, '_> {
    pub fn iter_entities(&self) -> impl Iterator<Item = EntityItem> {
        iter_descendents!(self, iter_entities, get_level)
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = LayerItem> {
        iter_descendents!(self, iter_layers, get_level)
    }

    pub fn get_world(&self) -> Option<WorldItem> {
        get_parent!(self, get_world)
    }

    pub fn get_project(&self) -> Option<ProjectItem> {
        get_ancestor!(self, get_world, get_project)
    }
}

impl LevelItem<'_, '_> {
    pub fn layer_by_identifier(&self, identifier: &'static str) -> Option<LayerItem> {
        let layer = self.iter_layers().filter_identifier(identifier).next();
        layer
    }
}

impl LevelItem<'_, '_> {
    pub fn get_level_background(&self) -> &Option<Ref<LevelBackground>> {
        &self.component_query_data
    }
}
