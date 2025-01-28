pub mod iter;
pub mod plugin;
pub mod systems;

use bevy_ecs::world::Ref;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_sprite::Sprite;

use crate::component::entity::EntityComponentQueryData;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;
use crate::tileset_rectangle::TilesetRectangle;

pub type EntityItem<'w, 's> = Item<'w, 's, EntityAsset, EntityComponentQueryData<'w>>;

impl EntityItem<'_, '_> {
    pub fn get_layer(&self) -> Option<LayerItem> {
        self.get_parent_component()
            .as_ref()
            .and_then(|parent| self.get_query().get_layer(parent.get()).ok())
    }

    pub fn get_level(&self) -> Option<LevelItem> {
        let layer = self.get_layer()?;

        let parent = layer.get_parent_component().as_ref()?.get();

        self.get_query().get_level(parent).ok()
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

impl EntityItem<'_, '_> {
    pub fn get_tileset_rectangle(&self) -> &Option<Ref<TilesetRectangle>> {
        &self.component_query_data.0
    }

    pub fn get_sprite(&self) -> &Option<Ref<Sprite>> {
        &self.component_query_data.1
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
