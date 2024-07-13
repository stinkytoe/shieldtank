use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::assets::entity::LdtkEntity;
use crate::assets::layer::LdtkLayer;
use crate::assets::level::LdtkLevel;
use crate::assets::traits::LdtkAsset;
use crate::assets::world::LdtkWorld;
use crate::assets::world::LdtkWorldError;
use crate::components::entity::LdtkEntityComponent;
use crate::components::entity::LdtkEntityComponentError;
use crate::components::layer::LdtkLayerComponent;
use crate::components::layer::LdtkLayerComponentError;
use crate::components::level::LdtkLevelComponent;
use crate::components::level::LdtkLevelComponentError;
use crate::components::traits::LdtkComponent;
use crate::components::world::LdtkWorldComponent;
use crate::components::world::LdtkWorldComponentError;
use crate::iid::IidMap;

#[derive(Debug, Error)]
pub enum LdtkProjectError {
    #[error(transparent)]
    LdtkWorldError(#[from] LdtkWorldError),
    #[error(transparent)]
    LdtkWorldComponentError(#[from] LdtkWorldComponentError),
    #[error(transparent)]
    LdtkLevelComponentError(#[from] LdtkLevelComponentError),
    #[error(transparent)]
    LdtkLayerComponentError(#[from] LdtkLayerComponentError),
    #[error(transparent)]
    LdtkEntityComponentError(#[from] LdtkEntityComponentError),
    #[error("Stub")]
    Stub,
}

#[derive(Clone, Debug, Deserialize, Reflect, Serialize, Default)]
pub struct LdtkProjectSettings {}

#[derive(Debug, Asset, Reflect)]
pub struct LdtkProject {
    pub(crate) settings: LdtkProjectSettings,
    pub(crate) worlds: IidMap<Handle<LdtkWorld>>,
    pub(crate) levels: IidMap<Handle<LdtkLevel>>,
    pub(crate) layers: IidMap<Handle<LdtkLayer>>,
    pub(crate) entities: IidMap<Handle<LdtkEntity>>,
}

impl LdtkProject {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn asset_event_system(
        mut commands: Commands,
        mut asset_events: EventReader<AssetEvent<LdtkProject>>,
        projects: Res<Assets<LdtkProject>>,
        project_query: Query<(Entity, &Handle<LdtkProject>)>,
        worlds: Res<Assets<LdtkWorld>>,
        world_query: Query<(Entity, &LdtkWorldComponent)>,
        levels: Res<Assets<LdtkLevel>>,
        level_query: Query<(Entity, &LdtkLevelComponent)>,
        layers: Res<Assets<LdtkLayer>>,
        layer_query: Query<(Entity, &LdtkLayerComponent)>,
        entities: Res<Assets<LdtkEntity>>,
        entity_query: Query<(Entity, &LdtkEntityComponent)>,
    ) -> Result<(), LdtkProjectError> {
        for asset_event in asset_events.read() {
            match asset_event {
                AssetEvent::Added { id } => {
                    trace!("LdtkProject Added: {id:?}");
                    let Some((project_entity, project_handle)) =
                        project_query.iter().find(|(_, handle)| handle.id() == *id)
                    else {
                        return Err(LdtkProjectError::Stub);
                    };

                    let project_asset =
                        projects.get(project_handle).ok_or(LdtkProjectError::Stub)?;

                    project_children(
                        &mut commands,
                        &worlds,
                        &world_query,
                        &levels,
                        &level_query,
                        &layers,
                        &layer_query,
                        &entities,
                        &entity_query,
                        project_asset,
                        project_entity,
                    )?;
                }
                AssetEvent::Modified { id } => {
                    trace!("LdtkProject Modified: {id:?}");
                }
                AssetEvent::Removed { id } => {
                    trace!("LdtkProject Removed: {id:?}");
                }
                AssetEvent::Unused { id } => {
                    trace!("LdtkProject Unused: {id:?}");
                }
                AssetEvent::LoadedWithDependencies { id } => {
                    trace!("LdtkProject LoadedWithDependencies: {id:?}");
                }
            }
        }

        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn project_children(
    commands: &mut Commands,
    worlds: &Assets<LdtkWorld>,
    world_query: &Query<(Entity, &LdtkWorldComponent)>,
    levels: &Assets<LdtkLevel>,
    level_query: &Query<(Entity, &LdtkLevelComponent)>,
    layers: &Assets<LdtkLayer>,
    layer_query: &Query<(Entity, &LdtkLayerComponent)>,
    entities: &Assets<LdtkEntity>,
    entity_query: &Query<(Entity, &LdtkEntityComponent)>,
    project_asset: &LdtkProject,
    project_entity: Entity,
) -> Result<(), LdtkProjectError> {
    project_asset
        .worlds
        .values()
        .map(|world_handle| -> Result<Entity, LdtkProjectError> {
            let world_asset = worlds.get(world_handle).ok_or(LdtkProjectError::Stub)?;

            let world_entity = if let Some((world_entity, _)) = world_query
                .iter()
                .find(|(_, inner_handle)| world_asset.iid() == inner_handle.iid())
            {
                world_entity
            } else {
                trace!("spawning LdtkWorldComponent: {}", world_asset.iid());
                commands
                    .spawn((
                        LdtkWorldComponent::new(world_asset, project_entity)?,
                        SpatialBundle::default(),
                    ))
                    .id()
            };

            world_asset
                .children()
                .iter()
                .map(|level_iid| -> Result<Entity, LdtkProjectError> {
                    let level_handle = project_asset
                        .levels
                        .get(level_iid)
                        .ok_or(LdtkProjectError::Stub)?;

                    let level_asset = levels.get(level_handle).ok_or(LdtkProjectError::Stub)?;

                    let level_entity = if let Some((level_entity, _)) = level_query
                        .iter()
                        .find(|(_, inner_handle)| level_asset.iid() == inner_handle.iid())
                    {
                        level_entity
                    } else {
                        trace!("spawning LdtkLevelComponent: {}", level_asset.iid());
                        commands
                            .spawn((
                                LdtkLevelComponent::new(level_asset, project_entity)?,
                                SpatialBundle::default(),
                            ))
                            .id()
                    };

                    level_asset
                        .children()
                        .iter()
                        .map(|layer_iid| -> Result<Entity, LdtkProjectError> {
                            let layer_handle = project_asset
                                .layers
                                .get(layer_iid)
                                .ok_or(LdtkProjectError::Stub)?;

                            let layer_asset =
                                layers.get(layer_handle).ok_or(LdtkProjectError::Stub)?;

                            let layer_entity = if let Some((layer_entity, _)) =
                                layer_query.iter().find(|(_, inner_component)| {
                                    layer_asset.iid() == inner_component.iid()
                                }) {
                                layer_entity
                            } else {
                                trace!("spawning LdtkLayerComponent: {}", layer_asset.iid());
                                commands
                                    .spawn((
                                        LdtkLayerComponent::new(layer_asset, project_entity)?,
                                        SpatialBundle::default(),
                                    ))
                                    .id()
                            };

                            layer_asset
                                .children()
                                .iter()
                                .map(|entity_iid| -> Result<Entity, LdtkProjectError> {
                                    let entity_handle = project_asset
                                        .entities
                                        .get(entity_iid)
                                        .ok_or(LdtkProjectError::Stub)?;

                                    let entity_asset = entities
                                        .get(entity_handle)
                                        .ok_or(LdtkProjectError::Stub)?;

                                    let entity_entity = if let Some((entity_entity, _)) =
                                        entity_query.iter().find(|(_, inner_component)| {
                                            layer_asset.iid() == inner_component.iid()
                                        }) {
                                        entity_entity
                                    } else {
                                        trace!(
                                            "spawning LdtkEntityComponent: {}",
                                            entity_asset.iid()
                                        );
                                        commands
                                            .spawn((
                                                LdtkEntityComponent::new(
                                                    entity_asset,
                                                    project_entity,
                                                )?,
                                                SpatialBundle::default(),
                                            ))
                                            .id()
                                    };

                                    Ok(entity_entity)
                                })
                                .collect::<Result<Vec<Entity>, LdtkProjectError>>()?
                                .iter()
                                .for_each(|&entity_entity| {
                                    commands.entity(layer_entity).add_child(entity_entity);
                                });

                            Ok(layer_entity)
                        })
                        .collect::<Result<Vec<Entity>, LdtkProjectError>>()?
                        .iter()
                        .for_each(|&layer_entity| {
                            commands.entity(level_entity).add_child(layer_entity);
                        });

                    Ok(level_entity)
                })
                .collect::<Result<Vec<Entity>, LdtkProjectError>>()?
                .iter()
                .for_each(|&level_entity| {
                    commands.entity(world_entity).add_child(level_entity);
                });

            Ok(world_entity)
        })
        .collect::<Result<Vec<Entity>, LdtkProjectError>>()?
        .iter()
        .for_each(|&world_entity| {
            commands.entity(project_entity).add_child(world_entity);
        });
    Ok(())
}
