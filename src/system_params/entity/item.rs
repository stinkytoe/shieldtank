use bevy::prelude::*;

use crate::assets::entity::LdtkEntityAsset;
use crate::reexports::field_instance::FieldInstance;
use crate::system_params::traits::LdtkItem;

pub struct LdtkEntity<'w> {
    pub(crate) entity: Entity,
    pub(crate) asset: &'w LdtkEntityAsset,
}

impl<'w> LdtkItem<'w, LdtkEntityAsset> for LdtkEntity<'w> {
    fn new(entity: Entity, asset: &'w LdtkEntityAsset) -> Self {
        Self { entity, asset }
    }

    fn ecs_entity(&self) -> Entity {
        self.entity
    }

    fn asset(&self) -> &LdtkEntityAsset {
        self.asset
    }
}

impl<'w> LdtkEntity<'w> {
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
