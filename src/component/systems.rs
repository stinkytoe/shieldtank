use bevy_core::Name;
use bevy_ecs::system::Commands;
use bevy_hierarchy::BuildChildren;
use bevy_ldtk_asset::prelude::LdtkAssetWithChildren;
use bevy_log::debug;

use crate::commands::ShieldtankCommands;
use crate::component::entity::EntityComponent;
use crate::component::layer::LayerComponent;
use crate::component::level::LevelComponent;
use crate::component::world::WorldComponent;
use crate::component::ShieldtankComponentFinalized;
use crate::item::iter::ItemIteratorExt as _;
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
                    shieldtank_commands.$commands_func(&item).insert(
                        ShieldtankComponentFinalized {
                            just_finalized: true,
                        },
                    );
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
                    shieldtank_commands.$commands_func(&item).insert(
                        ShieldtankComponentFinalized {
                            just_finalized: false,
                        },
                    );
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
                        .insert(Name::new(item.get_identifier().to_string()));
                });
        };
    }

    insert_name_component!(iter_projects, project, "Project");
    insert_name_component!(iter_worlds, world, "World");
    insert_name_component!(iter_levels, level, "Level");
    insert_name_component!(iter_layers, layer, "Layer");
    insert_name_component!(iter_entities, entity, "Entity");
}

pub fn spawn_children(mut commands: Commands, shieldtank_query: ShieldtankQuery) {
    macro_rules! lala {
        ($iter_func_parent:tt, $iter_func_children:tt, $Component:tt) => {
            shieldtank_query
                .$iter_func_parent()
                .filter_just_finalized()
                .for_each(|item| {
                    let asset = item.get_asset();

                    asset.get_children().for_each(|child| {
                        let config = item.get_config();

                        if config.get_load_pattern().handle_matches_pattern(child)
                            && !shieldtank_query
                                .$iter_func_children()
                                .any(|item| item.get_asset_handle() == *child)
                        {
                            commands
                                .entity(item.get_ecs_entity())
                                .with_child($Component {
                                    handle: child.clone(),
                                    config: item.get_config_handle().clone(),
                                });
                        }
                    });
                });
        };
    }

    lala!(iter_projects, iter_worlds, WorldComponent);
    lala!(iter_worlds, iter_levels, LevelComponent);
    lala!(iter_levels, iter_layers, LayerComponent);
    lala!(iter_layers, iter_entities, EntityComponent);
}
