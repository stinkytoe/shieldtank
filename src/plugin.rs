use bevy_app::{App, Plugin, Update};
use bevy_asset::AssetApp;
use bevy_ecs::system::IntoSystem;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_log::trace;
use bevy_utils::error;

use crate::component::{AwaitingFinalize, FinalizeEvent};
use crate::entity::EntityPlugin;
use crate::item::{LdtkPlugin, LdtkPluginChildSpawner};
use crate::layer::LayerPlugin;
use crate::level::LevelPlugin;
use crate::level_background::level_background_system;
use crate::project_config::{ProjectConfig, ProjectConfigLoader};
use crate::tiles::handle_tiles_system;
use crate::tileset_rectangle::handle_tileset_rectangle_system;

pub struct ShieldtankPlugin;

impl Plugin for ShieldtankPlugin {
    fn build(&self, app: &mut App) {
        trace!("Initializing ShieldtankPlugin!");
        app.add_plugins(BevyLdtkAssetPlugin)
            .add_plugins(LdtkPlugin::<EntityAsset>::default())
            .add_plugins(LdtkPlugin::<LayerAsset>::default())
            .add_plugins(LdtkPlugin::<LevelAsset>::default())
            .add_plugins(LdtkPlugin::<WorldAsset>::default())
            .add_plugins(LdtkPlugin::<ProjectAsset>::default())
            .add_plugins(LdtkPluginChildSpawner::<LayerAsset, EntityAsset>::default())
            .add_plugins(LdtkPluginChildSpawner::<LevelAsset, LayerAsset>::default())
            .add_plugins(LdtkPluginChildSpawner::<WorldAsset, LevelAsset>::default())
            .add_plugins(LdtkPluginChildSpawner::<ProjectAsset, WorldAsset>::default())
            .add_plugins(EntityPlugin)
            .add_plugins(LayerPlugin)
            .add_plugins(LevelPlugin)
            .add_systems(Update, handle_tileset_rectangle_system.map(error))
            .add_systems(Update, handle_tiles_system.map(error))
            .add_systems(Update, level_background_system.map(error))
            .init_asset::<ProjectConfig>()
            .init_asset_loader::<ProjectConfigLoader>()
            .insert_resource(AwaitingFinalize::<EntityAsset>::default())
            .insert_resource(AwaitingFinalize::<LayerAsset>::default())
            .insert_resource(AwaitingFinalize::<LevelAsset>::default())
            .insert_resource(AwaitingFinalize::<WorldAsset>::default())
            .insert_resource(AwaitingFinalize::<ProjectAsset>::default())
            .add_event::<FinalizeEvent<ProjectAsset>>()
            .add_event::<FinalizeEvent<WorldAsset>>()
            .add_event::<FinalizeEvent<LevelAsset>>()
            .add_event::<FinalizeEvent<LayerAsset>>()
            .add_event::<FinalizeEvent<EntityAsset>>()
            .register_asset_reflect::<ProjectConfig>();
    }
}
