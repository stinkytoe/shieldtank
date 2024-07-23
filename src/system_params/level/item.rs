use bevy::prelude::*;
use std::fmt::Debug;

use crate::assets::level::LdtkLevelAsset;
use crate::system_params::level::query::LdtkLevelQuery;
use crate::system_params::traits::LdtkItem;

pub struct LdtkLevel<'w, 's> {
    entity: Entity,
    asset: &'w LdtkLevelAsset,
    query: &'w LdtkLevelQuery<'w, 's>,
}

impl Debug for LdtkLevel<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LdtkLevel")
            .field("entity", &self.entity)
            .field("asset", &self.asset)
            // NOTE: field "query" ignored
            .finish()
    }
}

impl<'w, 's> LdtkItem<'w, 's, LdtkLevelAsset, LdtkLevelQuery<'w, 's>> for LdtkLevel<'w, 's> {
    fn new(entity: Entity, asset: &'w LdtkLevelAsset, query: &'w LdtkLevelQuery<'w, 's>) -> Self {
        Self {
            entity,
            asset,
            query,
        }
    }

    fn ecs_entity(&self) -> Entity {
        self.entity
    }

    fn asset(&self) -> &LdtkLevelAsset {
        self.asset
    }

    fn query(&self) -> &LdtkLevelQuery<'w, 's> {
        self.query
    }
}
