use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_utils::error;

use crate::component::{FinalizeEvent, LdtkComponent};
use crate::item::LdtkItem;
use crate::level_background::LevelBackground;
use crate::{bad_ecs_entity, bad_handle, Result};

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
    query: Query<&Level>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let FinalizeEvent {
            entity: ecs_entity, ..
        } = event;

        let component = query
            .get(*ecs_entity)
            .map_err(|e| bad_ecs_entity!("bad ecs entity! {ecs_entity:?}: {e}"))?;

        let asset = level_assets
            .get(component.handle.id())
            .ok_or(bad_handle!("bad handle! {:?}", component.handle))?;

        let level_background = LevelBackground::new(asset);

        commands.entity(*ecs_entity).insert(level_background);

        Ok(())
    })
}
