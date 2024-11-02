use bevy::asset::{AssetEvent, Assets, Handle};
use bevy::core::Name;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::{Commands, Query, Res};
use bevy::log::debug;
use bevy::prelude::Added;
use bevy::reflect::Reflect;
use bevy::render::view::Visibility;
use bevy::transform::components::Transform;
use bevy_ldtk_asset::prelude::ldtk_asset;

use crate::project_config::ProjectConfig;
use crate::{Error, Result};

#[derive(Component, Debug, Default, Reflect)]
pub struct Layer {
    pub handle: Handle<ldtk_asset::Layer>,
    pub config: Handle<ProjectConfig>,
}

// ## Layer
//  - Name
//  -- from identifier
//  -- Only on new, and if not present
//  -- if changed, then asset path changed also and is now a different asset
//
//  - Visibility
//  -- always visible
//  -- Only on new, and if not present
//
//  - Transform
//  -- always translation (0,0,0)
//  -- Only on new, and if not present
//
//  - Tiles
//  -- only for layers with tiles
//  -- delete if no tiles/changed to entity layer
//  -- from asset
//  -- always update
//  -- systems use this to draw layer
#[allow(clippy::type_complexity)]
pub(crate) fn handle_layer_component_added(
    mut commands: Commands,
    assets: Res<Assets<ldtk_asset::Layer>>,
    configs: Res<Assets<ProjectConfig>>,
    query: Query<(Entity, &Layer, Option<&Name>, Option<&Transform>), Added<Layer>>,
) -> Result<()> {
    query
        .iter()
        .try_for_each(|(entity, layer, name, transform)| -> Result<()> {
            let asset = assets.get(layer.handle.id()).ok_or(Error::BadHandle)?;
            let _config = configs.get(layer.config.id()).ok_or(Error::BadHandle)?;

            if name.is_none() {
                let name = asset.identifier.clone();
                commands.entity(entity).insert(Name::new(name));
            }

            if transform.is_none() {
                commands.entity(entity).insert(Transform::default());
            }

            commands.entity(entity).insert(Visibility::default());

            debug!("Layer entity added and set up! {entity:?}");
            Ok(())
        })?;

    Ok(())
}

pub(crate) fn _handle_layer_asset_modified(
    mut _commands: Commands,
    mut asset_events: EventReader<AssetEvent<ldtk_asset::Layer>>,
    assets: Res<Assets<ldtk_asset::Layer>>,
    query: Query<(Entity, &Layer)>,
) -> Result<()> {
    asset_events.read().try_for_each(|event| -> Result<()> {
        if let AssetEvent::Modified { id } = event {
            query
                .iter()
                .filter(|(_, layer, ..)| layer.handle.id() == *id)
                .try_for_each(|(_event, layer)| -> Result<()> {
                    let _asset = assets.get(layer.handle.id()).ok_or(Error::BadHandle)?;
                    Ok(())
                })?;
        };

        Ok(())
    })?;

    Ok(())
}
