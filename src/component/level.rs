use std::borrow::Cow;

use bevy_app::Plugin;
use bevy_asset::prelude::AssetChanged;
use bevy_asset::{AsAssetId, Assets, Handle};
use bevy_camera::visibility::Visibility;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::{Commands, Query, Res};
use bevy_ldtk_asset::layer::LayerInstance;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_math::{Rect, Vec2};
use bevy_reflect::Reflect;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::component::filter::ShieldtankComponentFilter;
use crate::component::world_bounds::ShieldtankWorldBounds;

use super::layer::ShieldtankLayer;
use super::level_background::color::ShieldtankLevelBackgroundColor;
use super::level_background::image::LevelBackgroundImage;
use super::shieldtank_component::{ShieldtankComponent, ShieldtankComponentSystemSet};
use super::spawn_children::SpawnChildren;

#[derive(Debug, Component, Reflect)]
#[require(GlobalTransform, Visibility)]
pub struct ShieldtankLevel {
    pub handle: Handle<LevelAsset>,
    pub level_separation: f32,
}

impl Default for ShieldtankLevel {
    fn default() -> Self {
        Self {
            handle: Default::default(),
            level_separation: 10.0,
        }
    }
}

impl AsAssetId for ShieldtankLevel {
    type Asset = LevelAsset;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

impl ShieldtankComponent for ShieldtankLevel {
    fn new(handle: Handle<<Self as AsAssetId>::Asset>) -> Self {
        Self {
            handle,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Component, Reflect)]
pub struct ShieldtankLevelFilter;

impl ShieldtankComponentFilter for ShieldtankLevelFilter {}

impl SpawnChildren for ShieldtankLevel {
    type Child = ShieldtankLayer;
    type Filter = ShieldtankLevelFilter;

    fn get_children(
        &self,
        asset: &LevelAsset,
        _filter: Cow<ShieldtankLevelFilter>,
    ) -> impl Iterator<Item = Handle<LayerInstance>> {
        asset.layers.values().cloned()
    }
}

#[allow(clippy::type_complexity)]
fn level_insert_components_system(
    query: Query<
        (Entity, &ShieldtankLevel, Option<&Transform>),
        Or<(Changed<ShieldtankLevel>, AssetChanged<ShieldtankLevel>)>,
    >,
    assets: Res<Assets<LevelAsset>>,
    mut commands: Commands,
) {
    query
        .iter()
        .filter_map(|(entity, component, transform)| {
            Some((
                entity,
                component,
                transform,
                assets.get(component.as_asset_id())?,
            ))
        })
        .for_each(|(entity, component, transform, asset)| {
            let mut entity_commands = commands.entity(entity);

            match &asset.background {
                Some(background) => {
                    let color = asset.bg_color;
                    let size = asset.size.as_uvec2();

                    let background = LevelBackgroundImage::new(color, size, background);

                    entity_commands.insert(background);
                }
                None => {
                    let background = ShieldtankLevelBackgroundColor {
                        color: asset.bg_color,
                        size: asset.size.as_vec2(),
                    };

                    entity_commands.insert(background);
                }
            };

            if transform.is_none() {
                let location = Vec2::new(1.0, -1.0) * asset.location.as_vec2();
                let z = asset.world_depth as f32 * component.level_separation;
                let translation = location.extend(z);
                let transform = Transform::from_translation(translation);

                entity_commands.insert(transform);
            }
        });
}

fn level_global_bounds_system(
    query: Query<(Entity, &ShieldtankLevel, &GlobalTransform), Changed<GlobalTransform>>,
    assets: Res<Assets<LevelAsset>>,
    mut commands: Commands,
) {
    query
        .iter()
        .filter_map(|(entity, component, global_transform)| {
            Some((
                entity,
                global_transform,
                assets.get(component.as_asset_id())?,
            ))
        })
        .for_each(|(entity, global_transform, asset)| {
            let global_location = global_transform.translation().truncate();
            let size = Vec2::new(1.0, -1.0) * asset.size.as_vec2();
            let rect = Rect::from_corners(global_location, global_location + size);
            let global_bounds = ShieldtankWorldBounds::from(rect);

            commands.entity(entity).insert(global_bounds);
        });
}

pub struct ShieldtankLevelPlugin;
impl Plugin for ShieldtankLevelPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ShieldtankLevel>();
        app.add_systems(
            ShieldtankComponentSystemSet,
            <ShieldtankLevel as ShieldtankComponent>::add_basic_components_system,
        );
        app.add_systems(ShieldtankComponentSystemSet, level_insert_components_system);
        app.add_systems(ShieldtankComponentSystemSet, level_global_bounds_system);
    }
}
