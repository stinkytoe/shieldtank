use bevy::prelude::*;
use bevy::utils::error;

use crate::asset_loader::LdtkProjectLoader;
use crate::assets::entity::LdtkEntity;
use crate::assets::event::LdkAssetEvent;
use crate::assets::layer::LdtkLayer;
use crate::assets::level::LdtkLevel;
use crate::assets::project::LdtkProject;
use crate::assets::traits::LdtkAsset;
use crate::assets::world::LdtkWorld;

pub struct ShieldTankPlugin;

impl Plugin for ShieldTankPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<LdtkProject>()
            .init_asset::<LdtkWorld>()
            .init_asset::<LdtkLevel>()
            .init_asset::<LdtkLayer>()
            .init_asset::<LdtkEntity>()
            .add_event::<LdkAssetEvent<LdtkWorld>>()
            .add_event::<LdkAssetEvent<LdtkLevel>>()
            .add_event::<LdkAssetEvent<LdtkLayer>>()
            .add_event::<LdkAssetEvent<LdtkEntity>>()
            .register_asset_reflect::<LdtkProject>()
            .register_asset_reflect::<LdtkWorld>()
            .register_asset_reflect::<LdtkLevel>()
            .register_asset_reflect::<LdtkLayer>()
            .register_asset_reflect::<LdtkEntity>()
            .register_type::<Image>()
            .register_type::<Handle<Image>>()
            .init_asset_loader::<LdtkProjectLoader>()
            .add_systems(
                Update,
                (
                    LdtkProject::asset_event_system.map(error),
                    LdtkWorld::ldtk_asset_event_system,
                    LdtkLevel::ldtk_asset_event_system,
                    LdtkLevel::level_background_system.map(error),
                    LdtkLayer::ldtk_asset_event_system,
                    LdtkLayer::layer_image_system.map(error),
                    LdtkEntity::ldtk_asset_event_system,
                ),
            );
    }
}
