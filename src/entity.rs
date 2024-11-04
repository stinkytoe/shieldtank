use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::entity::Entity as EcsEntity; // NOTE: Is this a good idea?
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Res};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::entity_definition::EntityDefinition;
use bevy_log::debug;

use crate::component::{DoFinalizeEvent, LdtkComponent, LdtkComponentExt};
use crate::project_config::ProjectConfig;
use crate::{bad_handle, Result};

pub type Entity = LdtkComponent<EntityAsset>;

pub(crate) fn entity_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<DoFinalizeEvent<EntityAsset>>,
    entity_assets: Res<Assets<EntityAsset>>,
    config_assets: Res<Assets<ProjectConfig>>,
    entity_definitions: Res<Assets<EntityDefinition>>,
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
            .try_for_each(|data| -> Result<()> {
                finalize(
                    &mut commands,
                    data,
                    &entity_assets,
                    &config_assets,
                    &entity_definitions,
                )
            })
    })
}

fn finalize(
    _commands: &mut Commands,
    (_ecs_entity, entity): (EcsEntity, &Entity),
    entity_assets: &Assets<EntityAsset>,
    config_assets: &Assets<ProjectConfig>,
    _entity_definitions: &Assets<EntityDefinition>,
) -> Result<()> {
    let entity_asset = entity_assets
        .get(entity.get_handle().id())
        .ok_or(bad_handle!(entity.get_handle()))?;

    let _project_config = config_assets
        .get(entity.get_config_handle().id())
        .ok_or(bad_handle!(entity.get_config_handle()))?;

    let _name = Name::from(entity_asset.identifier.clone());

    debug!(
        "Entity {}@{} finalized!",
        entity_asset.identifier, entity_asset.iid
    );

    Ok(())
}
