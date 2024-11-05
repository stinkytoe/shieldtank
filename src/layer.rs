use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Res};
use bevy_hierarchy::{BuildChildren, ChildBuild};
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::layer_definition::LayerDefinition;
use bevy_log::debug;
use bevy_math::Vec2;
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::component::{DoFinalizeEvent, LdtkComponent, LdtkComponentExt};
use crate::entity::Entity;
use crate::int_grid::IntGrid;
use crate::project_config::ProjectConfig;
use crate::tiles::Tiles;
use crate::{bad_handle, Result};

pub type Layer = LdtkComponent<LayerAsset>;

pub(crate) fn layer_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<DoFinalizeEvent<LayerAsset>>,
    layer_assets: Res<Assets<LayerAsset>>,
    config_assets: Res<Assets<ProjectConfig>>,
    layer_definitions: Res<Assets<LayerDefinition>>,
    query: Query<(EcsEntity, &Layer)>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let DoFinalizeEvent {
            entity: event_entity,
            ..
        } = event;

        query
            .iter()
            .filter(|(entity, ..)| entity == event_entity)
            .try_for_each(|data| -> Result<()> {
                finalize(
                    &mut commands,
                    data,
                    &layer_assets,
                    &config_assets,
                    &layer_definitions,
                )
            })
    })
}

fn finalize(
    commands: &mut Commands,
    (ecs_entity, layer): (EcsEntity, &Layer),
    layer_assets: &Assets<LayerAsset>,
    config_assets: &Assets<ProjectConfig>,
    layer_definitions: &Assets<LayerDefinition>,
) -> Result<()> {
    let layer_asset = layer_assets
        .get(layer.get_handle().id())
        .ok_or(bad_handle!(layer.get_handle()))?;

    let project_config = config_assets
        .get(layer.get_config_handle().id())
        .ok_or(bad_handle!(layer.get_config_handle()))?;

    let name = Name::from(layer_asset.identifier.clone());

    let transform = Transform::from_translation(
        Vec2::ZERO.extend(((layer_asset.index + 1) as f32) * project_config.layer_z_scale),
    );

    let visibility = Visibility::default();

    let mut entity_commands = commands.entity(ecs_entity);

    entity_commands.insert((name, transform, visibility));

    if let Some(entities_layer) = layer_asset.layer_type.get_entities_layer() {
        entity_commands.with_children(|parent| {
            entities_layer
                .entity_handles
                .values()
                .for_each(|entity_handle| {
                    parent.spawn(Entity {
                        handle: entity_handle.clone(),
                        config: layer.get_config_handle(),
                    });
                });
        });
    }

    if let Some(tiles_layer) = layer_asset.layer_type.get_tiles_layer() {
        // TODO: Unguarded Assets::get(..) here. Probably fine, but should fix.
        let layer_definition = layer_definitions
            .get(layer_asset.layer_definition.id())
            .ok_or(bad_handle!(layer_asset.layer_definition))?;
        let int_grid = IntGrid::from_layer(layer_asset, layer_definition)?;
        let tiles = Tiles::new(tiles_layer);
        entity_commands.insert((int_grid, tiles));
    }

    debug!("Layer {:?} finalized!", layer_asset.identifier);

    Ok(())
}
