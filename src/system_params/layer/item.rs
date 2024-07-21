use bevy::prelude::*;

use crate::assets::layer::LdtkLayerAsset;
use crate::system_params::traits::LdtkItem;

pub struct LdtkLayer<'w> {
    entity: Entity,
    asset: &'w LdtkLayerAsset,
}

impl<'w> LdtkItem<'w, LdtkLayerAsset> for LdtkLayer<'w> {
    fn new(entity: Entity, asset: &'w LdtkLayerAsset) -> Self {
        Self { entity, asset }
    }

    fn ecs_entity(&self) -> Entity {
        self.entity
    }

    fn asset(&self) -> &LdtkLayerAsset {
        self.asset
    }
}
