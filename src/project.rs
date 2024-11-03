use bevy::asset::{AssetEvent, AssetServer, Assets};
use bevy::core::Name;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Res;
use bevy::ecs::system::{Commands, Query};
use bevy::log::debug;
use bevy::prelude::{BuildChildren, Changed, ChildBuild, EventReader};
use bevy::transform::components::Transform;
use bevy::utils::HashSet;
use bevy_ldtk_asset::prelude::HasChildren;
use bevy_ldtk_asset::project::Project as ProjectAsset;

use crate::component::{LdtkComponent, LdtkComponentExt};
use crate::project_config::ProjectConfig;
use crate::world::World;
use crate::{bad_entity, bad_handle, Result};

pub type Project = LdtkComponent<ProjectAsset>;
pub type ProjectData<'a> = (&'a Project, Entity);

#[allow(clippy::too_many_arguments)]
pub(crate) fn handle_project_component_added(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut project_events: EventReader<AssetEvent<ProjectAsset>>,
    mut config_events: EventReader<AssetEvent<ProjectConfig>>,
    project_assets: Res<Assets<ProjectAsset>>,
    config_assets: Res<Assets<ProjectConfig>>,
    query: Query<ProjectData>,
    query_changed: Query<ProjectData, Changed<Project>>,
) -> Result<()> {
    let with_added_component_handle: HashSet<Entity> = project_events
        .read()
        .filter_map(|event| match event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => Some(*id),
            _ => None,
        })
        .inspect(|_| debug!("Added/Modified event for Project!"))
        .flat_map(|id| {
            query
                .iter()
                .filter(move |(component, ..)| component.get_handle().id() == id)
                .filter(|(component, ..)| component.is_loaded(&asset_server))
                .map(|(_, entity, ..)| entity)
        })
        .collect();

    let with_added_config_handle: HashSet<Entity> = config_events
        .read()
        .filter_map(|event| match event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => Some(*id),
            _ => None,
        })
        .inspect(|_| debug!("Added/Modified event for Project Config!"))
        .flat_map(|id| {
            query
                .iter()
                .filter(move |(component, ..)| component.get_config_handle().id() == id)
                .filter(|(component, ..)| component.is_loaded(&asset_server))
                .map(|(_, entity, ..)| entity)
        })
        .collect();

    let with_changed_component: HashSet<Entity> = query_changed
        .iter()
        .inspect(|_| debug!("Component changed for Project!"))
        .filter(|(component, ..)| component.is_loaded(&asset_server))
        .map(|(_, entity, ..)| entity)
        .collect();

    let entities_to_finalize: HashSet<Entity> = with_added_component_handle
        .into_iter()
        .chain(with_added_config_handle)
        .chain(with_changed_component)
        .collect();

    entities_to_finalize
        .into_iter()
        .try_for_each(|entity| -> Result<()> {
            let data = query.get(entity).map_err(|_| bad_entity!(entity))?;

            finish(&mut commands, data, &project_assets, &config_assets)
        })
}

fn finish(
    commands: &mut Commands,
    data: ProjectData,
    project_assets: &Assets<ProjectAsset>,
    config_assets: &Assets<ProjectConfig>,
) -> Result<()> {
    let (project, entity, ..) = data;

    let project_asset = project_assets
        .get(project.get_handle().id())
        .ok_or(bad_handle!(project.get_handle()))?;

    let project_config = config_assets
        .get(project.get_config_handle().id())
        .ok_or(bad_handle!(project.get_config_handle()))?;

    commands
        .entity(entity)
        .insert(Name::new(format!("{:?}", project.get_handle().path())));

    commands.entity(entity).insert(Transform::default());

    commands.entity(entity).with_children(|parent| {
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
        "Finished spawning Project: {:?}",
        project.get_handle().path()
    );

    Ok(())
}
