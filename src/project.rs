use bevy::asset::{AssetServer, Handle};
use bevy::core::Name;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::log::debug;
use bevy::prelude::Added;
use bevy::reflect::Reflect;
use bevy::tasks::block_on;
use bevy::transform::components::Transform;
use bevy_ldtk_asset::prelude::ldtk_asset;

use crate::project_config::ProjectConfig;
use crate::Result;

#[derive(Component, Debug, Reflect)]
pub struct Project {
    pub handle: Handle<ldtk_asset::Project>,
    pub config: Handle<ProjectConfig>,
}

// ## Project
// ### Components
//  - Name
//  -- from path
//  -- only need on new, because path would change otherwise
//
//  - Transform
//  -- always translation (0,0,0)
//  -- Only on new, and if not present
#[allow(clippy::type_complexity)]
pub(crate) fn handle_project_component_added(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query_added: Query<(Entity, &Project, Option<&Name>, Option<&Transform>), Added<Project>>,
) -> Result<()> {
    query_added
        .iter()
        .try_for_each(|(entity, project, name, transform)| -> Result<()> {
            block_on(async { asset_server.wait_for_asset(&project.handle).await })?;

            if name.is_none() {
                let name = project
                    .handle
                    .path()
                    .map(|path| path.to_string())
                    .unwrap_or("<project>".to_string());
                commands.entity(entity).insert(Name::new(name));
            }

            if transform.is_none() {
                commands.entity(entity).insert(Transform::default());
            }

            debug!("Project entity added and set up! {entity:?}");

            Ok(())
        })?;

    Ok(())
}
