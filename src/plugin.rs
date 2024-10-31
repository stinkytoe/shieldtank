use bevy::app::{Plugin, Update};
use bevy::asset::AssetApp;
use bevy::ecs::system::IntoSystem;
use bevy::utils::dbg;

use crate::children_spawn::handle_layer_load_children;
use crate::children_spawn::handle_level_load_children;
use crate::children_spawn::handle_project_load_children;
use crate::children_spawn::handle_world_load_children;
use crate::entity::Entity;
use crate::layer::Layer;
use crate::level::Level;
use crate::project::{handle_project_asset_events, Project};
use crate::project_config::{ProjectConfig, ProjectConfigLoader};
use crate::world::{handle_world_asset_events, World};

pub struct ShieldtankPlugin;

impl Plugin for ShieldtankPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app //
            .init_asset::<ProjectConfig>()
            .init_asset_loader::<ProjectConfigLoader>()
            .register_type::<Project>()
            .register_type::<ProjectConfig>()
            .register_type::<World>()
            .register_type::<Level>()
            .register_type::<Layer>()
            .register_type::<Entity>()
            .add_systems(
                Update,
                (
                    handle_project_asset_events.map(dbg),
                    handle_world_asset_events.map(dbg),
                    handle_project_load_children.map(dbg),
                    handle_world_load_children.map(dbg),
                    handle_level_load_children.map(dbg),
                    handle_layer_load_children.map(dbg),
                ),
            );
    }
}
