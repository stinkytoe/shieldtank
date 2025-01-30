pub mod iter;
pub mod plugin;
pub mod systems;

use bevy_ecs::world::Ref;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::prelude::LdtkAssetWithFieldInstances;
use bevy_math::Rect;
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

    pub fn get_field_tile(&self, identifier: &str) -> Option<TilesetRectangle> {
        self.get_asset()
            .get_field_instance(identifier)?
            .get_tile()
            .map(|value| TilesetRectangle::new(value.clone()))
    }

    pub fn get_field_array_tiles(&self, identifier: &str) -> Option<Vec<TilesetRectangle>> {
        self.get_asset()
            .get_field_instance(identifier)?
            .get_array_tile()
            .map(|value| {
                value
                    .iter()
                    .map(|value| TilesetRectangle::new(value.clone()))
                    .collect()
            })
    }
}

impl EntityItem<'_, '_> {
    pub fn get_region(&self) -> Rect {
        let anchor = self.get_asset().anchor.as_vec();
        let size = self.get_asset().size.as_vec2();

        let p0 = Vec2::new(-anchor.x - 0.5, -anchor.y + 0.5) * size;
        let p1 = p0 + size * Vec2::new(1.0, -1.0);

        Rect::from_corners(p0, p1)
    }

    pub fn location_in_region(&self, location: Vec2) -> bool {
        let offset = location - self.location();
        self.get_region().contains(offset)
    }

    pub fn level_location_in_region(&self, level_location: Vec2) -> bool {
        let offset = level_location - self.level_location();
        self.get_region().contains(offset)
    }

    pub fn world_location_in_region(&self, world_location: Vec2) -> bool {
        let offset = world_location - self.world_location();
        self.get_region().contains(offset)
    }
}
