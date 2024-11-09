use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::layer_definition::LayerDefinition;
use bevy_utils::error;

use crate::component::{FinalizeEvent, LdtkComponent};
use crate::int_grid::IntGrid;
use crate::item::LdtkItem;
use crate::tiles::Tiles;
use crate::{bad_ecs_entity, bad_handle, Result};

pub type Layer = LdtkComponent<LayerAsset>;
pub type LayerItem<'a> = LdtkItem<'a, LayerAsset>;

pub struct LayerPlugin;
impl Plugin for LayerPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(Update, layer_finalize_int_grid_and_tiles.map(error));
    }
}

pub(crate) fn layer_finalize_int_grid_and_tiles(
    mut commands: Commands,
    mut events: EventReader<FinalizeEvent<LayerAsset>>,
    layer_assets: Res<Assets<LayerAsset>>,
    layer_definitions: Res<Assets<LayerDefinition>>,
    query: Query<&Layer>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let FinalizeEvent {
            entity: ecs_entity, ..
        } = event;

        let component = query
            .get(*ecs_entity)
            .map_err(|e| bad_ecs_entity!("bad ecs entity! {ecs_entity:?}: {e}"))?;

        let asset = layer_assets
            .get(component.handle.id())
            .ok_or(bad_handle!("bad handle! {:?}", component.handle))?;

        if let Some(tiles_layer) = asset.layer_type.get_tiles_layer() {
            let mut entity_commands = commands.entity(*ecs_entity);

            if !tiles_layer.tiles.is_empty() {
                let tiles = Tiles::new(tiles_layer);
                entity_commands.insert(tiles);
            }

            if !tiles_layer.int_grid.is_empty() {
                // TODO: Unguarded Assets::get(..) here. Probably fine, but should fix.
                let layer_definition = layer_definitions
                    .get(asset.layer_definition.id())
                    .ok_or(bad_handle!("bad handle! {:?}", asset.layer_definition))?;
                let int_grid = IntGrid::from_layer(asset, layer_definition)?;
                entity_commands.insert(int_grid);
            }
        }

        Ok(())
    })
}
