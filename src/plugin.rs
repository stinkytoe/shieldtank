use bevy::prelude::*;
use bevy::utils::error;

use crate::asset_loader::LdtkProjectLoader;
use crate::assets::entity::LdtkEntity;
use crate::assets::layer::LdtkLayer;
use crate::assets::level::LdtkLevel;
use crate::assets::project::LdtkProject;
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
            .register_asset_reflect::<LdtkProject>()
            .register_asset_reflect::<LdtkWorld>()
            .register_asset_reflect::<LdtkLevel>()
            .register_asset_reflect::<LdtkLayer>()
            .register_asset_reflect::<LdtkEntity>()
            .init_asset_loader::<LdtkProjectLoader>()
            .add_systems(Update, LdtkProject::asset_event_system.map(error));
    }
}
