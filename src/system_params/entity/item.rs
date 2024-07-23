use bevy::prelude::*;

use crate::assets::entity::LdtkEntityAsset;
use crate::reexports::field_instance::FieldInstance;
use crate::system_params::entity::query::LdtkEntityQuery;
use crate::system_params::traits::LdtkItem;

pub struct LdtkEntity<'w, 's> {
    pub(crate) entity: Entity,
    pub(crate) asset: &'w LdtkEntityAsset,
    pub(crate) query: &'w LdtkEntityQuery<'w, 's>,
}

impl<'w, 's> LdtkItem<'w, 's, LdtkEntityAsset, LdtkEntityQuery<'w, 's>> for LdtkEntity<'w, 's> {
    fn new(entity: Entity, asset: &'w LdtkEntityAsset, query: &'w LdtkEntityQuery<'w, 's>) -> Self {
        Self {
            entity,
            asset,
            query,
        }
    }

    fn ecs_entity(&self) -> Entity {
        self.entity
    }

    fn asset(&self) -> &LdtkEntityAsset {
        self.asset
    }

    fn query(&self) -> &LdtkEntityQuery<'w, 's> {
        self.query
    }
}

impl<'w, 's> LdtkEntity<'w, 's> {
    pub fn get_field_instance(&self, identifier: &str) -> Option<&FieldInstance> {
        self.asset
            .field_instances
            .iter()
            .find(|field_instance| field_instance.identifier == identifier)
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.asset.tags.iter().any(|inner_tag| inner_tag == tag)
    }
}
