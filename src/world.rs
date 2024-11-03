use bevy::asset::Handle;
use bevy::core::Name;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::{Commands, Query};
use bevy::log::debug;
use bevy::prelude::Added;
use bevy::reflect::Reflect;
use bevy::render::view::Visibility;
use bevy::transform::components::Transform;
use bevy_ldtk_asset::prelude::ldtk_asset;

use crate::project_config::ProjectConfig;
use crate::Result;

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
#[allow(clippy::type_complexity)]
pub(crate) fn handle_world_component_added(
    mut commands: Commands,
    query_added: Query<(Entity, &World, Option<&Name>, Option<&Transform>), Added<World>>,
) -> Result<()> {
    query_added
        .iter()
        .try_for_each(|(entity, world, name, transform)| -> Result<()> {
            if name.is_none() {
                let name = world
                    .handle
                    .path()
                    .map(|path| path.to_string())
                    .unwrap_or("<project>".to_string());
                commands.entity(entity).insert(Name::new(name));
            }

            if transform.is_none() {
                commands.entity(entity).insert(Transform::default());
            }

            commands.entity(entity).insert(Visibility::default());

            debug!("World entity added and set up! {entity:?}");
            Ok(())
        })?;

    Ok(())
}
