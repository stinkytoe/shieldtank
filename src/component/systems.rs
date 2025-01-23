use bevy_ldtk_asset::prelude::LdtkAssetWithChildren;
use bevy_log::debug;

use crate::commands::ShieldtankCommands;
use crate::query::ShieldtankQuery;

pub fn find_and_mark_loaded_components(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    macro_rules! find_and_mark_loaded {
        ($iter_func:tt, $commands_func:tt, $name:expr) => {
            shieldtank_query
                .$iter_func()
                .filter(|item| !item.is_finalized() && item.asset_is_loaded())
                .for_each(|item| {
                    debug!("Shieldtank {} loaded: {}", $name, item.get_identifier());
                    shieldtank_commands
                        .$commands_func(&item)
                        .mark_finalized(true);
                });
        };
    }

    find_and_mark_loaded!(iter_projects, project, "Project");
    find_and_mark_loaded!(iter_worlds, world, "World");
    find_and_mark_loaded!(iter_levels, level, "Level");
    find_and_mark_loaded!(iter_layers, layer, "Layer");
    find_and_mark_loaded!(iter_entities, entity, "Entity");
}

pub fn find_and_unmark_just_loaded_components(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    macro_rules! find_and_mark_just_loaded {
        ($iter_func:tt, $commands_func:tt, $name:expr) => {
            shieldtank_query
                .$iter_func()
                .filter_just_finalized()
                .for_each(|item| {
                    debug!("Shieldtank {} finalized: {}", $name, item.get_identifier());
                    shieldtank_commands
                        .$commands_func(&item)
                        .mark_finalized(false);
                });
        };
    }

    find_and_mark_just_loaded!(iter_projects, project, "Project");
    find_and_mark_just_loaded!(iter_worlds, world, "World");
    find_and_mark_just_loaded!(iter_levels, level, "Level");
    find_and_mark_just_loaded!(iter_layers, layer, "Layer");
    find_and_mark_just_loaded!(iter_entities, entity, "Entity");
}

// TODO: should we gate this behind a feature?
pub fn insert_name_component(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    macro_rules! insert_name_component {
        ($iter_func:tt, $commands_func:tt, $name:expr) => {
            shieldtank_query
                .$iter_func()
                .filter_just_finalized()
                .for_each(|item| {
                    shieldtank_commands
                        .$commands_func(&item)
                        .insert_name_component(item.get_identifier());
                });
        };
    }

    insert_name_component!(iter_projects, project, "Project");
    insert_name_component!(iter_worlds, world, "World");
    insert_name_component!(iter_levels, level, "Level");
    insert_name_component!(iter_layers, layer, "Layer");
    insert_name_component!(iter_entities, entity, "Entity");
}

use crate::item::iter::ItemIteratorExt as _;
pub fn spawn_children(shieldtank_query: ShieldtankQuery) {
    shieldtank_query
        .iter_projects()
        .filter_just_finalized()
        .for_each(|item| {
            let asset = item.get_asset();
            asset.get_children().for_each(|child| {
                debug!("world: {:?}", child.path());
            });
        });

    // shieldtank_query
    //     .iter_projects()
    //     .filter(|item| item.is_just_finalized())
    //     .for_each(|item| {
    //         let asset = item.get_asset();
    //         asset.get_children().for_each(|child| {
    //             debug!("world: {:?}", child.path());
    //         });
    //     });
}
