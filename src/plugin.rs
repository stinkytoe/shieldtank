use bevy_app::{App, Plugin};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_log::trace;

use crate::entity::EntityPlugin;
use crate::item::{LdtkAssetPlugin, LdtkChildSpawnerPlugin};
use crate::layer::LayerPlugin;
use crate::level::LevelPlugin;
use crate::level_background::LevelBackgroundPlugin;
use crate::project_config::ProjectConfigPlugin;
use crate::tiles::TilesPlugin;
use crate::tileset_rectangle::TilesetRectangleSystem;
use crate::world::WorldPlugin;

pub struct ShieldtankPlugin;

impl Plugin for ShieldtankPlugin {
    fn build(&self, app: &mut App) {
        trace!("Initializing ShieldtankPlugin!");
        app.add_plugins(BevyLdtkAssetPlugin)
            .add_plugins(EntityPlugin)
            .add_plugins(LayerPlugin)
            .add_plugins(LdtkAssetPlugin::<EntityAsset>::default())
            .add_plugins(LdtkAssetPlugin::<LayerAsset>::default())
            .add_plugins(LdtkAssetPlugin::<LevelAsset>::default())
            .add_plugins(LdtkAssetPlugin::<ProjectAsset>::default())
            .add_plugins(LdtkAssetPlugin::<WorldAsset>::default())
            .add_plugins(LdtkChildSpawnerPlugin::<LayerAsset, EntityAsset>::default())
            .add_plugins(LdtkChildSpawnerPlugin::<LevelAsset, LayerAsset>::default())
            .add_plugins(LdtkChildSpawnerPlugin::<ProjectAsset, WorldAsset>::default())
            .add_plugins(LdtkChildSpawnerPlugin::<WorldAsset, LevelAsset>::default())
            .add_plugins(LevelBackgroundPlugin)
            .add_plugins(LevelPlugin)
            .add_plugins(ProjectConfigPlugin)
            .add_plugins(TilesetRectangleSystem)
            .add_plugins(TilesPlugin)
            .add_plugins(WorldPlugin);
    }
}
