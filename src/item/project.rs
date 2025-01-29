use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_math::Vec2;

use crate::component::project::ProjectComponentQueryData;
use crate::int_grid::IntGridValue;
use crate::item::entity::EntityItem;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::macros::iter_descendents;
use crate::item::world::WorldItem;
use crate::item::Item;

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

impl ProjectItem<'_, '_> {
    pub fn int_grid_at(&self, location: Vec2) -> Option<IntGridValue> {
        let mut world_rev_z_order: Vec<_> = self.iter_worlds().collect();

        world_rev_z_order.sort_by(|item_a, item_b| {
            item_b
                .get_transform()
                .translation
                .z
                .partial_cmp(&item_a.get_transform().translation.z)
                .expect("partial_cmp failed in int_grid_at")
        });

        world_rev_z_order.iter().find_map(|item| {
            let world_location = item.location();

            item.int_grid_at(location - world_location)
        })
    }
}
