use bevy_app::Plugin;
use bevy_asset::prelude::AssetChanged;
use bevy_asset::{AsAssetId, Assets, Handle};
use bevy_camera::visibility::Visibility;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::Query;
use bevy_ecs::system::{Commands, Res};
use bevy_ldtk_asset::entity::EntityInstance;
use bevy_math::{Rect, Vec2};
use bevy_reflect::Reflect;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::component::world_bounds::ShieldtankWorldBounds;

use super::entity_definition::ShieldtankEntityDefinition;
use super::field_instances::ShieldtankFieldInstances;
use super::shieldtank_component::{ShieldtankComponent, ShieldtankComponentSystemSet};
use super::tags::ShieldtankTags;
use super::tile::ShieldtankTile;

#[derive(Debug, Default, Component, Reflect)]
#[require(GlobalTransform, Visibility)]
pub struct ShieldtankEntity {
    pub handle: Handle<EntityInstance>,
}

impl AsAssetId for ShieldtankEntity {
    type Asset = EntityInstance;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

impl ShieldtankComponent for ShieldtankEntity {
    fn new(handle: Handle<<Self as AsAssetId>::Asset>) -> Self {
        Self { handle }
    }
}

#[allow(clippy::type_complexity)]
fn entity_insert_components_system(
    query: Query<
        (
            Entity,
            &ShieldtankEntity,
            Option<&Transform>,
            Option<&ShieldtankTile>,
        ),
        Or<(Changed<ShieldtankEntity>, AssetChanged<ShieldtankEntity>)>,
    >,
    assets: Res<Assets<EntityInstance>>,
    mut commands: Commands,
) {
    query
        .iter()
        .filter_map(|(entity, component, transform, tile)| {
            Some((
                entity,
                transform,
                tile,
                assets.get(component.as_asset_id())?,
            ))
        })
        .for_each(|(entity, transform, tile, asset)| {
            let mut entity_commands = commands.entity(entity);

            let entity_definition = asset.entity_definition.clone();
            let entity_definition = ShieldtankEntityDefinition::new(entity_definition);
            entity_commands.insert(entity_definition);

            if transform.is_none() {
                let location = Vec2::new(1.0, -1.0) * asset.location.as_vec2();
                let translation = location.extend(0.0);
                let transform = Transform::from_translation(translation);
                entity_commands.insert(transform);
            }

            if tile.is_none()
                && let Some(tile) = &asset.tile
            {
                let tile = ShieldtankTile::new(tile);
                entity_commands.insert(tile);
            }

            let field_instances = ShieldtankFieldInstances::new(asset.field_instances.clone());
            entity_commands.insert(field_instances);

            if !asset.tags.is_empty() {
                let tags = ShieldtankTags::new(&asset.tags);
                entity_commands.insert(tags);
            }
        });
}

#[allow(clippy::type_complexity)]
fn entity_global_bounds_system(
    query: Query<
        (Entity, &ShieldtankEntity, &GlobalTransform),
        Or<(
            Changed<ShieldtankEntity>,
            AssetChanged<ShieldtankEntity>,
            Changed<ShieldtankEntityDefinition>,
            AssetChanged<ShieldtankEntityDefinition>,
            Changed<GlobalTransform>,
        )>,
    >,
    assets: Res<Assets<EntityInstance>>,
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
            let size = asset.size.as_vec2();
            let anchor = asset.anchor;
            let offset = anchor.as_vec() * size;
            let rect = Rect::from_center_size(global_location - offset, size);
            let global_bounds = ShieldtankWorldBounds::from(rect);

            commands.entity(entity).insert(global_bounds);
        });
}

#[derive(Clone, Debug, Default)]
pub struct ShieldtankEntityPlugin;

impl Plugin for ShieldtankEntityPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ShieldtankEntity>();
        app.add_systems(
            ShieldtankComponentSystemSet,
            (
                entity_insert_components_system,
                entity_global_bounds_system,
                <ShieldtankEntity as ShieldtankComponent>::add_basic_components_system,
            ),
        );
    }
}
