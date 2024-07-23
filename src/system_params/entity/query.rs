use bevy::ecs::query::QueryEntityError;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use thiserror::Error;

use crate::assets::entity::LdtkEntityAsset;
use crate::assets::layer::LdtkLayerAsset;
use crate::prelude::LdtkQuery;
use crate::prelude::LdtkQueryEx;
use crate::system_params::entity::item::LdtkEntity;
use crate::system_params::layer::item::LdtkLayer;
use crate::system_params::layer::query::LdtkLayerQuery;
use crate::system_params::traits::LdtkItem;
use crate::system_params::traits::LdtkIterable;

#[derive(Debug, Error)]
pub enum LdtkEntityQueryError {
    #[error(transparent)]
    QueryEntityError(#[from] QueryEntityError),
    #[error("bad layer handle? {0:?}")]
    BadLayerHandle(Handle<LdtkLayerAsset>),
    #[error("could not query parent as a layer")]
    NoLayerParent,
}

#[derive(SystemParam)]
pub struct LdtkEntityQuery<'w, 's> {
    entity_assets: Res<'w, Assets<LdtkEntityAsset>>,
    entity_query: Query<'w, 's, (Entity, &'static Handle<LdtkEntityAsset>)>,
    entity_query_added:
        Query<'w, 's, (Entity, &'static Handle<LdtkEntityAsset>), Added<Handle<LdtkEntityAsset>>>,
    layer_query: LdtkLayerQuery<'w, 's>,
    parent_query: Query<'w, 's, &'static Parent>,
    transform_query: Query<'w, 's, &'static Transform, With<Handle<LdtkEntityAsset>>>,
}

impl<'w, 's> LdtkIterable<'w, 's, LdtkEntityAsset> for LdtkEntityQuery<'w, 's> {
    fn query(&self) -> impl Iterator<Item = (Entity, &Handle<LdtkEntityAsset>)> {
        self.entity_query.iter()
    }

    fn query_added(&self) -> impl Iterator<Item = (Entity, &Handle<LdtkEntityAsset>)> {
        self.entity_query_added.iter()
    }

    fn get_asset(&self, id: AssetId<LdtkEntityAsset>) -> Option<&LdtkEntityAsset> {
        self.entity_assets.get(id)
    }
}

impl<'w, 's> LdtkEntityQuery<'w, 's> {
    pub fn get_layer(
        &'w self,
        ldtk_entity: &LdtkEntity<'w, 's>,
    ) -> Result<LdtkLayer, LdtkEntityQueryError> {
        let entity = ldtk_entity.ecs_entity();
        let layer_entity = self.parent_query.get(entity)?.get();
        let ldtk_layer: LdtkLayer = self
            .layer_query
            .iter()
            .find_entity(layer_entity)
            .ok_or(LdtkEntityQueryError::NoLayerParent)?;
        Ok(ldtk_layer)
    }

    pub fn grid(&'w self, ldtk_entity: &LdtkEntity<'w, 's>) -> IVec2 {
        let entity = ldtk_entity.ecs_entity();
        let asset = ldtk_entity.asset();
        let translation = self
            .transform_query
            .get(entity)
            .expect("an entity with Handle<LdtkEntity> component")
            .translation
            .truncate();
        let ldtk_layer: LdtkLayer<'_, '_> = self.get_layer(ldtk_entity).expect("a layer asset");

        let anchor_vec = asset.anchor.as_vec();
        let focus = Vec2::new(1.0, -1.0) * (translation - anchor_vec);
        let focus = focus.as_ivec2();
        let grid_size = ldtk_layer.asset().grid_size as i32;

        focus / grid_size
    }
}
