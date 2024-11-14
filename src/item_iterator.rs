use std::marker::PhantomData;

use bevy_ecs::{change_detection::DetectChanges, entity::Entity as EcsEntity};
use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;

use crate::item::{LdtkItem, LdtkItemTrait};

pub trait LdtkItemIterator<'a, Asset>
where
    Self: Iterator<Item = LdtkItem<'a, Asset>> + Sized,
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
    fn find_iid(mut self, iid: Iid) -> Option<LdtkItem<'a, Asset>> {
        self.find(|item| item.get_iid() == iid)
    }

    fn find_ecs_entity(mut self, ecs_entity: EcsEntity) -> Option<LdtkItem<'a, Asset>> {
        self.find(|item| item.get_ecs_entity() == ecs_entity)
    }

    fn filter_added(self) -> impl Iterator<Item = LdtkItem<'a, Asset>> {
        self.filter(|item| item.component.is_added())
    }

    fn filter_changed(self) -> impl Iterator<Item = LdtkItem<'a, Asset>> {
        self.filter(|item| item.component.is_changed())
    }

    fn filter_changed_not_added(self) -> impl Iterator<Item = LdtkItem<'a, Asset>> {
        self.filter(|item| !item.component.is_added())
            .filter_changed()
    }
}

impl<'a, Asset, Iter> LdtkItemIterator<'a, Asset> for Iter
where
    Iter: Iterator<Item = LdtkItem<'a, Asset>> + Sized,
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
}

pub trait LdtkItemUniqueIdentifierIterator<'a, Asset>
where
    Self: Iterator<Item = LdtkItem<'a, Asset>> + Sized,
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
    fn find_identifier(mut self, identifier: &str) -> Option<LdtkItem<'a, Asset>> {
        self.find(|item| item.get_identifier() == identifier)
    }
}

#[macro_export]
macro_rules! impl_unique_identifer_iterator {
    ($asset:tt) => {
        impl<'a, Iter> $crate::item_iterator::LdtkItemUniqueIdentifierIterator<'a, $asset> for Iter
        where
            Iter: Iterator<Item = LdtkItem<'a, $asset>> + Sized,
        {
        }
    };
}

pub trait LdtkItemRecurrentIdentifierIterator<'a, Asset>
where
    Self: Iterator<Item = LdtkItem<'a, Asset>> + Sized,
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
    fn filter_identifier(self, identifier: &'a str) -> LdtkItemFilterIdentifier<'a, Asset, Self> {
        LdtkItemFilterIdentifier {
            iter: self,
            identifier,
            _phantom: PhantomData,
        }
    }
}

pub struct LdtkItemFilterIdentifier<'a, Asset, Iter>
where
    Asset: LdtkAsset + Sized + std::fmt::Debug,
    Iter: Iterator + Sized,
    Iter::Item: LdtkItemTrait<Asset> + Sized + std::fmt::Debug,
{
    iter: Iter,
    identifier: &'a str,
    _phantom: PhantomData<Asset>,
}

impl<'a, Asset, Iter> std::fmt::Debug for LdtkItemFilterIdentifier<'a, Asset, Iter>
where
    Asset: LdtkAsset + Sized + std::fmt::Debug,
    Iter: Iterator + Sized,
    Iter::Item: LdtkItemTrait<Asset> + Sized + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LdtkItemFilterIdentifier")
            //.field("iter", &self.iter)
            .field("identifier", &self.identifier)
            //.field("_phantom", &self._phantom)
            .finish()
    }
}

impl<'a, Asset, Iter> Iterator for LdtkItemFilterIdentifier<'a, Asset, Iter>
where
    Asset: LdtkAsset + Sized + std::fmt::Debug,
    Iter: Iterator<Item = LdtkItem<'a, Asset>> + Sized,
{
    type Item = LdtkItem<'a, Asset>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find(|item| item.get_identifier() == self.identifier)
    }
}

#[macro_export]
macro_rules! impl_recurrent_identifer_iterator {
    ($asset:tt) => {
        impl<'a, Iter> $crate::item_iterator::LdtkItemRecurrentIdentifierIterator<'a, $asset>
            for Iter
        where
            Iter: Iterator<Item = LdtkItem<'a, $asset>> + Sized,
        {
        }
    };
}
