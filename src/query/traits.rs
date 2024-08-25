use bevy::ecs::query::{QueryFilter, QueryIter};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::Iid;

pub struct LdtkItem<'a, A>
where
    A: LdtkAsset,
{
    pub entity: Entity,
    pub asset: &'a A,
    pub query: &'a LdtkAssetQuery<'a, 'a, A>,
}

impl<A> LdtkItem<'_, A>
where
    A: LdtkAsset,
{
    pub fn ecs_entity(&self) -> Entity {
        self.entity
    }

    pub fn asset(&self) -> &A {
        self.asset
    }
}

#[derive(SystemParam)]
pub struct LdtkAssetQuery<'w, 's, A>
where
    A: LdtkAsset,
{
    query_all: Query<'w, 's, (Entity, &'static Handle<A>)>,
    query_added: Query<'w, 's, (Entity, &'static Handle<A>), Added<Handle<A>>>,
    assets: Res<'w, Assets<A>>,
}

impl<A> LdtkAssetQuery<'_, '_, A>
where
    A: LdtkAsset,
{
    pub fn iter(&self) -> LdtkItemIterator<'_, '_, A, ()> {
        LdtkItemIterator {
            assets: &self.assets,
            iter: self.query_all.iter(),
            query: self,
        }
    }

    pub fn iter_added(&self) -> LdtkItemIterator<'_, '_, A, Added<Handle<A>>> {
        LdtkItemIterator {
            assets: &self.assets,
            iter: self.query_added.iter(),
            query: self,
        }
    }
}

pub struct LdtkItemIterator<'w, 's, A, F>
where
    A: LdtkAsset,
    F: QueryFilter,
    Self: Iterator<Item = LdtkItem<'w, A>>,
{
    assets: &'w Assets<A>,
    iter: QueryIter<'w, 's, (Entity, &'static Handle<A>), F>,
    query: &'w LdtkAssetQuery<'w, 's, A>,
}

impl<'w, 's, A, F> Iterator for LdtkItemIterator<'w, 's, A, F>
where
    A: LdtkAsset,
    F: QueryFilter,
{
    type Item = LdtkItem<'w, A>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .and_then(|(entity, handle)| Some((entity, self.assets.get(handle.id())?)))
            .map(|(entity, asset)| LdtkItem {
                entity,
                asset,
                query: self.query,
            })
    }
}

#[derive(Debug, Error)]
pub enum LdtkItemIteratorWithIdentifierError {
    #[error("Attempt to yield exactly one item with none present!")]
    None,
    #[error("Attempt to yield exactly one item with more than one present!")]
    MoreThanOne,
}

pub trait LdtkItemIteratorExt<'w, 's, A>
where
    A: LdtkAsset,
    Self: Iterator<Item = LdtkItem<'w, A>> + Sized,
{
    fn with_iid(&'w mut self, iid: Iid) -> Option<LdtkItem<'w, A>> {
        self.find(|item| item.asset.iid() == iid)
    }
}

impl<'w, 's, A, I> LdtkItemIteratorExt<'w, 's, A> for I
where
    A: LdtkAsset,
    I: Iterator<Item = LdtkItem<'w, A>>,
{
}

pub trait LdtkItemIteratorUniqueIdentifierExt<'w, 's, A, F>
where
    A: LdtkAsset,
    F: QueryFilter,
    Self: Iterator<Item = LdtkItem<'w, A>> + Sized,
{
    fn with_unique_identifier(&'w mut self, identifier: &str) -> Option<LdtkItem<'w, A>> {
        self.find(|item| item.asset.identifier() == identifier)
    }
}

pub struct LdtkItemIteratorWithIdentifier<'w, 's, A, F>
where
    A: LdtkAsset,
    F: QueryFilter,
    Self: Iterator<Item = LdtkItem<'w, A>>,
    LdtkItemIterator<'w, 's, A, F>: Iterator<Item = LdtkItem<'w, A>>,
{
    pub(crate) identifier: &'w str,
    pub(crate) iter: LdtkItemIterator<'w, 's, A, F>,
}

impl<'w, 's, A, F> Iterator for LdtkItemIteratorWithIdentifier<'w, 's, A, F>
where
    A: LdtkAsset,
    F: QueryFilter,
    LdtkItemIterator<'w, 's, A, F>: Iterator<Item = LdtkItem<'w, A>>,
{
    type Item = LdtkItem<'w, A>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find(|item| item.asset.identifier() == self.identifier)
    }
}
