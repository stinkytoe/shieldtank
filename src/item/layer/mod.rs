pub mod iter;
pub mod plugin;
pub mod systems;

use bevy_ecs::world::Ref;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_math::Vec2;

use crate::component::layer::LayerComponentQueryData;
use crate::int_grid::{IntGrid, IntGridValue};
use crate::item::entity::EntityItem;
use crate::item::level::LevelItem;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;
use crate::tiles::Tiles;

pub type LayerItem<'w, 's> = Item<'w, 's, LayerAsset, LayerComponentQueryData<'w>>;

impl LayerItem<'_, '_> {
    pub fn iter_entities(&self) -> impl Iterator<Item = EntityItem> {
        self.get_query()
            .iter_entities()
            .filter(|item| item.get_layer().as_ref() == Some(self))
    }

    pub fn get_level(&self) -> Option<LevelItem> {
        self.get_parent_component()
            .as_ref()
            .and_then(|parent| self.get_query().get_level(parent.get()).ok())
    }

    pub fn get_world(&self) -> Option<WorldItem> {
        let level = self.get_level()?;

        let parent = level.get_parent_component().as_ref()?.get();

        self.get_query().get_world(parent).ok()
    }

    pub fn get_project(&self) -> Option<ProjectItem> {
        let world = self.get_world()?;

        let parent = world.get_parent_component().as_ref()?.get();

        self.get_query().get_project(parent).ok()
    }
}

impl LayerItem<'_, '_> {
    pub fn get_int_grid(&self) -> &Option<Ref<IntGrid>> {
        &self.component_query_data.0
    }

    pub fn get_tiles(&self) -> &Option<Ref<Tiles>> {
        &self.component_query_data.1
    }
}

impl LayerItem<'_, '_> {
    pub fn is_tiles_layer(&self) -> bool {
        self.get_asset().layer_type.is_tiles_layer()
    }

    pub fn is_entities_layer(&self) -> bool {
        self.get_asset().layer_type.is_entities_layer()
    }

    pub fn int_grid_at(&self, location: Vec2) -> Option<&IntGridValue> {
        let grid_cell_size = self.get_asset().grid_cell_size as f32;
        let location = (Vec2::new(1.0, -1.0) * location / grid_cell_size).as_i64vec2();

        self.get_int_grid()
            .as_ref()
            .and_then(|int_grid| int_grid.get(location))
    }
}
