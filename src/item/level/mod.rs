pub mod iter;
pub mod plugin;
pub mod systems;

use bevy_ecs::world::Ref;
use bevy_ldtk_asset::level::Level as LevelAsset;

use crate::component::level::LevelComponentQueryData;
use crate::item::entity::EntityItem;
use crate::item::iter::recurrent_identifier::ItemRecurrentIdentifierIteratorExt as _;
use crate::item::layer::LayerItem;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;
use crate::level_background::LevelBackground;

pub type LevelItem<'w, 's> = Item<'w, 's, LevelAsset, LevelComponentQueryData<'w>>;

impl LevelItem<'_, '_> {
    pub fn iter_entities(&self) -> impl Iterator<Item = EntityItem> {
        self.get_query()
            .iter_entities()
            .filter(|item| item.get_level().as_ref() == Some(self))
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = LayerItem> {
        self.get_query()
            .iter_layers()
            .filter(|item| item.get_level().as_ref() == Some(self))
    }

    pub fn get_world(&self) -> Option<WorldItem> {
        self.get_parent_component()
            .as_ref()
            .and_then(|parent| self.get_query().get_world(parent.get()).ok())
    }

    pub fn get_project(&self) -> Option<ProjectItem> {
        let world = self.get_world()?;

        self.get_query().get_project(world.get_ecs_entity()).ok()
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
