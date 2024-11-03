use bevy::asset::{AssetEvent, Assets, Handle};
use bevy::core::Name;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::log::debug;
use bevy::math::Vec2;
use bevy::prelude::Added;
use bevy::reflect::Reflect;
use bevy::render::view::Visibility;
use bevy::transform::components::Transform;
use bevy_ldtk_asset::prelude::ldtk_asset;

use crate::automations::{IntGridAutomation, TilesAutomation};
use crate::int_grid::IntGrid;
use crate::project_config::ProjectConfig;
use crate::tiles::Tiles;
use crate::{bad_handle, Result};

#[derive(Component, Debug, Default, Reflect)]
pub struct Layer {
    pub handle: Handle<ldtk_asset::Layer>,
    pub config: Handle<ProjectConfig>,
}

// ## Layer
//  - Name
//  -- from identifier
//  -- Only on new, and if not present
//  -- if changed, then asset path changed also and is now a different asset
//
//  - Visibility
//  -- always visible
//  -- Only on new, and if not present
//
//  - Transform
//  -- always translation (0,0,0)
//  -- Only on new, and if not present
//
//  - Tiles
//  -- only for layers with tiles
//  -- delete if no tiles/changed to entity layer
//  -- from asset
//  -- always update
//  -- systems use this to draw layer
//
//  - IntGrid
//  -- Only for TilesLayer with IntGrids
//  -- from asset
//  -- use IntGridAutomation to determine if we manage or not
#[allow(clippy::type_complexity)]
pub(crate) fn handle_layer_component_added(
    mut commands: Commands,
    assets: Res<Assets<ldtk_asset::Layer>>,
    definitions: Res<Assets<ldtk_asset::LayerDefinition>>,
    configs: Res<Assets<ProjectConfig>>,
    query: Query<
        (
            Entity,
            &Layer,
            Option<&Name>,
            Option<&Transform>,
            Option<&IntGrid>,
            Option<&Tiles>,
        ),
        Added<Layer>,
    >,
) -> Result<()> {
    query.iter().try_for_each(
        |(entity, layer, name, transform, int_grid, tiles)| -> Result<()> {
            let asset = assets
                .get(layer.handle.id())
                .ok_or(bad_handle!(layer.handle))?;

            let config = configs
                .get(layer.config.id())
                .ok_or(bad_handle!(layer.config))?;

            if name.is_none() {
                let name = asset.identifier.clone();
                commands.entity(entity).insert(Name::new(name));
            }

            if transform.is_none() {
                commands
                    .entity(entity)
                    .insert(Transform::default().with_translation(
                        Vec2::ZERO.extend(((asset.index + 1) as f32) * config.layer_z_scale),
                    ));
            }

            commands.entity(entity).insert(Visibility::default());

            if int_grid.is_none() && asset.layer_type.is_tiles_layer() {
                let layer_definition = definitions
                    .get(asset.layer_definition.id())
                    .ok_or(bad_handle!(asset.layer_definition))?;
                let int_grid = IntGrid::from_layer(asset, layer_definition)?;
                if !int_grid.is_empty() {
                    commands
                        .entity(entity)
                        .insert((int_grid, IntGridAutomation));
                    debug!("IntGrid layer added for layer {}!", asset.identifier);
                }
            }

            if tiles.is_none() {
                if let Some(tiles_layer) = asset.layer_type.get_tiles_layer() {
                    let tiles = Tiles::new(tiles_layer);
                    commands.entity(entity).insert((tiles, TilesAutomation));
                    debug!("Tiles layer added for layer {}!", asset.identifier);
                }
            }

            debug!("Layer entity added and set up! {entity:?}");
            Ok(())
        },
    )
}

pub(crate) fn handle_layer_asset_modified(
    mut commands: Commands,
    mut asset_events: EventReader<AssetEvent<ldtk_asset::Layer>>,
    assets: Res<Assets<ldtk_asset::Layer>>,
    definitions: Res<Assets<ldtk_asset::LayerDefinition>>,
    query: Query<(
        Entity,
        &Layer,
        Option<&IntGridAutomation>,
        Option<&TilesAutomation>,
    )>,
) -> Result<()> {
    asset_events.read().try_for_each(|event| -> Result<()> {
        let AssetEvent::Modified { id } = event else {
            return Ok(());
        };

        debug!("layer modified!");

        query
            .iter()
            .filter(|(_, layer, ..)| layer.handle.id() == *id)
            .try_for_each(
                |(entity, layer, int_grid_automation, tiles_automation)| -> Result<()> {
                    let asset = assets
                        .get(layer.handle.id())
                        .ok_or(bad_handle!(layer.handle))?;

                    if int_grid_automation.is_some() {
                        let layer_definition = definitions
                            .get(asset.layer_definition.id())
                            .ok_or(bad_handle!(asset.layer_definition))?;

                        let int_grid = IntGrid::from_layer(asset, layer_definition)?;

                        commands
                            .entity(entity)
                            .insert((int_grid, IntGridAutomation));
                    }

                    if tiles_automation.is_some() {
                        if let Some(tiles_layer) = asset.layer_type.get_tiles_layer() {
                            let tiles = Tiles::new(tiles_layer);

                            commands.entity(entity).insert((tiles, TilesAutomation));
                        }
                    }

                    Ok(())
                },
            )
    })
}
