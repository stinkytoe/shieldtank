use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_math::Vec2;

use crate::component::world::WorldComponentQueryData;
use crate::int_grid::IntGridValue;
use crate::item::entity::EntityItem;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::macros::get_parent;
use crate::item::macros::iter_descendents;
use crate::item::project::ProjectItem;
use crate::item::Item;

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

impl WorldItem<'_, '_> {
    pub fn int_grid_at(&self, location: Vec2) -> Option<IntGridValue> {
        let mut levels_rev_z_order: Vec<_> = self.iter_levels().collect();

        levels_rev_z_order.sort_by(|item_a, item_b| {
            item_b
                .get_transform()
                .translation
                .z
                .partial_cmp(&item_a.get_transform().translation.z)
                .expect("partial_cmp failed in int_grid_at")
        });

        levels_rev_z_order.iter().find_map(|item| {
            let level_location = item.location();

            item.int_grid_at(location - level_location)
        })
    }
}
