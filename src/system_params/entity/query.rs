use bevy::ecs::query::QueryEntityError;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use thiserror::Error;

use crate::assets::entity::LdtkEntityAsset;
use crate::assets::layer::LdtkLayerAsset;
use crate::system_params::layer::query::LdtkLayerQuery;
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
    pub(crate) entity_assets: Res<'w, Assets<LdtkEntityAsset>>,
    pub(crate) entity_query: Query<'w, 's, (Entity, &'static Handle<LdtkEntityAsset>)>,
    pub(crate) entity_query_added:
        Query<'w, 's, (Entity, &'static Handle<LdtkEntityAsset>), Added<Handle<LdtkEntityAsset>>>,
    pub(crate) layer_query: LdtkLayerQuery<'w, 's>,
    pub(crate) parent_query: Query<'w, 's, &'static Parent>,
    pub(crate) transform_query: Query<'w, 's, &'static Transform, With<Handle<LdtkEntityAsset>>>,
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

impl<'w, 's> LdtkEntityQuery<'w, 's> {}
