use bevy_ecs::entity::Entity;

use crate::error::Result;
use crate::item::entity::EntityItem;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;
use crate::query::{shieldtank_error, ShieldtankQuery};

macro_rules! make_getter {
    ($name:tt, $result_item:ident, $query_field:tt, $assets_field:tt) => {
        pub fn $name(&self, ecs_entity: Entity) -> Result<$result_item> {
            self.$query_field
                .get(ecs_entity)
                .map(|(shieldtank_query_data, component_query_data)| {
                    let asset_id = shieldtank_query_data.1.handle.id();
                    let config_id = shieldtank_query_data.1.config.id();

                    let asset = self
                        .$assets_field
                        .get(asset_id)
                        .ok_or(shieldtank_error!("Bad asset handle! {asset_id:?}",))?;

                    let config = self
                        .config_assets
                        .get(config_id)
                        .ok_or(shieldtank_error!("Bad config handle! {asset_id:?}",))?;

                    Ok(Item::new(
                        asset,
                        config,
                        shieldtank_query_data,
                        component_query_data,
                        self,
                    ))
                })
                .map_err(|e| shieldtank_error!("Bad query! {e} {ecs_entity:?}",))?
        }
    };
}

impl ShieldtankQuery<'_, '_> {
    make_getter!(get_project, ProjectItem, project_query, project_assets);
    make_getter!(get_world, WorldItem, world_query, world_assets);
    make_getter!(get_level, LevelItem, level_query, level_assets);
    make_getter!(get_layer, LayerItem, layer_query, layer_assets);
    make_getter!(get_entity, EntityItem, entity_query, entity_assets);
}
