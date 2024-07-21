use bevy::prelude::*;
use bevy::utils::error;

use crate::asset_loader::LdtkProjectLoader;
use crate::assets::entity::LdtkEntityAsset;
use crate::assets::event::LdkAssetEvent;
use crate::assets::layer::LdtkLayerAsset;
use crate::assets::level::LdtkLevelAsset;
use crate::assets::project::LdtkProject;
use crate::assets::traits::LdtkAsset;
use crate::assets::world::LdtkWorldAsset;

pub struct ShieldTankPlugin;

impl Plugin for ShieldTankPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<LdtkProject>()
            .init_asset::<LdtkWorldAsset>()
            .init_asset::<LdtkLevelAsset>()
            .init_asset::<LdtkLayerAsset>()
            .init_asset::<LdtkEntityAsset>()
            .add_event::<LdkAssetEvent<LdtkWorldAsset>>()
            .add_event::<LdkAssetEvent<LdtkLevelAsset>>()
            .add_event::<LdkAssetEvent<LdtkLayerAsset>>()
            .add_event::<LdkAssetEvent<LdtkEntityAsset>>()
            .register_asset_reflect::<LdtkProject>()
            .register_asset_reflect::<LdtkWorldAsset>()
            .register_asset_reflect::<LdtkLevelAsset>()
            .register_asset_reflect::<LdtkLayerAsset>()
            .register_asset_reflect::<LdtkEntityAsset>()
            .register_type::<Image>()
            .register_type::<Handle<Image>>()
            .init_asset_loader::<LdtkProjectLoader>()
            .add_systems(
                Update,
                (
                    LdtkProject::asset_event_system.map(error),
                    LdtkWorldAsset::ldtk_asset_event_system,
                    LdtkLevelAsset::ldtk_asset_event_system,
                    LdtkLevelAsset::level_background_system.map(error),
                    LdtkLayerAsset::ldtk_asset_event_system,
                    LdtkLayerAsset::layer_image_system.map(error),
                    LdtkEntityAsset::ldtk_asset_event_system,
                    LdtkEntityAsset::entity_tile_system.map(error),
                    LdtkEntityAsset::handle_tile_system.map(error),
                ),
            );
    }
}
