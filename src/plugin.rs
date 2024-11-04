use bevy_app::{App, Plugin, Update};
use bevy_asset::AssetApp;
use bevy_ecs::system::IntoSystem;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_utils::error;

use crate::component::{
    handle_project_component_added, send_finalize_if_ready, AwaitingFinalize, DoFinalizeEvent,
};
use crate::entity::Entity;
use crate::layer::Layer;
use crate::level::Level;
use crate::project::{finalize_on_event, Project};
use crate::project_config::{ProjectConfig, ProjectConfigLoader};
use crate::world::World;

pub struct ShieldtankPlugin;

impl Plugin for ShieldtankPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_asset::<ProjectConfig>()
            .init_asset_loader::<ProjectConfigLoader>()
            .insert_resource(AwaitingFinalize::default())
            .add_event::<DoFinalizeEvent<ProjectAsset>>()
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
                    handle_project_component_added::<ProjectAsset>.map(error),
                    send_finalize_if_ready::<ProjectAsset>,
                    finalize_on_event.map(error),
                ),
            );
    }
}
