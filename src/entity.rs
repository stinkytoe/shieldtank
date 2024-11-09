use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_utils::error;

use crate::component::{FinalizeEvent, LdtkComponent};
use crate::item::LdtkItem;
use crate::tileset_rectangle::TilesetRectangle;
use crate::{bad_ecs_entity, bad_handle, Result};

pub type Entity = LdtkComponent<EntityAsset>;
pub type EntityItem<'a> = LdtkItem<'a, EntityAsset>;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(Update, entity_finalize_tileset_rectangle.map(error));
    }
}

pub(crate) fn entity_finalize_tileset_rectangle(
    mut commands: Commands,
    mut events: EventReader<FinalizeEvent<EntityAsset>>,
    entity_assets: Res<Assets<EntityAsset>>,
    query: Query<&Entity>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let FinalizeEvent {
            entity: ecs_entity, ..
        } = event;

        let component = query
            .get(*ecs_entity)
            .map_err(|e| bad_ecs_entity!("bad ecs entity! {ecs_entity:?}: {e}"))?;

        let asset = entity_assets
            .get(component.handle.id())
            .ok_or(bad_handle!("bad handle! {:?}", component.handle))?;

        if let Some(tile) = asset.tile.as_ref() {
            commands.entity(*ecs_entity).insert(TilesetRectangle {
                anchor: asset.anchor,
                tile: tile.clone(),
            });
        }

        Ok(())
    })
}
