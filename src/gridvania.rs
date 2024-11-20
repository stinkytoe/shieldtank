use bevy_app::{Plugin, Update};
use bevy_ecs::component::Component;
use bevy_ecs::system::Commands;
use bevy_ldtk_asset::world::WorldLayout;
use bevy_log::debug;
use bevy_math::Vec2;
use bevy_reflect::Reflect;

use crate::item::LdtkItemTrait;
use crate::item_iterator::LdtkItemIterator;
use crate::query::LdtkQuery;

#[derive(Component, Debug, Reflect)]
pub struct GridvaniaToolkit {
    grid_size: Vec2,
}

pub fn register_gridvania_worlds(mut commands: Commands, ldtk_query: LdtkQuery) {
    ldtk_query
        .worlds()
        .filter_changed()
        .inspect(|item| debug!("World added: {}", item.get_identifier()))
        .filter_map(|item| {
            if let WorldLayout::GridVania(grid) = item.asset.world_layout {
                Some((item, grid))
            } else {
                None
            }
        })
        .for_each(|(item, grid)| {
            debug!(
                "Registering Gridvania toolkit for world: {}",
                item.get_identifier()
            );
            commands
                .entity(item.get_ecs_entity())
                .insert(GridvaniaToolkit { grid_size: grid });
        });
}

//pub fn process_gridvania_levels(mut commands: Commands, ldtk_query: LdtkQuery) {}

pub struct GridvaniaPlugin;
impl Plugin for GridvaniaPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<GridvaniaToolkit>()
            .add_systems(Update, register_gridvania_worlds);
    }
}
