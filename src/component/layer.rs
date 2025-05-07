use bevy_app::Plugin;
use bevy_asset::prelude::AssetChanged;
use bevy_asset::{AsAssetId, Assets, Handle};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::{Commands, Query, Res};
use bevy_ldtk_asset::layer::EntitiesLayer;
use bevy_ldtk_asset::layer::LayerInstance;
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_render::view::Visibility;
use bevy_transform::components::{GlobalTransform, Transform};
use either::Either;

use super::entity::LdtkEntity;
use super::global_bounds::LdtkGlobalBounds;
use super::layer_definition::LdtkLayerDefinition;
use super::layer_tiles::LdtkLayerTiles;
use super::shieldtank_component::{ShieldtankComponent, ShieldtankComponentSystemSet};
use super::spawn_children::SpawnChildren;

#[derive(Debug, Component, Reflect)]
#[require(GlobalTransform, Visibility)]
pub struct LdtkLayer {
    pub handle: Handle<LayerInstance>,
    pub layer_separation: f32,
}

impl Default for LdtkLayer {
    fn default() -> Self {
        Self {
            handle: Default::default(),
            layer_separation: 0.1,
        }
    }
}

impl AsAssetId for LdtkLayer {
    type Asset = LayerInstance;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

impl ShieldtankComponent for LdtkLayer {
    fn new(handle: Handle<<Self as bevy_asset::AsAssetId>::Asset>) -> Self {
        Self {
            handle,
            ..Default::default()
        }
    }
}

impl SpawnChildren for LdtkLayer {
    type Child = LdtkEntity;

    fn get_children(
        &self,
        asset: &<Self as AsAssetId>::Asset,
    ) -> impl Iterator<Item = Handle<<Self::Child as AsAssetId>::Asset>> {
        if let Some(EntitiesLayer { entities, .. }) = asset.layer_type.get_entities_layer() {
            Either::Left(entities.values().cloned())
        } else {
            Either::Right(vec![].into_iter())
        }
    }
}

#[allow(clippy::type_complexity)]
fn layer_insert_components_system(
    query: Query<
        (Entity, &LdtkLayer, Option<&Transform>),
        Or<(Changed<LdtkLayer>, AssetChanged<LdtkLayer>)>,
    >,
    assets: Res<Assets<LayerInstance>>,
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

            if let Some(tiles_layer) = asset.layer_type.get_tiles_layer() {
                if !tiles_layer.tiles.is_empty() {
                    let layer_tiles = LdtkLayerTiles::new(asset, tiles_layer);

                    entity_commands.insert(layer_tiles);
                }

                let layer_definition = asset.layer_definition.clone();
                let layer_definition = LdtkLayerDefinition::new(layer_definition);

                entity_commands.insert(layer_definition);
            }

            if transform.is_none() {
                let location = Vec2::new(1.0, -1.0) * asset.location.as_vec2();
                let z = (asset.index + 1) as f32 * component.layer_separation;
                let translation = location.extend(z);
                let transform = Transform::from_translation(translation);

                entity_commands.insert(transform);
            }
        });
}

fn layer_global_bounds_system(
    query: Query<(Entity, &LdtkLayer, &GlobalTransform), Changed<GlobalTransform>>,
    assets: Res<Assets<LayerInstance>>,
    mut commands: Commands,
) {
    query
        .iter()
        .filter_map(|(entity, component, global_transform)| {
            Some((
                entity,
                assets.get(component.as_asset_id())?,
                global_transform,
            ))
        })
        .for_each(|(entity, asset, global_transform)| {
            let global_location = global_transform.translation().truncate();
            let size = asset.grid_size * asset.grid_cell_size;
            let size = Vec2::new(1.0, -1.0) * size.as_vec2();
            let global_bounds = LdtkGlobalBounds::new(global_location, global_location + size);

            commands.entity(entity).insert(global_bounds);
        });
}

pub struct LdtkLayerPlugin;
impl Plugin for LdtkLayerPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LdtkLayer>();
        app.add_systems(ShieldtankComponentSystemSet, layer_insert_components_system);
        app.add_systems(ShieldtankComponentSystemSet, layer_global_bounds_system);
        app.add_systems(
            ShieldtankComponentSystemSet,
            <LdtkLayer as ShieldtankComponent>::add_basic_components_system,
        );
    }
}
