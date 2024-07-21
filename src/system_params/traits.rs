use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use std::fmt::Debug;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::Iid;

pub trait LdtkItem<'w, A>
where
    A: LdtkAsset,
    Self: Sized,
{
    fn new(entity: Entity, asset: &'w A) -> Self;
    fn ecs_entity(&self) -> Entity;
    fn asset(&self) -> &A;
}

#[derive(Debug, Error)]
pub enum LdtkQueryError {
    #[error("Itentifier yielded no values: {0}")]
    NoValues(String),
    #[error("Identifier yielded more than one value: {0}")]
    MoreThanOneValue(String),
}

pub trait LdtkIterable<'w, 's, A>
where
    A: LdtkAsset,
    Self: SystemParam,
{
    fn query(&self) -> impl Iterator<Item = (Entity, &Handle<A>)>;
    fn query_added(&self) -> impl Iterator<Item = (Entity, &Handle<A>)>;
    fn get_asset(&self, id: AssetId<A>) -> Option<&A>;
}

pub trait LdtkQuery<'w, 's, I, A>
where
    I: LdtkItem<'w, A>,
    A: LdtkAsset,
    Self: LdtkIterable<'w, 's, A>,
{
    fn iter(&'w self) -> impl Iterator<Item = I> {
        self.query()
            .filter_map(move |(entity, handle)| Some(I::new(entity, self.get_asset(handle.id())?)))
    }

    fn get_single_with_identifier(&'w self, identifier: &str) -> Result<I, LdtkQueryError> {
        let mut iter = self.iter().filter_identifier(identifier);
        let first = iter.next();
        let rest = iter.next();
        match (first, rest) {
            (None, None) => Err(LdtkQueryError::NoValues(identifier.to_string())),
            (None, Some(_)) => unreachable!(),
            (Some(inner), None) => Ok(inner),
            (Some(_), Some(_)) => Err(LdtkQueryError::MoreThanOneValue(identifier.to_string())),
        }
    }

    fn single_with_identifier(&'w self, identifier: &str) -> I {
        self.get_single_with_identifier(identifier).unwrap()
    }
}

impl<'w, 's, I, A, Q> LdtkQuery<'w, 's, I, A> for Q
where
    I: LdtkItem<'w, A>,
    A: LdtkAsset,
    Q: LdtkIterable<'w, 's, A>,
{
}

pub trait LdtkQueryEx<'w, 's, I, A>
where
    I: LdtkItem<'w, A>,
    A: LdtkAsset,
    Self: Iterator<Item = I> + Sized,
{
    fn filter_identifier(self, identifier: &str) -> impl Iterator<Item = I> {
        self.filter(move |ldtk_entity| ldtk_entity.asset().identifier() == identifier)
    }

    fn find_entity(mut self, entity: Entity) -> Option<I> {
        self.find(|item| item.ecs_entity() == entity)
    }

    fn find_iid(mut self, iid: Iid) -> Option<I> {
        self.find(|item| item.asset().iid() == iid)
    }
}

impl<'w, 's, I, A, It> LdtkQueryEx<'w, 's, I, A> for It
where
    I: LdtkItem<'w, A>,
    A: LdtkAsset,
    It: Iterator<Item = I>,
{
}
