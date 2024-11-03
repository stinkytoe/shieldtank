use bevy_app::{App, Plugin, Update};
use bevy_asset::AssetApp;
use bevy_ecs::system::IntoSystem;
use bevy_utils::error;

use crate::entity::Entity;
use crate::layer::{handle_layer_asset_modified, handle_layer_component_added, Layer};
use crate::level::{handle_level_asset_modified, handle_level_component_added, Level};
use crate::level_background::level_background_system;
use crate::project::{handle_project_component_added, Project};
use crate::project_config::{ProjectConfig, ProjectConfigLoader};
use crate::tiles::handle_tiles_system;
use crate::world::{handle_world_component_added, World};

pub struct ShieldtankPlugin;

impl Plugin for ShieldtankPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectConfig>()
            .init_asset_loader::<ProjectConfigLoader>()
            .register_asset_reflect::<ProjectConfig>()
            .register_type::<Project>()
            .register_type::<ProjectConfig>()
            .register_type::<World>()
            .register_type::<Level>()
            .register_type::<Layer>()
            .register_type::<Entity>()
            .add_systems(
                Update,
                (
                    //project
                    handle_project_component_added.map(error),
                    //world
                    handle_world_component_added.map(error),
                    //level
                    handle_level_component_added.map(error),
                    handle_level_asset_modified.map(error),
                    level_background_system.map(error),
                    //layer
                    handle_layer_component_added.map(error),
                    handle_layer_asset_modified.map(error),
                    handle_tiles_system.map(error),
                ),
            );
    }
}
