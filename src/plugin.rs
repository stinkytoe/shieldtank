use bevy_app::{App, Plugin, Update};
use bevy_asset::AssetApp;
use bevy_ecs::system::IntoSystem;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_utils::error;

use crate::component::{
    handle_ldtk_component_added, send_finalize_if_ready, AwaitingFinalize, DoFinalizeEvent,
};
use crate::entity::{entity_finalize_on_event, Entity};
use crate::layer::{layer_finalize_on_event, Layer};
use crate::level::{level_finalize_on_event, Level};
use crate::level_background::level_background_system;
use crate::project::{project_finalize_on_event, Project};
use crate::project_config::{ProjectConfig, ProjectConfigLoader};
use crate::tiles::handle_tiles_system;
use crate::tileset_rectangle::handle_tileset_rectangle_system;
use crate::world::{world_finalize_on_event, World};

pub struct ShieldtankPlugin;

impl Plugin for ShieldtankPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_plugins(BevyLdtkAssetPlugin)
            .init_asset::<ProjectConfig>()
            .init_asset_loader::<ProjectConfigLoader>()
            .insert_resource(AwaitingFinalize::default())
            .add_event::<DoFinalizeEvent<ProjectAsset>>()
            .add_event::<DoFinalizeEvent<WorldAsset>>()
            .add_event::<DoFinalizeEvent<LevelAsset>>()
            .add_event::<DoFinalizeEvent<LayerAsset>>()
            .add_event::<DoFinalizeEvent<EntityAsset>>()
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
                    handle_ldtk_component_added::<ProjectAsset>.map(error),
                    send_finalize_if_ready::<ProjectAsset>,
                    project_finalize_on_event.map(error),
                    //world
                    handle_ldtk_component_added::<WorldAsset>.map(error),
                    send_finalize_if_ready::<WorldAsset>,
                    world_finalize_on_event.map(error),
                    //level
                    handle_ldtk_component_added::<LevelAsset>.map(error),
                    send_finalize_if_ready::<LevelAsset>,
                    level_finalize_on_event.map(error),
                    //level_background
                    level_background_system.map(error),
                    //layer
                    handle_ldtk_component_added::<LayerAsset>.map(error),
                    send_finalize_if_ready::<LayerAsset>,
                    layer_finalize_on_event.map(error),
                    //layer tiles
                    handle_tiles_system.map(error),
                    //entity
                    handle_ldtk_component_added::<EntityAsset>.map(error),
                    send_finalize_if_ready::<EntityAsset>,
                    entity_finalize_on_event.map(error),
                    //tileset_rectangle
                    handle_tileset_rectangle_system.map(error),
                ),
            );
    }
}
