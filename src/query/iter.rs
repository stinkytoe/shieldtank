use crate::item::entity::EntityItem;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;
use crate::query::ShieldtankQuery;

macro_rules! make_iter {
    ($name:tt, $iter_item:ident, $query_field:tt, $assets_field:tt) => {
        pub fn $name(&self) -> impl Iterator<Item = $iter_item> {
            self.$query_field.iter().filter_map(
                |(shieldtank_query_data, component_query_data)| -> Option<_> {
                    let asset_id = shieldtank_query_data.1.handle.id();
                    let config_id = shieldtank_query_data.1.config.id();

                    let asset = self.$assets_field.get(asset_id)?;
                    let config = self.config_assets.get(config_id)?;

                    Some(Item::new(
                        asset,
                        config,
                        shieldtank_query_data,
                        component_query_data,
                        self,
                    ))
                },
            )
        }
    };
}

impl ShieldtankQuery<'_, '_> {
    make_iter!(iter_projects, ProjectItem, project_query, project_assets);
    make_iter!(iter_worlds, WorldItem, world_query, world_assets);
    make_iter!(iter_levels, LevelItem, level_query, level_assets);
    make_iter!(iter_layers, LayerItem, layer_query, layer_assets);
    make_iter!(iter_entities, EntityItem, entity_query, entity_assets);
}
