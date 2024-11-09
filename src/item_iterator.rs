use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;

use crate::item::LdtkItemTrait;

pub trait LdtkItemIterator<Asset>
where
    Self: Iterator + Sized,
    Self::Item: LdtkItemTrait<Asset>,
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
    fn find_iid(mut self, iid: Iid) -> Option<impl LdtkItemTrait<Asset>> {
        self.find(|item| item.get_iid() == iid)
    }
}

impl<Asset, Iter> LdtkItemIterator<Asset> for Iter
where
    Iter: Iterator + Sized,
    Iter::Item: LdtkItemTrait<Asset>,
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
}
