use bevy::prelude::*;

use crate::assets::entity::LdtkEntityAsset;
use crate::prelude::{LdtkQuery, LdtkQueryEx};
use crate::reexports::field_instance::FieldInstance;
use crate::system_params::entity::query::LdtkEntityQuery;
use crate::system_params::layer::item::LdtkLayer;
use crate::system_params::traits::LdtkItem;

use super::query::LdtkEntityQueryError;

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

    pub fn get_parent_layer(&'w self) -> Result<LdtkLayer, LdtkEntityQueryError> {
        let entity = self.ecs_entity();
        let layer_entity = self.query.parent_query.get(entity)?.get();
        let ldtk_layer: LdtkLayer = self
            .query
            .layer_query
            .iter()
            .find_entity(layer_entity)
            .ok_or(LdtkEntityQueryError::NoLayerParent)?;
        Ok(ldtk_layer)
    }

    pub fn grid(&'w self) -> IVec2 {
        let entity = self.ecs_entity();
        let asset = self.asset();
        let translation = self
            .query
            .transform_query
            .get(entity)
            .expect("an entity with Handle<LdtkEntity> component")
            .translation
            .truncate();
        let ldtk_layer: LdtkLayer<'_, '_> = self.get_parent_layer().expect("a layer asset");

        let anchor_vec = asset.anchor.as_vec();
        let focus = Vec2::new(1.0, -1.0) * (translation - anchor_vec);
        let focus = focus.as_ivec2();
        let grid_size = ldtk_layer.asset().grid_size as i32;

        focus / grid_size
    }
}
