use bevy_app::{Plugin, PostUpdate};
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::world::Ref;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_sprite::Sprite;

use crate::commands::ShieldtankCommands;
use crate::component::level::LevelComponentQueryData;
use crate::item::entity::EntityItem;
use crate::item::iter::ItemIteratorExt;
use crate::item::layer::LayerItem;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;
use crate::level_background::LevelBackground;
use crate::query::ShieldtankQuery;

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
    pub fn get_level_background(&self) -> &Option<Ref<LevelBackground>> {
        &self.component_query_data.0
    }

    pub fn get_sprite(&self) -> &Option<Ref<Sprite>> {
        &self.component_query_data.1
    }
}

pub trait LevelItemIteratorExt<'w, 's>
where
    Self: Iterator<Item = LevelItem<'w, 's>> + Sized,
    's: 'w,
{
    fn filter_level_background_changed(self) -> impl Iterator<Item = LevelItem<'w, 's>> {
        self.filter(|item| {
            item.get_level_background()
                .as_ref()
                .and_then(|level_background| level_background.is_changed().then_some(()))
                .is_some()
        })
    }
}

impl<'w, 's, I: Iterator<Item = LevelItem<'w, 's>>> LevelItemIteratorExt<'w, 's> for I where 's: 'w {}

fn level_spawn_system(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    shieldtank_query
        .iter_levels()
        .filter_just_finalized()
        .for_each(|item| {
            let asset_handle = item.get_asset_handle();
            let config = item.get_config();
            if config
                .get_load_level_background_pattern()
                .handle_matches_pattern(&asset_handle)
            {
                let asset = item.get_asset();
                let level_background = LevelBackground::new(asset);
                shieldtank_commands
                    .level(&item)
                    .insert_level_background(level_background);
            }
        });
}

pub struct LevelItemPlugin;
impl Plugin for LevelItemPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(PostUpdate, level_spawn_system);
    }
}
