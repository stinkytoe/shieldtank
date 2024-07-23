use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::assets::level::LdtkLevelAsset;
use crate::system_params::traits::LdtkIterable;

#[derive(SystemParam)]
pub struct LdtkLevelQuery<'w, 's> {
    level_assets: Res<'w, Assets<LdtkLevelAsset>>,
    level_query: Query<'w, 's, (Entity, &'static Handle<LdtkLevelAsset>)>,
    level_query_added:
        Query<'w, 's, (Entity, &'static Handle<LdtkLevelAsset>), Added<Handle<LdtkLevelAsset>>>,
}

impl<'w, 's> LdtkIterable<'w, 's, LdtkLevelAsset> for LdtkLevelQuery<'w, 's> {
    fn query(&self) -> impl Iterator<Item = (Entity, &Handle<LdtkLevelAsset>)> {
        self.level_query.iter()
    }

    fn query_added(&self) -> impl Iterator<Item = (Entity, &Handle<LdtkLevelAsset>)> {
        self.level_query_added.iter()
    }

    fn get_asset(&self, id: AssetId<LdtkLevelAsset>) -> Option<&LdtkLevelAsset> {
        self.level_assets.get(id)
    }
}
