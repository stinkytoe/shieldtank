use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use thiserror::Error;

use crate::assets::entity::LdtkEntityAsset;
use crate::assets::event::LdkAssetEvent;
use crate::assets::layer::LdtkLayerAsset;
use crate::assets::level::LdtkLevelAsset;
use crate::assets::traits::LdtkAsset;
use crate::assets::traits::LdtkAssetError;
use crate::assets::world::LdtkWorldAsset;
use crate::assets::world::LdtkWorldAssetError;
use crate::iid::IidMap;
use crate::iid::IidSet;
use crate::prelude::Iid;
use crate::reexports::layer_definition::LayerDefinition;
use crate::reexports::tileset_definition::TilesetDefinition;

#[derive(Debug, Error)]
pub enum LdtkProjectError {
    #[error(transparent)]
    LdtkWorldError(#[from] LdtkWorldAssetError),
    #[error(transparent)]
    LdtkAssetWorldError(#[from] LdtkAssetError<LdtkWorldAsset>),
    #[error(transparent)]
    LdtkAssetLevelError(#[from] LdtkAssetError<LdtkLevelAsset>),
    #[error(transparent)]
    LdtkAssetLayerError(#[from] LdtkAssetError<LdtkLayerAsset>),
    #[error(transparent)]
    LdtkAssetEntityError(#[from] LdtkAssetError<LdtkEntityAsset>),
    #[error("Bad project handle?")]
    BadProjectHandle,
    #[error("Bad world handle?")]
    BadWorldHandle(Handle<LdtkWorldAsset>),
    #[error("Bad level handle?")]
    BadLevelHandle(Handle<LdtkLevelAsset>),
    #[error("Bad layer handle?")]
    BadLayerHandle(Handle<LdtkLayerAsset>),
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
    pub(crate) worlds: IidMap<Handle<LdtkWorldAsset>>,
    pub(crate) levels: IidMap<Handle<LdtkLevelAsset>>,
    pub(crate) layer_defs: HashMap<i64, LayerDefinition>,
    pub(crate) layers: IidMap<Handle<LdtkLayerAsset>>,
    pub(crate) entities: IidMap<Handle<LdtkEntityAsset>>,
    pub(crate) tileset_defs: HashMap<i64, TilesetDefinition>,
    pub(crate) tilesets: HashMap<String, Handle<Image>>,
    // LDtk exports
    pub(crate) bg_color: Color,
    // TODO: defs
    // NOTE: external_levels field not exported, but honored in LdtkProjectLoader
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
        mut ldtk_world_events: EventWriter<LdkAssetEvent<LdtkWorldAsset>>,
        mut ldtk_level_events: EventWriter<LdkAssetEvent<LdtkLevelAsset>>,
        mut ldtk_layer_events: EventWriter<LdkAssetEvent<LdtkLayerAsset>>,
        mut ldtk_entity_events: EventWriter<LdkAssetEvent<LdtkEntityAsset>>,
        projects: Res<Assets<LdtkProject>>,
        project_query: Query<(Entity, &Handle<LdtkProject>)>,
        world_assets: Res<Assets<LdtkWorldAsset>>,
        level_assets: Res<Assets<LdtkLevelAsset>>,
        layer_assets: Res<Assets<LdtkLayerAsset>>,
        entity_assets: Res<Assets<LdtkEntityAsset>>,
        world_query: Query<(Entity, &Handle<LdtkWorldAsset>)>,
        level_query: Query<(Entity, &Handle<LdtkLevelAsset>)>,
        layer_query: Query<(Entity, &Handle<LdtkLayerAsset>)>,
        entity_query: Query<(Entity, &Handle<LdtkEntityAsset>)>,
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
    world_events: &mut EventWriter<LdkAssetEvent<LdtkWorldAsset>>,
    level_events: &mut EventWriter<LdkAssetEvent<LdtkLevelAsset>>,
    layer_events: &mut EventWriter<LdkAssetEvent<LdtkLayerAsset>>,
    entity_events: &mut EventWriter<LdkAssetEvent<LdtkEntityAsset>>,
    world_assets: &Assets<LdtkWorldAsset>,
    level_assets: &Assets<LdtkLevelAsset>,
    layer_assets: &Assets<LdtkLayerAsset>,
    entity_assets: &Assets<LdtkEntityAsset>,
    world_query: &Query<(Entity, &Handle<LdtkWorldAsset>)>,
    level_query: &Query<(Entity, &Handle<LdtkLevelAsset>)>,
    layer_query: &Query<(Entity, &Handle<LdtkLayerAsset>)>,
    entity_query: &Query<(Entity, &Handle<LdtkEntityAsset>)>,
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

    let worlds =
        LdtkWorldAsset::collect_entities(commands, project_asset, &world_iids, world_query)?;

    worlds.iter().try_for_each(
        |(world_entity, world_handle)| -> Result<(), LdtkProjectError> {
            commands.entity(project_entity).add_child(*world_entity);
            world_events.send(LdkAssetEvent::<LdtkWorldAsset>::Modified {
                entity: *world_entity,
                handle: world_handle.clone(),
            });

            let world_asset = world_assets
                .get(world_handle.id())
                .ok_or(LdtkProjectError::BadWorldHandle(world_handle.clone()))?;

            let level_iids = world_asset.children();

            let levels =
                LdtkLevelAsset::collect_entities(commands, project_asset, level_iids, level_query)?;

            levels.iter().try_for_each(
                |(level_entity, level_handle)| -> Result<(), LdtkProjectError> {
                    commands.entity(*world_entity).add_child(*level_entity);
                    level_events.send(LdkAssetEvent::<LdtkLevelAsset>::Modified {
                        entity: *level_entity,
                        handle: level_handle.clone(),
                    });

                    let level_asset = level_assets
                        .get(level_handle.id())
                        .ok_or(LdtkProjectError::BadLevelHandle(level_handle.clone()))?;

                    let layer_iids = level_asset.children();

                    let layers = LdtkLayerAsset::collect_entities(
                        commands,
                        project_asset,
                        layer_iids,
                        layer_query,
                    )?;

                    layers.iter().try_for_each(
                        |(layer_entity, layer_handle)| -> Result<(), LdtkProjectError> {
                            commands.entity(*level_entity).add_child(*layer_entity);
                            layer_events.send(LdkAssetEvent::<LdtkLayerAsset>::Modified {
                                entity: *layer_entity,
                                handle: layer_handle.clone(),
                            });

                            let layer_asset = layer_assets
                                .get(layer_handle.id())
                                .ok_or(LdtkProjectError::BadLayerHandle(layer_handle.clone()))?;

                            let entity_iids = layer_asset.children();

                            let entities = LdtkEntityAsset::collect_entities(
                                commands,
                                project_asset,
                                entity_iids,
                                entity_query,
                            )?;

                            entities.iter().try_for_each(
                                |(entity_entity, entity_handle)| -> Result<(), LdtkProjectError> {
                                    commands.entity(*layer_entity).add_child(*entity_entity);
                                    entity_events.send(
                                        LdkAssetEvent::<LdtkEntityAsset>::Modified {
                                            entity: *entity_entity,
                                            handle: entity_handle.clone(),
                                        },
                                    );

                                    Ok(())
                                },
                            )
                        },
                    )
                },
            )
        },
    )?;

    LdtkWorldAsset::despawn_orphaned_entities(
        commands,
        world_assets,
        project_asset.worlds.keys().copied().collect(),
        world_query.iter(),
    )?;

    LdtkLevelAsset::despawn_orphaned_entities(
        commands,
        level_assets,
        project_asset.levels.keys().copied().collect(),
        level_query.iter(),
    )?;

    LdtkLayerAsset::despawn_orphaned_entities(
        commands,
        layer_assets,
        project_asset.layers.keys().copied().collect(),
        layer_query.iter(),
    )?;

    LdtkEntityAsset::despawn_orphaned_entities(
        commands,
        entity_assets,
        project_asset.entities.keys().copied().collect(),
        entity_query.iter(),
    )?;

    Ok(())
}
