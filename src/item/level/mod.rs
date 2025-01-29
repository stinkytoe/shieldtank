pub mod plugin;
pub mod systems;

use bevy_ecs::world::Ref;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_math::Vec2;

use crate::component::level::LevelComponentQueryData;
use crate::int_grid::IntGridValue;
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

impl LevelItem<'_, '_> {
    pub fn int_grid_at(&self, location: Vec2) -> Option<IntGridValue> {
        let mut layers_rev_z_order: Vec<_> = self.iter_layers().collect();

        layers_rev_z_order.sort_by(|item_a, item_b| {
            item_b
                .get_transform()
                .translation
                .z
                .partial_cmp(&item_a.get_transform().translation.z)
                .expect("partial_cmp failed in int_grid_at")
        });

        layers_rev_z_order.iter().find_map(|item| {
            let layer_location = item.location();

            item.int_grid_at(location - layer_location)
        })
    }
}
