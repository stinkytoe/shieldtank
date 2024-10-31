use bevy::asset::{AssetEvent, AssetServer, Assets, Handle};
use bevy::core::Name;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::log::debug;
use bevy::reflect::Reflect;
use bevy::tasks::block_on;
use bevy::transform::components::Transform;
use bevy_ldtk_asset::prelude::ldtk_asset;

use crate::project_config::ProjectConfig;
use crate::{Error, Result};

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
pub(crate) fn handle_project_asset_events(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<ldtk_asset::Project>>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<ldtk_asset::Project>>,
    query_added: Query<(Entity, &Project, Option<&Transform>)>,
    //query_modified: Query<(Entity, &Project)>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        match event {
            AssetEvent::Added { id } => {
                let handle = assets.get_strong_handle(*id).ok_or(Error::BadHandle)?;
                block_on(async { asset_server.wait_for_asset(&handle).await })?;
                query_added
                    .iter()
                    .filter(|(_, project, _)| project.handle.id() == *id)
                    .for_each(|(entity, project, transform)| {
                        // Name
                        let path = project
                            .handle
                            .path()
                            .map(|path| path.to_string())
                            .unwrap_or("<project>".to_string());
                        commands.entity(entity).insert(Name::new(path.clone()));

                        // Transform
                        if transform.is_none() {
                            commands.entity(entity).insert(Transform::default());
                        }

                        debug!("Project entity added: {path} entity: {entity:?}");
                    });
            }
            AssetEvent::Modified { id: _ } => {}
            _ => {}
        };

        Ok(())
    })?;

    Ok(())
}
