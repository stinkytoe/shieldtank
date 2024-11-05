use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::entity::Entity as EcsEntity; // NOTE: Is this a good idea?
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Res};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_log::debug;
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::component::{DoFinalizeEvent, LdtkComponent, LdtkComponentExt};
use crate::tileset_rectangle::TilesetRectangle;
use crate::{bad_handle, Result};

pub type Entity = LdtkComponent<EntityAsset>;

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
