use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::layer_definition::LayerDefinition;
use bevy_log::debug;
use bevy_utils::error;

use crate::component::{FinalizeEvent, LdtkComponent};
use crate::int_grid::IntGrid;
use crate::item::LdtkItem;
use crate::tiles::Tiles;
use crate::{bad_handle, Result};

pub type Layer = LdtkComponent<LayerAsset>;
pub type LayerItem<'a> = LdtkItem<'a, LayerAsset>;

pub struct LayerPlugin;
impl Plugin for LayerPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(Update, layer_finalize_on_event.map(error));
    }
}

pub(crate) fn layer_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<FinalizeEvent<LayerAsset>>,
    layer_assets: Res<Assets<LayerAsset>>,
    layer_definitions: Res<Assets<LayerDefinition>>,
    query: Query<(EcsEntity, &Layer)>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let FinalizeEvent {
            entity: event_entity,
            ..
        } = event;

        query
            .iter()
            .filter(|(entity, ..)| entity == event_entity)
            .try_for_each(|data| -> Result<()> {
                finalize(&mut commands, data, &layer_assets, &layer_definitions)
            })
    })
}

fn finalize(
    commands: &mut Commands,
    (ecs_entity, layer): (EcsEntity, &Layer),
    layer_assets: &Assets<LayerAsset>,
    layer_definitions: &Assets<LayerDefinition>,
) -> Result<()> {
    let layer_asset = layer_assets
        .get(layer.handle.id())
        .ok_or(bad_handle!("bad handle! {:?}", layer.handle))?;

    let mut entity_commands = commands.entity(ecs_entity);

    if let Some(tiles_layer) = layer_asset.layer_type.get_tiles_layer() {
        // TODO: Unguarded Assets::get(..) here. Probably fine, but should fix.
        let layer_definition = layer_definitions
            .get(layer_asset.layer_definition.id())
            .ok_or(bad_handle!(
                "bad handle! {:?}",
                layer_asset.layer_definition
            ))?;
        let int_grid = IntGrid::from_layer(layer_asset, layer_definition)?;
        let tiles = Tiles::new(tiles_layer);
        entity_commands.insert((int_grid, tiles));
    }

    debug!("Layer {:?} finalized!", layer_asset.identifier);

    Ok(())
}
