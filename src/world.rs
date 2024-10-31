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
pub struct World {
    pub handle: Handle<ldtk_asset::World>,
    pub config: Handle<ProjectConfig>,
}

// ## World
//  - Name
//  -- from identifier
//  -- Only on new, and if not present
//  -- if changed, then asset path changed also and is now a different asset
//
//  - Transform
//  -- always translation (0,0,0)
//  -- Only on new, and if not present
pub(crate) fn handle_world_asset_events(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<ldtk_asset::World>>,
    asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<ldtk_asset::World>>,
    query_added: Query<(Entity, &World, Option<&Transform>)>,
    //query_modified: Query<(Entity, &Project)>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        match event {
            AssetEvent::Added { id } => {
                let handle = assets.get_strong_handle(*id).ok_or(Error::BadHandle)?;
                block_on(async { asset_server.wait_for_asset(&handle).await })?;
                query_added
                    .iter()
                    .filter(|(_, world, _)| world.handle.id() == *id)
                    .try_for_each(|(entity, world, transform)| -> Result<()> {
                        let asset = assets.get(world.handle.id()).ok_or(Error::BadHandle)?;
                        // Name
                        let name = &asset.identifier;
                        commands.entity(entity).insert(Name::new(name.clone()));

                        // Transform
                        if transform.is_none() {
                            commands.entity(entity).insert(Transform::default());
                        }

                        debug!("World entity added: {name} entity: {entity:?}");
                        Ok(())
                    })?;
            }
            AssetEvent::Modified { id: _ } => {}
            _ => {}
        };

        Ok(())
    })?;

    Ok(())
}
