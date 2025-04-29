use bevy_app::Plugin;
use bevy_asset::prelude::AssetChanged;
use bevy_asset::{AsAssetId, Assets, Handle};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::Query;
use bevy_ecs::system::{Commands, Res};
use bevy_ldtk_asset::entity::EntityInstance;
use bevy_math::{Rect, Vec2};
use bevy_reflect::Reflect;
use bevy_render::view::Visibility;
use bevy_transform::components::{GlobalTransform, Transform};

use super::entity_definition::LdtkEntityDefinition;
use super::field_instances::LdtkFieldInstances;
use super::global_bounds::LdtkGlobalBounds;
use super::shieldtank_component::{ShieldtankComponent, ShieldtankComponentSystemSet};
use super::tags::LdtkTags;
use super::tile::LdtkTile;

#[derive(Debug, Default, Component, Reflect)]
#[require(GlobalTransform, Visibility)]
pub struct LdtkEntity {
    pub handle: Handle<EntityInstance>,
}

impl AsAssetId for LdtkEntity {
    type Asset = EntityInstance;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

impl ShieldtankComponent for LdtkEntity {
    fn new(handle: Handle<<Self as AsAssetId>::Asset>) -> Self {
        Self { handle }
    }
}

#[allow(clippy::type_complexity)]
fn entity_insert_components_system(
    query: Query<(Entity, &LdtkEntity), Or<(Changed<LdtkEntity>, AssetChanged<LdtkEntity>)>>,
    assets: Res<Assets<EntityInstance>>,
    mut commands: Commands,
) {
    query
        .iter()
        .filter_map(|(entity, component)| Some((entity, assets.get(component.as_asset_id())?)))
        .for_each(|(entity, asset)| {
            let mut entity_commands = commands.entity(entity);

            let entity_definition = asset.entity_definition.clone();
            let entity_definition = LdtkEntityDefinition::new(entity_definition);
            entity_commands.insert(entity_definition);

            if let Some(tile) = &asset.tile {
                let tile = LdtkTile::new(tile);
                entity_commands.insert(tile);
            }

            let location = Vec2::new(1.0, -1.0) * asset.location.as_vec2();
            let translation = location.extend(0.0);
            let transform = Transform::from_translation(translation);
            entity_commands.insert(transform);

            let field_instances = LdtkFieldInstances::new(asset.field_instances.clone());
            entity_commands.insert(field_instances);

            if !asset.tags.is_empty() {
                let tags = LdtkTags::new(&asset.tags);
                entity_commands.insert(tags);
            }
        });
}

#[allow(clippy::type_complexity)]
fn entity_global_bounds_system(
    query: Query<
        (Entity, &LdtkEntity, &GlobalTransform),
        Or<(
            Changed<LdtkEntity>,
            AssetChanged<LdtkEntity>,
            Changed<LdtkEntityDefinition>,
            AssetChanged<LdtkEntityDefinition>,
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
            let rect = Rect::from_center_size(global_location, size);
            let global_bounds = LdtkGlobalBounds::from(rect);

            commands.entity(entity).insert(global_bounds);
        });
}

pub struct LdtkEntityPlugin;
impl Plugin for LdtkEntityPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LdtkEntity>();
        app.add_systems(
            ShieldtankComponentSystemSet,
            (entity_insert_components_system, entity_global_bounds_system),
        );
        app.add_systems(
            ShieldtankComponentSystemSet,
            <LdtkEntity as ShieldtankComponent>::add_basic_components_system,
        );
    }
}
