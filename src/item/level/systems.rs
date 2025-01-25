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
                .get_load_level_background_pattern()
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
