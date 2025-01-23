pub mod recurrent_identifier;
pub mod unique_identifier;

use bevy_ecs::entity::Entity;
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

    fn find_ecs_entity(mut self, ecs_entity: Entity) -> Option<Item<'w, 's, A, D>> {
        self.find(|item| item.get_ecs_entity() == ecs_entity)
    }

    fn filter_finalized(self) -> impl Iterator<Item = Item<'w, 's, A, D>> {
        self.filter(|item| item.is_finalized())
    }

    fn filter_just_finalized(self) -> impl Iterator<Item = Item<'w, 's, A, D>> {
        self.filter(|item| item.is_just_finalized())
    }
}

impl<'w, 's, A, D, I> ItemIteratorExt<'w, 's, A, D> for I
where
    's: 'w,
    I: Iterator<Item = Item<'w, 's, A, D>>,
    A: LdtkAsset + 'w,
    D: QueryData + 'w,
{
}
