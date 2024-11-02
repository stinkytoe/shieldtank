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

use crate::automations::LevelBackgroundAutomation;
use crate::level_background::LevelBackground;
use crate::project_config::ProjectConfig;
use crate::{Error, Result};

#[derive(Component, Debug, Reflect)]
pub struct Level {
    pub handle: Handle<ldtk_asset::Level>,
    pub config: Handle<ProjectConfig>,
}

// ## Level
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
//  -- Depends on WorldLayout
//  --- Free or GridVania: from asset: location, world_depth TODO: We need to establish a scale factor for calculating z
//  --- LinearHorizontal or LinearVertical: TODO: What to do here?
//  -- Only on new, and if not present
//
//  - LevelBackground
//  -- from asset
//  -- always update
//  -- systems use this to draw background
#[allow(clippy::type_complexity)]
pub(crate) fn handle_level_component_added(
    mut commands: Commands,
    assets: Res<Assets<ldtk_asset::Level>>,
    configs: Res<Assets<ProjectConfig>>,
    query: Query<
        (
            Entity,
            &Level,
            Option<&Name>,
            Option<&Transform>,
            Option<&LevelBackground>,
        ),
        Added<Level>,
    >,
) -> Result<()> {
    query.iter().try_for_each(
        |(entity, level, name, transform, background)| -> Result<()> {
            let asset = assets.get(level.handle.id()).ok_or(Error::BadHandle)?;
            let config = configs.get(level.config.id()).ok_or(Error::BadHandle)?;

            if name.is_none() {
                let name = asset.identifier.clone();
                commands.entity(entity).insert(Name::new(name));
            }

            if transform.is_none() {
                let location = asset
                    .location
                    .extend((asset.world_depth as f32) * config.level_z_scale);
                commands
                    .entity(entity)
                    .insert(Transform::from_translation(location));
            }

            commands.entity(entity).insert(Visibility::default());

            if background.is_none() {
                let color = asset.bg_color;
                let size = asset.size;
                let background = asset.background.clone();
                let background = LevelBackground {
                    color,
                    size,
                    background,
                };

                commands
                    .entity(entity)
                    .insert((background, LevelBackgroundAutomation));
            }

            debug!("Level entity added and set up! {entity:?}");
            Ok(())
        },
    )?;

    Ok(())
}

pub(crate) fn handle_level_asset_modified(
    mut commands: Commands,
    mut asset_events: EventReader<AssetEvent<ldtk_asset::Level>>,
    assets: Res<Assets<ldtk_asset::Level>>,
    query: Query<(Entity, &Level, Option<&LevelBackgroundAutomation>)>,
) -> Result<()> {
    asset_events.read().try_for_each(|event| -> Result<()> {
        if let AssetEvent::Modified { id } = event {
            query
                .iter()
                .filter(|(_, level, ..)| level.handle.id() == *id)
                .try_for_each(|(entity, level, background_automation)| -> Result<()> {
                    if background_automation.is_some() {
                        let asset = assets.get(level.handle.id()).ok_or(Error::BadHandle)?;
                        let color = asset.bg_color;
                        let size = asset.size;
                        let background = asset.background.clone();
                        let background = LevelBackground {
                            color,
                            size,
                            background,
                        };
                        commands.entity(entity).insert(background);
                    }

                    Ok(())
                })?;
        }

        Ok(())
    })?;

    Ok(())
}
