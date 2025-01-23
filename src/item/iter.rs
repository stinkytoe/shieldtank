use bevy_ecs::query::QueryData;
use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;

use super::Item;

pub trait ItemIteratorExt<'w, 's, A, D>
where
    's: 'w,
    Self: Iterator<Item = Item<'w, 's, A, D>> + Sized,
    A: LdtkAsset + 'w,
    D: QueryData + 'w,
{
    fn find_iid(mut self, iid: Iid) -> Option<Item<'w, 's, A, D>> {
        self.find(|item| item.get_iid() == iid)
    }

    fn filter_finalized(self) -> impl Iterator<Item = Item<'w, 's, A, D>> {
        self.filter(|item| item.is_finalized())
    }

    fn filter_just_finalized(self) -> impl Iterator<Item = Item<'w, 's, A, D>> {
        self.filter(|item| item.is_just_finalized())
    }
}

// pub trait ShieldtankItemIterator<'a, A, D>
// where
//     Self: Iterator<'w, 's, A: LdtkAsset + 'w, D: QueryData + 'w> + Sized,
//     A: LdtkAsset + Sized + std::fmt::Debug,
// {
//     // fn find_iid(mut self, iid: Iid) -> Option<Item<'a, A>> {
//     //     todo!()
//     //     // self.find(|item| item.get_iid() == iid)
//     // }
//
//     // fn find_ecs_entity(mut self, ecs_entity: EcsEntity) -> Option<LdtkItem<'a, A>> {
//     //     self.find(|item| item.get_ecs_entity() == ecs_entity)
//     // }
//     //
//     // fn filter_added(self) -> impl Iterator<Item = LdtkItem<'a, A>> {
//     //     self.filter(|item| item.component.is_added())
//     // }
//     //
//     // fn filter_changed(self) -> impl Iterator<Item = LdtkItem<'a, A>> {
//     //     self.filter(|item| item.component.is_changed())
//     // }
//     //
//     // fn filter_changed_not_added(self) -> impl Iterator<Item = LdtkItem<'a, A>> {
//     //     self.filter(|item| !item.component.is_added())
//     //         .filter_changed()
//     // }
// }

impl<'w, 's, A, D, I> ItemIteratorExt<'w, 's, A, D> for I
where
    's: 'w,
    I: Iterator<Item = Item<'w, 's, A, D>>,
    A: LdtkAsset + 'w,
    D: QueryData + 'w,
{
}
// impl<'a, Asset, Iter> LdtkItemIterator<'a, Asset> for Iter
// where
//     Iter: Iterator<Item = LdtkItem<'a, Asset>> + Sized,
//     Asset: LdtkAsset + Sized + std::fmt::Debug,
// {
// }
