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
use crate::level::{handle_level_component_added, Level};
use crate::project::{handle_project_component_added, Project};
use crate::project_config::{ProjectConfig, ProjectConfigLoader};
use crate::world::{handle_world_component_added, World};

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
                    handle_project_component_added.map(dbg),
                    handle_world_component_added.map(dbg),
                    handle_level_component_added.map(dbg),
                    handle_project_load_children.map(dbg),
                    handle_world_load_children.map(dbg),
                    handle_level_load_children.map(dbg),
                    handle_layer_load_children.map(dbg),
                ),
            );
    }
}
