use bevy_ldtk_asset::field_instance::FieldInstance;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAssetWithFieldInstances;

use crate::item::LdtkItem;
use crate::item::LdtkItemTrait;

pub trait LdtkItemFieldInstancesExt<Asset>: LdtkItemTrait<Asset>
where
    Asset: LdtkAsset + LdtkAssetWithFieldInstances + std::fmt::Debug,
{
    fn get_field_instance(&self, identifier: &str) -> Option<&FieldInstance> {
        self.get_asset().get_field_instance(identifier)
    }
}

impl<'a, Asset> LdtkItemFieldInstancesExt<Asset> for LdtkItem<'a, Asset> where
    Asset: LdtkAsset + LdtkAssetWithFieldInstances + std::fmt::Debug
{
}
