use bevy::prelude::*;

use crate::assets::layer::LdtkLayerAsset;
use crate::system_params::layer::query::LdtkLayerQuery;
use crate::system_params::traits::LdtkItem;

pub struct LdtkLayer<'w, 's> {
    entity: Entity,
    asset: &'w LdtkLayerAsset,
    query: &'w LdtkLayerQuery<'w, 's>,
}

impl<'w, 's> LdtkItem<'w, 's, LdtkLayerAsset, LdtkLayerQuery<'w, 's>> for LdtkLayer<'w, 's> {
    fn new(entity: Entity, asset: &'w LdtkLayerAsset, query: &'w LdtkLayerQuery<'w, 's>) -> Self {
        Self {
            entity,
            asset,
            query,
        }
    }
    fn ecs_entity(&self) -> Entity {
        self.entity
    }

    fn asset(&self) -> &LdtkLayerAsset {
        self.asset
    }

    fn query(&self) -> &LdtkLayerQuery<'w, 's> {
        self.query
    }
}
