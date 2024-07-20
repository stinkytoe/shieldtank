use bevy::prelude::*;

use crate::assets::entity::LdtkEntityAsset;
use crate::reexports::field_instance::FieldInstance;

pub struct LdtkEntity<'w> {
    pub(crate) entity: Entity,
    pub(crate) asset: &'w LdtkEntityAsset,
}

impl<'w> LdtkEntity<'w> {
    pub fn get_field_instance(&self, identifier: &str) -> Option<&FieldInstance> {
        self.asset
            .field_instances
            .iter()
            .find(|field_instance| field_instance.identifier == identifier)
    }

    pub fn ecs_entity(&self) -> Entity {
        self.entity
    }

    pub fn asset(&self) -> &LdtkEntityAsset {
        self.asset
    }
    pub(crate) fn new(entity: Entity, asset: &'w LdtkEntityAsset) -> Self {
        Self { entity, asset }
    }
}
