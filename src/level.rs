use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Res};
// NOTE: Is this a good idea?
use bevy_ecs::world::Ref;
use bevy_hierarchy::{BuildChildren, ChildBuild};
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_log::debug;
use bevy_math::{Rect, Vec2};
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::component::{DoFinalizeEvent, LdtkComponent, LdtkComponentExt};
use crate::item::LdtkItem;
use crate::layer::Layer;
use crate::level_background::LevelBackground;
use crate::project_config::ProjectConfig;
use crate::query::LdtkQuery;
use crate::{bad_handle, Result};

pub type Level = LdtkComponent<LevelAsset>;
pub type LevelItem<'a> = LdtkItem<'a, LevelAsset, LevelData<'a>>;
pub type LevelData<'a> = (
    EcsEntity,
    Ref<'a, Level>,
    Ref<'a, Visibility>,
    Ref<'a, Transform>,
    Option<Ref<'a, LevelBackground>>,
);

impl LevelItem<'_> {
    pub fn transform(&self) -> &Transform {
        &self.data.3
    }
}

impl LevelItem<'_> {
    pub(crate) fn make_level_iterator<'a>(
        query: &'a LdtkQuery,
    ) -> impl Iterator<Item = LevelItem<'a>> {
        query
            .levels_query
            .iter()
            .filter_map(|data| {
                query
                    .level_assets
                    .get(data.1.handle.id())
                    .map(|asset| (asset, data))
            })
            .map(|(asset, data)| LevelItem {
                asset,
                data,
                _query: query,
            })
    }

    pub(crate) fn get_level<'a>(
        query: &'a LdtkQuery,
        ecs_entity: EcsEntity,
    ) -> Option<LevelItem<'a>> {
        query
            .levels_query
            .get(ecs_entity)
            .ok()
            .and_then(|data| {
                query
                    .level_assets
                    .get(data.1.handle.id())
                    .map(|asset| (asset, data))
            })
            .map(|(asset, data)| LevelItem {
                asset,
                data,
                _query: query,
            })
    }
}

pub trait LevelItemIteratorExt<'a>: Iterator<Item = LevelItem<'a>> + Sized {
    fn added(self) -> impl Iterator<Item = LevelItem<'a>> {
        self.filter(|item| item.data.1.is_added())
    }

    fn changed(self) -> impl Iterator<Item = LevelItem<'a>> {
        self.filter(|item| item.data.1.is_changed())
    }

    fn contains_point(self, point: Vec2) -> impl Iterator<Item = LevelItem<'a>> {
        self.filter(move |item| {
            let p0 = item.transform().translation.truncate();
            let p1 = p0 + item.asset.size * Vec2::new(1.0, -1.0);
            let bounding_rectangle = Rect::from_corners(p0, p1);

            bounding_rectangle.contains(point)
        })
    }
}

impl<'a, I: Iterator<Item = LevelItem<'a>>> LevelItemIteratorExt<'a> for I {}

pub(crate) fn level_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<DoFinalizeEvent<LevelAsset>>,
    level_assets: Res<Assets<LevelAsset>>,
    config_assets: Res<Assets<ProjectConfig>>,
    query: Query<(EcsEntity, &Level)>,
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
    (entity, level): (EcsEntity, &Level),
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
