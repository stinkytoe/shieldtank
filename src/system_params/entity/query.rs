use bevy::ecs::query::QueryEntityError;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use thiserror::Error;

use crate::assets::entity::LdtkEntityAsset;
use crate::assets::layer::LdtkLayerAsset;
use crate::iid::Iid;
use crate::system_params::entity::item::LdtkEntity;

#[derive(Debug, Error)]
pub enum LdtkEntityQueryError {
    #[error(transparent)]
    QueryEntityError(#[from] QueryEntityError),
    #[error("bad layer handle? {0:?}")]
    BadLayerHandle(Handle<LdtkLayerAsset>),
    #[error("Itentifier yielded no values: {0}")]
    NoValues(String),
    #[error("Identifier yielded more than one value: {0}")]
    MoreThanOneValue(String),
}

#[derive(SystemParam)]
pub struct LdtkEntityQuery<'w, 's> {
    entity_assets: Res<'w, Assets<LdtkEntityAsset>>,
    entity_query: Query<'w, 's, (Entity, &'static Handle<LdtkEntityAsset>)>,
    layer_assets: Res<'w, Assets<LdtkLayerAsset>>,
    layer_query: Query<'w, 's, &'static Handle<LdtkLayerAsset>>,
    parent_query: Query<'w, 's, &'static Parent>,
    transform_query: Query<'w, 's, &'static Transform, With<Handle<LdtkEntityAsset>>>,
}

impl<'w, 's> LdtkEntityQuery<'w, 's> {
    pub fn get(&self, iid: Iid) -> Option<LdtkEntity> {
        self.iter().find(|ldtk_entity| ldtk_entity.asset.iid == iid)
    }

    pub fn iter(&self) -> impl Iterator<Item = LdtkEntity<'_>> {
        self.entity_query.iter().filter_map(|(entity, handle)| {
            // FIXME: Is this safe/sane to .filter_map(..) here?
            Some((entity, self.entity_assets.get(handle.id())?).into())
        })
    }

    pub fn get_single_with_identifier(
        &self,
        identifier: &str,
    ) -> Result<LdtkEntity<'_>, LdtkEntityQueryError> {
        let mut iter = self.iter().with_identifier(identifier);
        let first = iter.next();
        let rest = iter.next();
        match (first, rest) {
            (None, None) => Err(LdtkEntityQueryError::NoValues(identifier.to_string())),
            (None, Some(_)) => unreachable!(),
            (Some(inner), None) => Ok(inner),
            (Some(_), Some(_)) => Err(LdtkEntityQueryError::MoreThanOneValue(
                identifier.to_string(),
            )),
        }
    }

    pub fn single_with_identifier(&self, identifier: &str) -> LdtkEntity<'_> {
        self.get_single_with_identifier(identifier).unwrap()
    }

    pub fn get_layer(
        &self,
        ldtk_entity: &LdtkEntity<'_>,
    ) -> Result<&LdtkLayerAsset, LdtkEntityQueryError> {
        let entity = ldtk_entity.ecs_entity();
        let layer_entity = self.parent_query.get(entity)?.get();
        let layer_handle = self.layer_query.get(layer_entity)?;
        let layer_asset = self
            .layer_assets
            .get(layer_handle.id())
            .ok_or(LdtkEntityQueryError::BadLayerHandle(layer_handle.clone()))?;
        Ok(layer_asset)
    }

    pub fn grid(&self, ldtk_entity: &LdtkEntity<'_>) -> IVec2 {
        let entity = ldtk_entity.ecs_entity();
        let asset = ldtk_entity.asset();
        let translation = self
            .transform_query
            .get(entity)
            .expect("an entity with Handle<LdtkEntity> component")
            .translation
            .truncate();
        let layer_asset = self.get_layer(ldtk_entity).expect("a layer asset");

        let anchor_vec = asset.anchor.as_vec();
        let focus = Vec2::new(1.0, -1.0) * (translation - anchor_vec);
        let focus = focus.as_ivec2();
        let grid_size = layer_asset.grid_size as i32;

        focus / grid_size
    }
}

pub trait LdtkEntityQueryEx<'w>
where
    Self: Iterator<Item = LdtkEntity<'w>> + Sized,
{
    fn with_identifier(self, identifier: &str) -> impl Iterator<Item = LdtkEntity<'w>> {
        self.filter(move |ldtk_entity| ldtk_entity.asset.identifier == identifier)
    }

    fn with_tag(self, tag: &str) -> impl Iterator<Item = LdtkEntity<'w>> {
        self.filter(move |ldtk_entity| ldtk_entity.has_tag(tag))
    }
}

impl<'w, I> LdtkEntityQueryEx<'w> for I where I: Iterator<Item = LdtkEntity<'w>> {}
