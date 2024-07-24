use bevy::prelude::*;
use std::fmt::Debug;

use crate::assets::layer::LdtkLayerAsset;
use crate::reexports::int_grid_value::IntGridValue;
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

impl LdtkLayer<'_, '_> {
    pub fn get_int_grid_value_at(&self, coord: IVec2) -> Option<&IntGridValue> {
        (!self.asset.int_grid_csv.is_empty()
            && coord.x >= 0
            && coord.y >= 0
            && coord.x < self.asset.grid_dimensions.x as i32
            && coord.y < self.asset.grid_dimensions.y as i32)
            .then_some(coord.x + coord.y * self.asset.grid_dimensions.x as i32)
            .map(|index| index as usize)
            .filter(|&index| index < self.asset.int_grid_csv.len())
            .map(|index| self.asset.int_grid_csv[index])
            .filter(|&value| value != 0)
            .and_then(|value| {
                self.query
                    .project_query
                    .get(self.asset.iid)
                    .expect("a project asset")
                    .layer_defs
                    .get(&self.asset.layer_def_uid)
                    .expect("a layer definition")
                    .int_grid_values
                    .iter()
                    .find(|int_grid| int_grid.value == value)
            })
    }
}
