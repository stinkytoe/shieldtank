use bevy::prelude::*;
use std::fmt::Debug;

use crate::assets::layer::LdtkLayerAsset;
use crate::system_params::layer::query::LdtkLayerQuery;
use crate::system_params::traits::LdtkItem;

pub struct LdtkLayer<'w, 's> {
    entity: Entity,
    asset: &'w LdtkLayerAsset,
    query: &'w LdtkLayerQuery<'w, 's>,
}

impl Debug for LdtkLayer<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LdtkLayer")
            .field("entity", &self.entity)
            .field("asset", &self.asset)
            // NOTE: field "query" ignored
            .finish()
    }
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
