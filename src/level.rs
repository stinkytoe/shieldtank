use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
// NOTE: Is this a good idea?
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_log::debug;
use bevy_utils::error;

use crate::component::{FinalizeEvent, LdtkComponent};
use crate::item::LdtkItem;
use crate::level_background::LevelBackground;
use crate::{bad_handle, Result};

pub type Level = LdtkComponent<LevelAsset>;
pub type LevelItem<'a> = LdtkItem<'a, LevelAsset>;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(Update, level_finalize_on_event.map(error));
    }
}

pub(crate) fn level_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<FinalizeEvent<LevelAsset>>,
    level_assets: Res<Assets<LevelAsset>>,
    query: Query<(EcsEntity, &Level)>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let FinalizeEvent {
            entity: event_entity,
            ..
        } = event;

        query
            .iter()
            .filter(|(entity, ..)| entity == event_entity)
            .try_for_each(|data| -> Result<()> { finalize(&mut commands, data, &level_assets) })
    })
}

fn finalize(
    commands: &mut Commands,
    (ecs_entity, level): (EcsEntity, &Level),
    level_assets: &Assets<LevelAsset>,
) -> Result<()> {
    let level_asset = level_assets
        .get(level.handle.id())
        .ok_or(bad_handle!("bad handle! {:?}", level.handle))?;

    let color = level_asset.bg_color;
    let size = level_asset.size;
    let background = level_asset.background.clone();
    let background = LevelBackground {
        color,
        size,
        background,
    };

    commands.entity(ecs_entity).insert(background);

    debug!("Level {:?} finalized!", level_asset.identifier);

    Ok(())
}
