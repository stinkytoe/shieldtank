use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::prelude::LdtkAsset;

use crate::item::entity::EntityItem;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;
use crate::query::ShieldtankQuery;

macro_rules! make_by_iid {
    ($name:tt, $result_item:ident, $query_field:tt, $assets_field:tt) => {
        pub fn $name(&self, iid: Iid) -> Option<$result_item> {
            self.$query_field
                .iter()
                .find(|((_, component, ..), _)| {
                    let id = component.handle.id();
                    self.$assets_field
                        .get(id)
                        .and_then(|asset| (asset.get_iid() == iid).then_some(()))
                        .is_some()
                })
                .and_then(|(shieldtank_query_data, component_query_data)| {
                    let asset_id = shieldtank_query_data.1.handle.id();
                    let asset = self.$assets_field.get(asset_id)?;
                    let config_id = shieldtank_query_data.1.config.id();
                    let config = self.config_assets.get(config_id)?;

                    Some(Item::new(
                        asset,
                        config,
                        shieldtank_query_data,
                        component_query_data,
                        self,
                    ))
                })
        }
    };
}

impl ShieldtankQuery<'_, '_> {
    make_by_iid!(project_by_iid, ProjectItem, project_query, project_assets);
    make_by_iid!(world_by_iid, WorldItem, world_query, world_assets);
    make_by_iid!(level_by_iid, LevelItem, level_query, level_assets);
    make_by_iid!(layer_by_iid, LayerItem, layer_query, layer_assets);
    make_by_iid!(entity_by_iid, EntityItem, entity_query, entity_assets);
}
