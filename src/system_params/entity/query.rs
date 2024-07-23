use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::assets::entity::LdtkEntityAsset;
use crate::system_params::entity::item::LdtkEntity;
use crate::system_params::layer::query::LdtkLayerQuery;
use crate::system_params::traits::LdtkIterable;

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

pub trait LdtkEntityQueryEx<'a>
where
    Self: Iterator<Item = LdtkEntity<'a, 'a>> + Sized,
{
    fn filter_tag(self, tag: &str) -> impl Iterator<Item = LdtkEntity<'a, 'a>> {
        self.filter(move |ldtk_entity| ldtk_entity.has_tag(tag))
    }
}

impl<'a, It> LdtkEntityQueryEx<'a> for It where It: Iterator<Item = LdtkEntity<'a, 'a>> {}
