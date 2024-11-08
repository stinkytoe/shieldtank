use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Res};
// NOTE: Is this a good idea?
use bevy_ecs::world::Ref;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_log::debug;
use bevy_math::Vec2;
use bevy_render::view::Visibility;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::component::{DoFinalizeEvent, LdtkComponent, LdtkComponentExt};
use crate::item::LdtkItem;
use crate::level::LevelItem;
use crate::query::LdtkQuery;
use crate::tileset_rectangle::TilesetRectangle;
use crate::{bad_handle, Result};

pub type Entity = LdtkComponent<EntityAsset>;
pub type EntityItem<'a> = LdtkItem<'a, EntityAsset, EntityData<'a>>;
pub(crate) type EntityData<'a> = (EcsEntity, Ref<'a, Entity>, Ref<'a, GlobalTransform>);

impl EntityItem<'_> {
    pub fn get_ecs_entity(&self) -> EcsEntity {
        self.data.0
    }

    pub fn get_global_transform(&self) -> &GlobalTransform {
        &self.data.2
    }
}

impl EntityItem<'_> {
    pub fn get_global_location(&self) -> Vec2 {
        self.get_global_transform().translation().truncate()
    }

    pub fn get_level(&self) -> Option<LevelItem> {
        todo!()
    }
}

impl EntityItem<'_> {
    pub(crate) fn make_entity_iterator<'a>(
        query: &'a LdtkQuery,
    ) -> impl Iterator<Item = EntityItem<'a>> {
        query
            .entities_query
            .iter()
            .filter_map(|data| {
                query
                    .entity_assets
                    .get(data.1.handle.id())
                    .map(|asset| (asset, data))
            })
            .map(|(asset, data)| EntityItem {
                asset,
                data,
                _query: query,
            })
    }

    pub(crate) fn get_entity<'a>(
        query: &'a LdtkQuery,
        ecs_entity: EcsEntity,
    ) -> Option<EntityItem<'a>> {
        query
            .entities_query
            .get(ecs_entity)
            .ok()
            .and_then(|data| {
                query
                    .entity_assets
                    .get(data.1.handle.id())
                    .map(|asset| (asset, data))
            })
            .map(|(asset, data)| EntityItem {
                asset,
                data,
                _query: query,
            })
    }
}

pub(crate) fn entity_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<DoFinalizeEvent<EntityAsset>>,
    entity_assets: Res<Assets<EntityAsset>>,
    query: Query<(EcsEntity, &Entity)>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let DoFinalizeEvent {
            entity: event_entity,
            ..
        } = event;

        query
            .iter()
            .filter(|(ecs_entity, ..)| ecs_entity == event_entity)
            .try_for_each(|data| -> Result<()> { finalize(&mut commands, data, &entity_assets) })
    })
}

fn finalize(
    commands: &mut Commands,
    (ecs_entity, entity): (EcsEntity, &Entity),
    entity_assets: &Assets<EntityAsset>,
) -> Result<()> {
    let entity_asset = entity_assets
        .get(entity.get_handle().id())
        .ok_or(bad_handle!(entity.get_handle()))?;

    let name = Name::from(entity_asset.identifier.clone());

    let transform = Transform::from_translation(entity_asset.location.extend(0.0));

    let visibility = Visibility::default();

    let mut entity_commands = commands.entity(ecs_entity);

    entity_commands.insert((name, transform, visibility));

    if let Some(tile) = entity_asset.tile.as_ref() {
        entity_commands.insert(TilesetRectangle {
            anchor: entity_asset.anchor,
            tile: tile.clone(),
        });
    }

    debug!(
        "Entity {}@{} finalized!",
        entity_asset.identifier, entity_asset.iid
    );

    Ok(())
}
