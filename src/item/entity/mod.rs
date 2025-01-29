pub mod iter;
pub mod plugin;
pub mod systems;

use bevy_ecs::world::Ref;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_math::Vec2;
use bevy_sprite::Sprite;

use crate::component::entity::EntityComponentQueryData;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::macros::get_ancestor;
use crate::item::macros::get_parent;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;
use crate::tileset_rectangle::TilesetRectangle;

pub type EntityItem<'w, 's> = Item<'w, 's, EntityAsset, EntityComponentQueryData<'w>>;

impl EntityItem<'_, '_> {
    pub fn get_layer(&self) -> Option<LayerItem> {
        get_parent!(self, get_layer)
    }

    pub fn get_level(&self) -> Option<LevelItem> {
        get_ancestor!(self, get_layer, get_level)
    }

    pub fn get_world(&self) -> Option<WorldItem> {
        get_ancestor!(self, get_level, get_world)
    }

    pub fn get_project(&self) -> Option<ProjectItem> {
        get_ancestor!(self, get_world, get_project)
    }
}

impl EntityItem<'_, '_> {
    pub fn get_tileset_rectangle(&self) -> &Option<Ref<TilesetRectangle>> {
        &self.component_query_data.0
    }

    pub fn get_sprite(&self) -> &Option<Ref<Sprite>> {
        &self.component_query_data.1
    }
}

impl EntityItem<'_, '_> {
    pub fn level_location(&self) -> Vec2 {
        let layer_location = self.location();

        let layer_offset = self
            .get_layer()
            .map(|item| item.location())
            // TODO: Is this how we want to handle missing ancestors?
            .unwrap_or(Vec2::ZERO);

        layer_location + layer_offset
    }

    pub fn world_location(&self) -> Vec2 {
        let level_location = self.level_location();

        let level_offset = self
            .get_level()
            .map(|item| item.location())
            // TODO: Is this how we want to handle missing ancestors?
            .unwrap_or(Vec2::ZERO);

        level_location + level_offset
    }

    pub fn project_location(&self) -> Vec2 {
        let world_location = self.world_location();

        let world_offset = self
            .get_world()
            .map(|item| item.location())
            // TODO: Is this how we want to handle missing ancestors?
            .unwrap_or(Vec2::ZERO);

        world_location + world_offset
    }
}

impl EntityItem<'_, '_> {
    pub fn has_tag(&self, tag: &str) -> bool {
        self.get_asset()
            .tags
            .iter()
            .any(|inner_tag| inner_tag == tag)
    }
}
