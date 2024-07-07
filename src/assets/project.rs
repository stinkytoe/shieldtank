use bevy::prelude::*;
use bevy::reflect::Map;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::assets::entity::LdtkEntity;
use crate::assets::layer::LdtkLayer;
use crate::assets::level::LdtkLevel;
use crate::assets::traits::LdtkAsset;
use crate::assets::world::LdtkWorld;
use crate::assets::world::LdtkWorldError;
use crate::components::entity::LdtkEntityComponent;
use crate::components::layer::LdtkLayerComponent;
use crate::components::level::LdtkLevelComponent;
use crate::components::world::LdtkWorldComponent;
use crate::components::world::LdtkWorldComponentError;
use crate::iid::IidMap;

#[derive(Debug, Error)]
pub enum LdtkProjectError {
    #[error(transparent)]
    LdtkWorldError(#[from] LdtkWorldError),
    #[error(transparent)]
    LdtkWorldComponentError(#[from] LdtkWorldComponentError),
    // #[error("Stub!")]
    // Stub,
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
    pub(crate) fn asset_event_system(
        mut commands: Commands,
        mut asset_events: EventReader<AssetEvent<LdtkProject>>,
        project_query: Query<(Entity, &Handle<LdtkProject>)>,
        projects: Res<Assets<LdtkProject>>,
        worlds: Res<Assets<LdtkWorld>>,
        levels: Res<Assets<LdtkLevel>>,
        layers: Res<Assets<LdtkLayer>>,
        entities: Res<Assets<LdtkEntity>>,
    ) -> Result<(), LdtkProjectError> {
        for asset_event in asset_events.read() {
            match asset_event {
                AssetEvent::Added { id } => {
                    debug!("LdtkProject Added: {id:?}");
                    for (entity, handle) in project_query
                        .iter()
                        .find(|(_, handle)| handle.id() == *id)
                        .iter()
                    {
                        let project = projects.get(*handle).expect("bad handle?");

                        commands.entity(*entity).with_children(|parent| {
                            for world_handle in project.worlds.values() {
                                let world = worlds.get(world_handle).expect("bad handle?");

                                parent
                                    .spawn((
                                        LdtkWorldComponent::new(world)
                                            .expect("failed world component spawn?"),
                                        SpatialBundle::default(),
                                    ))
                                    .with_children(|parent| {
                                        for level_iid in world.children() {
                                            let level_handle = project
                                                .levels
                                                .get(level_iid)
                                                .expect("bad level iid?");

                                            let level =
                                                levels.get(level_handle).expect("bad handle?");

                                            parent
                                                .spawn((
                                                    LdtkLevelComponent::new(level)
                                                        .expect("failed component?"),
                                                    SpatialBundle::default(),
                                                ))
                                                .with_children(|parent| {
                                                    for layer_iid in level.children() {
                                                        let layer_handle = project
                                                            .layers
                                                            .get(layer_iid)
                                                            .expect("bad layer iid?");

                                                        let layer = layers
                                                            .get(layer_handle)
                                                            .expect("bad handle?");

                                                        parent
                                                            .spawn((
                                                                LdtkLayerComponent::new(layer)
                                                                    .expect("failed component?"),
                                                                SpatialBundle::default(),
                                                            ))
                                                            .with_children(|parent| {
                                                                for entity_iid in layer.children() {
                                                                    let entity_handle = project
                                                                        .entities
                                                                        .get(entity_iid)
                                                                        .expect("bad entity iid?");

                                                                    let lentity = entities
                                                                        .get(entity_handle)
                                                                        .expect("bad handle?");

                                                                    parent.spawn((
                                                                        LdtkEntityComponent::new(lentity).expect("failed component?"),
                                                                        SpatialBundle::default(),
                                                                    ));
                                                                }
                                                            });
                                                    }
                                                });
                                        }
                                    });
                            }
                        });
                    }
                }
                AssetEvent::Modified { id } => {
                    debug!("LdtkProject Modified: {id:?}");
                }
                AssetEvent::Removed { id } => {
                    debug!("LdtkProject Removed: {id:?}");
                }
                AssetEvent::Unused { id } => {
                    debug!("LdtkProject Unused: {id:?}");
                }
                AssetEvent::LoadedWithDependencies { id } => {
                    debug!("LdtkProject LoadedWithDependencies: {id:?}");
                }
            }
        }

        Ok(())
    }

    // fn spawn_worlds(
    //     commands: &mut Commands,
    //     project: &LdtkProject,
    //     worlds: &Assets<LdtkWorld>,
    //     levels: &Assets<LdtkLevel>,
    // ) -> Result<Vec<Entity>, LdtkProjectError> {
    //     project
    //         .worlds
    //         .values()
    //         .map(move |world_handle| {
    //             let world = worlds.get(world_handle).ok_or(LdtkProjectError::Stub)?;
    //             let entity_commands =
    //                 commands.spawn((LdtkWorldComponent::new(world)?, SpatialBundle::default()));
    //
    //             // let level_entities = Self::spawn_levels(commands, world, levels);
    //
    //             let entity = entity_commands.id();
    //
    //             Ok(entity)
    //         })
    //         .collect::<Result<Vec<Entity>, LdtkProjectError>>()
    // }
    //
    // fn spawn_levels(
    //     commands: &mut Commands,
    //     world: &LdtkWorld,
    //     levels: &Assets<LdtkLevel>,
    // ) -> Result<Vec<Entity>, LdtkProjectError> {
    //     todo!()
    // }
}
