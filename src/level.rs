use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::entity::Entity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Res};
use bevy_hierarchy::{BuildChildren, ChildBuild};
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_log::debug;
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::component::{DoFinalizeEvent, LdtkComponent, LdtkComponentExt};
use crate::layer::Layer;
use crate::level_background::LevelBackground;
use crate::project_config::ProjectConfig;
use crate::{bad_handle, Result};

pub type Level = LdtkComponent<LevelAsset>;

pub(crate) fn level_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<DoFinalizeEvent<LevelAsset>>,
    level_assets: Res<Assets<LevelAsset>>,
    config_assets: Res<Assets<ProjectConfig>>,
    query: Query<(Entity, &Level)>,
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
                finalize(&mut commands, data, &level_assets, &config_assets)
            })
    })
}

fn finalize(
    commands: &mut Commands,
    (entity, level): (Entity, &Level),
    level_assets: &Assets<LevelAsset>,
    config_assets: &Assets<ProjectConfig>,
) -> Result<()> {
    let level_asset = level_assets
        .get(level.get_handle().id())
        .ok_or(bad_handle!(level.get_handle()))?;

    let project_config = config_assets
        .get(level.get_config_handle().id())
        .ok_or(bad_handle!(level.get_config_handle()))?;

    let name = Name::from(level_asset.identifier.clone());

    let translation = level_asset
        .location
        .extend((level_asset.world_depth as f32) * project_config.level_z_scale);
    let transform = Transform::from_translation(translation);

    let visibility = Visibility::default();

    let color = level_asset.bg_color;
    let size = level_asset.size;
    let background = level_asset.background.clone();
    let background = LevelBackground {
        color,
        size,
        background,
    };

    commands
        .entity(entity)
        .insert((name, transform, visibility, background))
        .with_children(|parent| {
            level_asset.layers.values().for_each(|layer_handle| {
                if project_config
                    .load_pattern
                    .handle_matches_pattern(layer_handle)
                {
                    parent.spawn(Layer {
                        handle: layer_handle.clone(),
                        config: level.get_config_handle(),
                    });
                }
            })
        });

    debug!("Level {:?} finalized!", level_asset.identifier);

    Ok(())
}
