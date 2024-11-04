use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Res};
use bevy_hierarchy::{BuildChildren, ChildBuild};
use bevy_ldtk_asset::prelude::HasChildren;
use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_log::debug;
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::component::{DoFinalizeEvent, LdtkComponent, LdtkComponentExt};
use crate::level::Level;
use crate::project_config::ProjectConfig;
use crate::{bad_handle, Result};

pub type World = LdtkComponent<WorldAsset>;

pub(crate) fn world_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<DoFinalizeEvent<WorldAsset>>,
    world_assets: Res<Assets<WorldAsset>>,
    config_assets: Res<Assets<ProjectConfig>>,
    query: Query<(Entity, &World)>,
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
                finalize(&mut commands, data, &world_assets, &config_assets)
            })
    })
}

fn finalize(
    commands: &mut Commands,
    (entity, world): (Entity, &World),
    world_assets: &Assets<WorldAsset>,
    config_assets: &Assets<ProjectConfig>,
) -> Result<()> {
    let world_asset = world_assets
        .get(world.get_handle().id())
        .ok_or(bad_handle!(world.get_handle()))?;

    let project_config = config_assets
        .get(world.get_config_handle().id())
        .ok_or(bad_handle!(world.get_config_handle()))?;

    let name = Name::from(world_asset.identifier.clone());
    let transform = Transform::default();
    let visibility = Visibility::default();

    commands
        .entity(entity)
        .insert((name, transform, visibility))
        .with_children(|parent| {
            world_asset.children().for_each(|level_handle| {
                if project_config
                    .load_pattern
                    .handle_matches_pattern(level_handle)
                {
                    parent.spawn(Level {
                        handle: level_handle.clone(),
                        config: world.get_config_handle(),
                    });
                }
            })
        });

    debug!("World {:?} finalized!", world_asset.identifier);

    Ok(())
}
