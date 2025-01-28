use bevy_log::error;
use bevy_math::Vec2;

use crate::commands::ShieldtankCommands;
use crate::error::Result;
use crate::int_grid::IntGrid;
use crate::item::iter::ItemIteratorExt;
use crate::item::layer::iter::LayerItemIteratorExt;
use crate::query::ShieldtankQuery;
use crate::shieldtank_error;
use crate::tiles::Tiles;

pub(crate) fn layer_spawn_system(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    shieldtank_query
        .iter_layers()
        .filter_just_finalized()
        .filter_tiles_layer()
        .map(|item| -> Result<()> {
            let asset_handle = item.get_asset_handle();
            let config = item.get_config();
            if config
                .get_layer_tiles()
                .handle_matches_pattern(&asset_handle)
            {
                let asset = item.get_asset();

                let Some(tiles_layer) = asset.layer_type.get_tiles_layer() else {
                    return Ok(());
                };

                if !tiles_layer.tiles.is_empty() {
                    let tiles = Tiles::new(tiles_layer);
                    shieldtank_commands.layer(&item).insert(tiles);
                }

                if !tiles_layer.int_grid.is_empty() {
                    let layer_definition = shieldtank_query
                        .get_layer_definition(asset.layer_definition.id())
                        .ok_or(shieldtank_error!(
                            "bad Handle<LayerDefinition>! {:?}",
                            asset.layer_definition
                        ))?;

                    let int_grid = IntGrid::from_layer(asset, layer_definition)?;
                    shieldtank_commands.layer(&item).insert(int_grid);
                }
            }

            Ok(())
        })
        .for_each(|ret| {
            // TODO: We're just printing the error and moving on to the next layer.
            // Should we do something else?
            if let Err(e) = ret {
                error!("failed to load layer: {e}");
            }
        });
}

pub(crate) fn layer_override_transform_system(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    shieldtank_query
        .iter_layers()
        .filter_just_finalized()
        .for_each(|item| {
            let config = item.get_config();

            if config.layers_override_transform() {
                let asset = item.get_asset();
                let location = Vec2::new(1.0, -1.0) * asset.location.as_vec2();
                let z = (asset.index + 1) as f32 * config.layer_separation();
                let transform = item.get_transform().with_translation(location.extend(z));

                shieldtank_commands.layer(&item).insert(transform);
            }
        });
}
