use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::entity::Entity as EcsEntity; // NOTE: Is this a good idea?
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Res};
use bevy_hierarchy::BuildChildren;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::entity_definition::EntityDefinition;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::layer_definition::LayerDefinition;
use bevy_log::debug;
use bevy_math::Vec2;
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::component::{DoFinalizeEvent, LdtkComponent, LdtkComponentExt};
use crate::int_grid::IntGrid;
use crate::project_config::ProjectConfig;
use crate::tiles::Tiles;
use crate::{bad_handle, Result};

pub type Entity = LdtkComponent<EntityAsset>;

pub(crate) fn entity_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<DoFinalizeEvent<EntityAsset>>,
    entity_assets: Res<Assets<EntityAsset>>,
    config_assets: Res<Assets<ProjectConfig>>,
    entity_definitions: Res<Assets<EntityDefinition>>,
    query: Query<(EcsEntity, &Entity)>,
) -> Result<()> {
    todo!()
}
