use bevy_math::Vec2;

use crate::commands::ShieldtankCommands;
use crate::item::iter::ItemIteratorExt;
use crate::query::ShieldtankQuery;
use crate::tileset_rectangle::TilesetRectangle;

pub(crate) fn entity_spawn_system(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    shieldtank_query
        .iter_entities()
        .filter_just_finalized()
        .for_each(|item| {
            let asset_handle = item.get_asset_handle();
            let config = item.get_config();
            if config
                .get_load_entity_tileset_rectangles()
                .handle_matches_pattern(&asset_handle)
            {
                let asset = item.get_asset();

                let Some(ldtk_tile) = asset.tile.as_ref().cloned() else {
                    return;
                };

                let tile = TilesetRectangle::new(ldtk_tile);

                shieldtank_commands.entity(&item).insert(tile);
            }
        });
}

pub(crate) fn entity_override_transform_system(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    shieldtank_query
        .iter_entities()
        .filter_just_finalized()
        .for_each(|item| {
            let config = item.get_config();

            if config.entities_override_transform() {
                let asset = item.get_asset();
                let location = Vec2::new(1.0, -1.0) * asset.location.as_vec2();
                let transform = item.get_transform().with_translation(location.extend(0.0));

                shieldtank_commands.entity(&item).insert(transform);
            }
        });
}
