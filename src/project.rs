use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Res};
use bevy_hierarchy::{BuildChildren, ChildBuild};
use bevy_ldtk_asset::prelude::HasChildren;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_log::debug;
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::component::{DoFinalizeEvent, LdtkComponent, LdtkComponentExt};
use crate::project_config::ProjectConfig;
use crate::world::World;
use crate::{bad_handle, Result};

pub type Project = LdtkComponent<ProjectAsset>;

pub(crate) fn project_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<DoFinalizeEvent<ProjectAsset>>,
    project_assets: Res<Assets<ProjectAsset>>,
    config_assets: Res<Assets<ProjectConfig>>,
    query: Query<(Entity, &Project)>,
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
                finalize(&mut commands, data, &project_assets, &config_assets)
            })
    })
}

fn finalize(
    commands: &mut Commands,
    (entity, project): (Entity, &Project),
    project_assets: &Assets<ProjectAsset>,
    config_assets: &Assets<ProjectConfig>,
) -> Result<()> {
    let project_asset = project_assets
        .get(project.get_handle().id())
        .ok_or(bad_handle!(project.get_handle()))?;

    let project_config = config_assets
        .get(project.get_config_handle().id())
        .ok_or(bad_handle!(project.get_config_handle()))?;

    let name = Name::new(format!("{:?}", project.get_handle().path()));
    let transform = Transform::default();
    let visibility = Visibility::default();

    commands
        .entity(entity)
        .insert((name, transform, visibility))
        .with_children(|parent| {
            project_asset.children().for_each(|world_handle| {
                if project_config
                    .load_pattern
                    .handle_matches_pattern(world_handle)
                {
                    parent.spawn(World {
                        handle: world_handle.clone(),
                        config: project.get_config_handle(),
                    });
                };
            });
        });

    debug!(
        "Project {:?} finalized!",
        project
            .get_handle()
            .path()
            .map(|path| format!("{}", path))
            .unwrap_or(String::default())
    );

    Ok(())
}
