use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::assets::layer::LdtkLayerAsset;
use crate::system_params::traits::LdtkIterable;

#[derive(SystemParam)]
pub struct LdtkLayerQuery<'w, 's> {
    layer_assets: Res<'w, Assets<LdtkLayerAsset>>,
    layer_query: Query<'w, 's, (Entity, &'static Handle<LdtkLayerAsset>)>,
    layer_query_added:
        Query<'w, 's, (Entity, &'static Handle<LdtkLayerAsset>), Added<Handle<LdtkLayerAsset>>>,
}

impl<'w, 's> LdtkIterable<'w, 's, LdtkLayerAsset> for LdtkLayerQuery<'w, 's> {
    fn query(&self) -> impl Iterator<Item = (Entity, &Handle<LdtkLayerAsset>)> {
        self.layer_query.iter()
    }

    fn query_added(&self) -> impl Iterator<Item = (Entity, &Handle<LdtkLayerAsset>)> {
        self.layer_query_added.iter()
    }

    fn get_asset(&self, id: AssetId<LdtkLayerAsset>) -> Option<&LdtkLayerAsset> {
        self.layer_assets.get(id)
    }
}
