use bevy_ecs::query::QueryData;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::prelude::LdtkAsset;

use crate::component::entity::EntityComponentQueryData;
use crate::component::layer::LayerComponentQueryData;
use crate::item::Item;

pub struct RecurrentIdentifierIterator<'w, 's, A, D, I>
where
    's: 'w,
    A: LdtkAsset + 'w,
    D: QueryData + 'w,
    I: Iterator<Item = Item<'w, 's, A, D>> + Sized,
{
    iter: I,
    identifier: &'w str,
}

impl<'w, 's, A, D, I> Iterator for RecurrentIdentifierIterator<'w, 's, A, D, I>
where
    's: 'w,
    A: LdtkAsset + 'w,
    D: QueryData + 'w,
    I: Iterator<Item = Item<'w, 's, A, D>> + Sized,
{
    type Item = Item<'w, 's, A, D>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find(|item| item.get_identifier() == self.identifier)
    }
}

pub trait ItemRecurrentIdentifierIteratorExt<'w, 's, A, D>
where
    's: 'w,
    Self: Iterator<Item = Item<'w, 's, A, D>> + Sized,
    A: LdtkAsset + 'w,
    D: QueryData + 'w,
{
    fn filter_identifier(
        self,
        identifier: &'w str,
    ) -> RecurrentIdentifierIterator<'w, 's, A, D, Self> {
        RecurrentIdentifierIterator {
            iter: self,
            identifier,
        }
    }
}

impl<'w, 's, I> ItemRecurrentIdentifierIteratorExt<'w, 's, LayerAsset, LayerComponentQueryData<'w>>
    for I
where
    's: 'w,
    I: Iterator<Item = Item<'w, 's, LayerAsset, LayerComponentQueryData<'w>>> + Sized,
{
}

impl<'w, 's, I>
    ItemRecurrentIdentifierIteratorExt<'w, 's, EntityAsset, EntityComponentQueryData<'w>> for I
where
    's: 'w,
    I: Iterator<Item = Item<'w, 's, EntityAsset, EntityComponentQueryData<'w>>> + Sized,
{
}
