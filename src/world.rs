use bevy_asset::{AssetServer, Assets, Handle};
use bevy_core::Name;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::Added;
use bevy_ecs::system::{Commands, Query, Res};
use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_reflect::Reflect;
use bevy_transform::components::Transform;

use crate::project_config::ProjectConfig;
use crate::Result;

#[derive(Component, Debug, Reflect)]
pub struct World {
    pub handle: Handle<WorldAsset>,
    pub config: Handle<ProjectConfig>,
}

impl World {
    pub(crate) fn _is_loaded(&self, asset_server: &AssetServer) -> bool {
        asset_server.is_loaded_with_dependencies(self.handle.id())
            && asset_server.is_loaded_with_dependencies(self.config.id())
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn handle_world_component_added(
    mut _commands: Commands,
    _assets: Res<Assets<WorldAsset>>,
    _query_added: Query<(Entity, &World, Option<&Name>, Option<&Transform>), Added<World>>,
) -> Result<()> {
    //query_added
    //    .iter()
    //    .try_for_each(|(entity, world, name, transform)| -> Result<()> {
    //        let asset = assets
    //            .get(world.handle.id())
    //            .ok_or(bad_handle!(world.handle))?;
    //        if name.is_none() {
    //            //let name = world
    //            //    .handle
    //            //    .path()
    //            //    .map(|path| path.to_string())
    //            //    .unwrap_or("<project>".to_string());
    //            let name = asset.identifier.clone();
    //            commands.entity(entity).insert(Name::new(name));
    //        }
    //
    //        if transform.is_none() {
    //            commands.entity(entity).insert(Transform::default());
    //        }
    //
    //        commands.entity(entity).insert(Visibility::default());
    //
    //        debug!("World entity added and set up! {entity:?}");
    //        Ok(())
    //    })?;

    Ok(())
}

pub(crate) fn _finish_loading(
    _commands: &mut Commands,
    _world: &World,
    _world_assets: &Assets<WorldAsset>,
) -> Result<()> {
    todo!()
}
