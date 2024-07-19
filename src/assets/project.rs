use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use thiserror::Error;

use crate::assets::entity::LdtkEntity;
use crate::assets::layer::LdtkLayer;
use crate::assets::level::LdtkLevel;
use crate::assets::traits::LdtkAsset;
use crate::assets::traits::LdtkAssetError;
use crate::assets::world::LdtkWorld;
use crate::assets::world::LdtkWorldError;
use crate::iid::IidMap;
use crate::iid::IidSet;
use crate::prelude::Iid;
use crate::reexports::tileset_definition::TilesetDefinition;

use super::event::LdkAssetEvent;

#[derive(Debug, Error)]
pub enum LdtkProjectError {
    #[error(transparent)]
    LdtkWorldError(#[from] LdtkWorldError),
    #[error(transparent)]
    LdtkAssetWorldError(#[from] LdtkAssetError<LdtkWorld>),
    #[error(transparent)]
    LdtkAssetLevelError(#[from] LdtkAssetError<LdtkLevel>),
    #[error(transparent)]
    LdtkAssetLayerError(#[from] LdtkAssetError<LdtkLayer>),
    #[error(transparent)]
    LdtkAssetEntityError(#[from] LdtkAssetError<LdtkEntity>),
    #[error("Bad project handle?")]
    BadProjectHandle,
    #[error("Bad world handle?")]
    BadWorldHandle(Handle<LdtkWorld>),
    #[error("Bad level handle?")]
    BadLevelHandle(Handle<LdtkLevel>),
    #[error("Bad layer handle?")]
    BadLayerHandle(Handle<LdtkLayer>),
}

#[derive(Clone, Debug, Deserialize, Reflect, Serialize)]
pub struct LdtkProjectSettings {
    pub level_separation: f32,
    pub layer_separation: f32,
}

impl Default for LdtkProjectSettings {
    fn default() -> Self {
        Self {
            level_separation: 1.0,
            layer_separation: 0.1,
        }
    }
}

#[derive(Debug, Asset, Reflect)]
pub struct LdtkProject {
    pub(crate) iid: Iid,
    pub(crate) settings: LdtkProjectSettings,
    pub(crate) worlds: IidMap<Handle<LdtkWorld>>,
    pub(crate) levels: IidMap<Handle<LdtkLevel>>,
    pub(crate) layers: IidMap<Handle<LdtkLayer>>,
    pub(crate) entities: IidMap<Handle<LdtkEntity>>,
    pub(crate) tileset_defs: HashMap<i64, TilesetDefinition>,
    pub(crate) tilesets: HashMap<String, Handle<Image>>,
    // LDtk exports
    pub(crate) bg_color: Color,
    // TODO: defs
    // NOTE: external_levels ignored
    pub(crate) json_version: String,
    // TODO: TOC
    // NOTE: world_grid_height, world_grid_width, and world_layout
    //      are exported to the LdtkWorld struct, even for
    //      non-multiworld projects.
}

impl LdtkProject {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn asset_event_system(
        mut commands: Commands,
        mut asset_events: EventReader<AssetEvent<LdtkProject>>,
        mut ldtk_world_events: EventWriter<LdkAssetEvent<LdtkWorld>>,
        mut ldtk_level_events: EventWriter<LdkAssetEvent<LdtkLevel>>,
        mut ldtk_layer_events: EventWriter<LdkAssetEvent<LdtkLayer>>,
        mut ldtk_entity_events: EventWriter<LdkAssetEvent<LdtkEntity>>,
        projects: Res<Assets<LdtkProject>>,
        project_query: Query<(Entity, &Handle<LdtkProject>)>,
        world_assets: Res<Assets<LdtkWorld>>,
        level_assets: Res<Assets<LdtkLevel>>,
        layer_assets: Res<Assets<LdtkLayer>>,
        entity_assets: Res<Assets<LdtkEntity>>,
        world_query: Query<(Entity, &Handle<LdtkWorld>)>,
        level_query: Query<(Entity, &Handle<LdtkLevel>)>,
        layer_query: Query<(Entity, &Handle<LdtkLayer>)>,
        entity_query: Query<(Entity, &Handle<LdtkEntity>)>,
    ) -> Result<(), LdtkProjectError> {
        for asset_event in asset_events.read() {
            match asset_event {
                AssetEvent::Added { id } => {
                    trace!("LdtkProject Added: {id:?}");
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
                    handle_loaded_with_dependencies(
                        *id,
                        &mut commands,
                        &projects,
                        &project_query,
                        &mut ldtk_world_events,
                        &mut ldtk_level_events,
                        &mut ldtk_layer_events,
                        &mut ldtk_entity_events,
                        &world_assets,
                        &level_assets,
                        &layer_assets,
                        &entity_assets,
                        &world_query,
                        &level_query,
                        &layer_query,
                        &entity_query,
                    )?;
                }
            }
        }

        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_loaded_with_dependencies(
    id: AssetId<LdtkProject>,
    commands: &mut Commands,
    projects: &Assets<LdtkProject>,
    project_query: &Query<(Entity, &Handle<LdtkProject>)>,
    world_events: &mut EventWriter<LdkAssetEvent<LdtkWorld>>,
    level_events: &mut EventWriter<LdkAssetEvent<LdtkLevel>>,
    layer_events: &mut EventWriter<LdkAssetEvent<LdtkLayer>>,
    entity_events: &mut EventWriter<LdkAssetEvent<LdtkEntity>>,
    world_assets: &Assets<LdtkWorld>,
    level_assets: &Assets<LdtkLevel>,
    layer_assets: &Assets<LdtkLayer>,
    entity_assets: &Assets<LdtkEntity>,
    world_query: &Query<(Entity, &Handle<LdtkWorld>)>,
    level_query: &Query<(Entity, &Handle<LdtkLevel>)>,
    layer_query: &Query<(Entity, &Handle<LdtkLayer>)>,
    entity_query: &Query<(Entity, &Handle<LdtkEntity>)>,
) -> Result<(), LdtkProjectError> {
    let Some((project_entity, project_handle)) =
        project_query.iter().find(|(_, handle)| handle.id() == id)
    else {
        return Err(LdtkProjectError::BadProjectHandle);
    };

    let project_asset = projects
        .get(project_handle)
        .ok_or(LdtkProjectError::BadProjectHandle)?;

    commands.entity(project_entity).insert(Name::new(
        project_handle
            .path()
            .map(|path| path.to_string())
            .unwrap_or("LdtkProject".to_string()),
    ));

    let world_iids: IidSet = project_asset.worlds.keys().copied().collect();

    let worlds = LdtkWorld::collect_entities(commands, project_asset, &world_iids, world_query)?;

    worlds.iter().try_for_each(
        |(world_entity, world_handle)| -> Result<(), LdtkProjectError> {
            commands.entity(project_entity).add_child(*world_entity);
            world_events.send(LdkAssetEvent::<LdtkWorld>::Modified {
                entity: *world_entity,
                handle: world_handle.clone(),
            });

            let world_asset = world_assets
                .get(world_handle.id())
                .ok_or(LdtkProjectError::BadWorldHandle(world_handle.clone()))?;

            let level_iids = world_asset.children();

            let levels =
                LdtkLevel::collect_entities(commands, project_asset, level_iids, level_query)?;

            levels.iter().try_for_each(
                |(level_entity, level_handle)| -> Result<(), LdtkProjectError> {
                    commands.entity(*world_entity).add_child(*level_entity);
                    level_events.send(LdkAssetEvent::<LdtkLevel>::Modified {
                        entity: *level_entity,
                        handle: level_handle.clone(),
                    });

                    let level_asset = level_assets
                        .get(level_handle.id())
                        .ok_or(LdtkProjectError::BadLevelHandle(level_handle.clone()))?;

                    let layer_iids = level_asset.children();

                    let layers = LdtkLayer::collect_entities(
                        commands,
                        project_asset,
                        layer_iids,
                        layer_query,
                    )?;

                    layers.iter().try_for_each(
                        |(layer_entity, layer_handle)| -> Result<(), LdtkProjectError> {
                            commands.entity(*level_entity).add_child(*layer_entity);
                            layer_events.send(LdkAssetEvent::<LdtkLayer>::Modified {
                                entity: *layer_entity,
                                handle: layer_handle.clone(),
                            });

                            let layer_asset = layer_assets
                                .get(layer_handle.id())
                                .ok_or(LdtkProjectError::BadLayerHandle(layer_handle.clone()))?;

                            let entity_iids = layer_asset.children();

                            let entities = LdtkEntity::collect_entities(
                                commands,
                                project_asset,
                                entity_iids,
                                entity_query,
                            )?;

                            entities.iter().try_for_each(
                                |(entity_entity, entity_handle)| -> Result<(), LdtkProjectError> {
                                    commands.entity(*layer_entity).add_child(*entity_entity);
                                    entity_events.send(LdkAssetEvent::<LdtkEntity>::Modified {
                                        entity: *entity_entity,
                                        handle: entity_handle.clone(),
                                    });

                                    Ok(())
                                },
                            )
                        },
                    )
                },
            )
        },
    )?;

    LdtkWorld::despawn_orphaned_entities(
        commands,
        world_assets,
        project_asset.worlds.keys().copied().collect(),
        world_query.iter(),
    )?;

    LdtkLevel::despawn_orphaned_entities(
        commands,
        level_assets,
        project_asset.levels.keys().copied().collect(),
        level_query.iter(),
    )?;

    LdtkLayer::despawn_orphaned_entities(
        commands,
        layer_assets,
        project_asset.layers.keys().copied().collect(),
        layer_query.iter(),
    )?;

    LdtkEntity::despawn_orphaned_entities(
        commands,
        entity_assets,
        project_asset.entities.keys().copied().collect(),
        entity_query.iter(),
    )?;

    Ok(())
}
