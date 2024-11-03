use bevy::asset::AssetServer;
use bevy::asset::Assets;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::hierarchy::{BuildChildren, ChildBuild};
use bevy::log::trace;
use bevy::prelude::{Added, Changed, Or};
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::layer::LayerType;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_ldtk_asset::world::World as WorldAsset;

use crate::entity::Entity as EntityComponent;
use crate::layer::Layer as LayerComponent;
use crate::level::Level as LevelComponent;
use crate::project::Project as ProjectComponent;
use crate::project_config::ProjectConfig;
use crate::world::World as WorldComponent;
use crate::{bad_handle, Error, Result};

macro_rules! handle_load_children_builder {
    ($func_name:ident, $parent_component:ident, $parent_asset:path, $child_component:ident, $child_field:ident) => {
        #[allow(clippy::type_complexity)]
        pub(crate) fn $func_name(
            mut commands: Commands,
            config_assets: Res<Assets<ProjectConfig>>,
            parent_assets: Res<Assets<$parent_asset>>,
            added_query: Query<
                (Entity, &$parent_component),
                Or<(Added<$parent_component>, Changed<$parent_component>)>,
            >,
            children_query: Query<(Entity, &$child_component)>,
        ) -> Result<()> {
            added_query
                .iter()
                .try_for_each(|(entity, parent)| -> Result<()> {
                    trace!(
                        "Processing {} children for {} {entity:?}",
                        stringify!($child_component),
                        stringify!($parent_component)
                    );

                    let project_config = config_assets
                        .get(parent.config.id())
                        .ok_or(bad_handle!(parent.config))?;

                    let world_asset = parent_assets
                        .get(parent.handle.id())
                        .ok_or(bad_handle!(parent.handle))?;

                    world_asset.$child_field.values().for_each(|child_handle| {
                        if !children_query
                            .iter()
                            .filter(|(_, child)| {
                                project_config
                                    .load_pattern
                                    .handle_matches_pattern(&child.handle)
                            })
                            .any(|(_, child)| child.handle == *child_handle)
                        {
                            commands.entity(entity).with_children(|parent_commands| {
                                let id = parent_commands
                                    .spawn($child_component {
                                        handle: child_handle.clone(),
                                        config: parent.config.clone(),
                                    })
                                    .id();

                                trace!(
                                    "Added {} entity {id:?} as child of {} entity {entity:?}",
                                    stringify!($child_component),
                                    stringify!($parent_component)
                                );
                            });
                        }
                    });

                    Ok(())
                })?;

            Ok(())
        }
    };
}

handle_load_children_builder!(
    handle_project_load_children,
    ProjectComponent,
    ProjectAsset,
    WorldComponent,
    worlds
);

handle_load_children_builder!(
    handle_world_load_children,
    WorldComponent,
    WorldAsset,
    LevelComponent,
    levels
);

handle_load_children_builder!(
    handle_level_load_children,
    LevelComponent,
    LevelAsset,
    LayerComponent,
    layers
);

#[allow(clippy::type_complexity)]
pub(crate) fn handle_layer_load_children(
    mut commands: Commands,
    config_assets: Res<Assets<ProjectConfig>>,
    parent_assets: Res<Assets<LayerAsset>>,
    added_query: Query<
        (Entity, &LayerComponent),
        Or<(Added<LayerComponent>, Changed<LayerComponent>)>,
    >,
    children_query: Query<(Entity, &EntityComponent)>,
) -> Result<()> {
    added_query
        .iter()
        .try_for_each(|(entity, parent)| -> Result<()> {
            trace!(
                "Processing {} children for {} {entity:?}",
                stringify!(EntityComponent),
                stringify!(LayerComponent)
            );
            let project_config = config_assets
                .get(parent.config.id())
                .ok_or(bad_handle!(parent.config))?;
            let layer_asset = parent_assets
                .get(parent.handle.id())
                .ok_or(bad_handle!(parent.handle))?;
            if let LayerType::Entities(entities) = &layer_asset.layer_type {
                entities.entity_handles.values().for_each(|child_handle| {
                    if !children_query
                        .iter()
                        .filter(|(_, child)| {
                            project_config
                                .load_pattern
                                .handle_matches_pattern(&child.handle)
                        })
                        .any(|(_, child)| child.handle == *child_handle)
                    {
                        commands.entity(entity).with_children(|parent_commands| {
                            let id = parent_commands
                                .spawn(EntityComponent {
                                    handle: child_handle.clone(),
                                    config: parent.config.clone(),
                                })
                                .id();
                            trace!(
                                "Added {} entity {id:?} as child of {} entity {entity:?}",
                                stringify!(EntityComponent),
                                stringify!(LayerComponent)
                            );
                        });
                    }
                });
            }
            Ok(())
        })?;
    Ok(())
}
