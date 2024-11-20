use bevy_app::PluginGroup;
use bevy_app::PluginGroupBuilder;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_ldtk_asset::world::World as WorldAsset;

use crate::child_spawner::LdtkChildSpawnerPlugin;
use crate::entity::EntityPlugin;
use crate::item::LdtkAssetPlugin;
use crate::layer::LayerPlugin;
use crate::level::LevelPlugin;
use crate::level_background::LevelBackgroundPlugin;
use crate::project_config::ProjectConfigPlugin;
use crate::tiles::TilesPlugin;
use crate::tileset_rectangle::TilesetRectangleSystem;
use crate::world::WorldPlugin;

pub struct ShieldtankPlugins;

impl PluginGroup for ShieldtankPlugins {
    fn build(self) -> PluginGroupBuilder {
        let builder = PluginGroupBuilder::start::<Self>()
            .add(BevyLdtkAssetPlugin)
            .add(EntityPlugin)
            .add(LayerPlugin)
            .add(LdtkAssetPlugin::<EntityAsset>::default())
            .add(LdtkAssetPlugin::<LayerAsset>::default())
            .add(LdtkAssetPlugin::<LevelAsset>::default())
            .add(LdtkAssetPlugin::<ProjectAsset>::default())
            .add(LdtkAssetPlugin::<WorldAsset>::default())
            .add(LdtkChildSpawnerPlugin::<LayerAsset, EntityAsset>::default())
            .add(LdtkChildSpawnerPlugin::<LevelAsset, LayerAsset>::default())
            .add(LdtkChildSpawnerPlugin::<ProjectAsset, WorldAsset>::default())
            .add(LdtkChildSpawnerPlugin::<WorldAsset, LevelAsset>::default())
            .add(LevelBackgroundPlugin)
            .add(LevelPlugin)
            .add(ProjectConfigPlugin)
            .add(TilesetRectangleSystem)
            .add(TilesPlugin)
            .add(WorldPlugin);

        #[cfg(feature = "gridvania_toolkit")]
        let builder = {
            use crate::gridvania::GridvaniaPlugin;
            builder.add(GridvaniaPlugin)
        };

        builder
    }
}
