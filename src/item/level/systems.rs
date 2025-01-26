use bevy_math::Vec2;

use crate::commands::ShieldtankCommands;
use crate::item::iter::ItemIteratorExt as _;
use crate::level_background::LevelBackground;
use crate::query::ShieldtankQuery;

pub(crate) fn level_spawn_system(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    shieldtank_query
        .iter_levels()
        .filter_just_finalized()
        .for_each(|item| {
            let asset_handle = item.get_asset_handle();
            let config = item.get_config();
            if config
                .get_load_level_background()
                .handle_matches_pattern(&asset_handle)
            {
                let asset = item.get_asset();
                let level_background = LevelBackground::new(asset);
                shieldtank_commands
                    .level(&item)
                    .insert_level_background(level_background);
            }
        });
}

pub(crate) fn level_override_transform_system(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    shieldtank_query
        .iter_levels()
        .filter_just_finalized()
        .for_each(|item| {
            let config = item.get_config();

            if config.levels_override_transform() {
                let asset = item.get_asset();
                let location = Vec2::new(1.0, -1.0) * asset.location.as_vec2();
                let z = asset.world_depth as f32 * config.level_separation();
                let transform = item.get_transform().with_translation(location.extend(z));

                shieldtank_commands
                    .level(&item)
                    .insert_transform_component(transform);
            }
        });
}
