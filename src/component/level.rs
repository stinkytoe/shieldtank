use bevy_app::Plugin;
use bevy_asset::prelude::AssetChanged;
use bevy_asset::{AsAssetId, Assets, Handle};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::{Commands, Query, Res};
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_math::{Rect, Vec2};
use bevy_reflect::Reflect;
use bevy_render::view::Visibility;
use bevy_transform::components::{GlobalTransform, Transform};

use super::global_bounds::LdtkGlobalBounds;
use super::layer::LdtkLayer;
use super::level_background::color::LevelBackgroundColor;
use super::level_background::image::LevelBackgroundImage;
use super::shieldtank_component::{ShieldtankComponent, ShieldtankComponentSystemSet};
use super::spawn_children::SpawnChildren;

#[derive(Debug, Default, Reflect)]
pub enum LayersToSpawn {
    #[default]
    All,
    None,
}

impl LayersToSpawn {
    fn handle_matches(&self) -> bool {
        match self {
            LayersToSpawn::All => true,
            LayersToSpawn::None => false,
        }
    }
}

#[derive(Debug, Component, Reflect)]
#[require(GlobalTransform, Visibility)]
pub struct LdtkLevel {
    pub handle: Handle<LevelAsset>,
    pub level_separation: f32,
    pub layers_to_spawn: LayersToSpawn,
}

impl Default for LdtkLevel {
    fn default() -> Self {
        Self {
            handle: Default::default(),
            level_separation: 10.0,
            layers_to_spawn: LayersToSpawn::default(),
        }
    }
}

impl AsAssetId for LdtkLevel {
    type Asset = LevelAsset;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

impl ShieldtankComponent for LdtkLevel {
    fn new(handle: Handle<<Self as AsAssetId>::Asset>) -> Self {
        Self {
            handle,
            layers_to_spawn: LayersToSpawn::default(),
            ..Default::default()
        }
    }
}

impl SpawnChildren for LdtkLevel {
    type Child = LdtkLayer;

    fn get_children(
        &self,
        asset: &<Self as AsAssetId>::Asset,
    ) -> impl Iterator<Item = Handle<<Self::Child as AsAssetId>::Asset>> {
        asset
            .layers
            .values()
            .filter(|_| self.layers_to_spawn.handle_matches())
            .cloned()
    }
}

#[allow(clippy::type_complexity)]
fn level_insert_components_system(
    query: Query<
        (Entity, &LdtkLevel, Option<&Transform>),
        Or<(Changed<LdtkLevel>, AssetChanged<LdtkLevel>)>,
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
                    let background = LevelBackgroundColor {
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
    query: Query<(Entity, &LdtkLevel, &GlobalTransform), Changed<GlobalTransform>>,
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
            let global_bounds = LdtkGlobalBounds::from(rect);

            commands.entity(entity).insert(global_bounds);
        });
}

pub struct LdtkLevelPlugin;
impl Plugin for LdtkLevelPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LdtkLevel>();
        app.add_systems(
            ShieldtankComponentSystemSet,
            <LdtkLevel as ShieldtankComponent>::add_basic_components_system,
        );
        app.add_systems(ShieldtankComponentSystemSet, level_insert_components_system);
        app.add_systems(ShieldtankComponentSystemSet, level_global_bounds_system);
    }
}
