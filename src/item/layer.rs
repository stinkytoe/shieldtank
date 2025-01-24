use bevy_ldtk_asset::layer::Layer as LayerAsset;

use crate::component::layer::LayerComponentQueryData;
use crate::item::entity::EntityItem;
use crate::item::level::LevelItem;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;

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

        self.get_query().get_world(level.get_ecs_entity()).ok()
    }

    pub fn get_project(&self) -> Option<ProjectItem> {
        let world = self.get_world()?;

        self.get_query().get_project(world.get_ecs_entity()).ok()
    }
}
