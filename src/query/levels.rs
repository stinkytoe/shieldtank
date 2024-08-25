use bevy::ecs::query::QueryFilter;

use crate::assets::level::LdtkLevelAsset;
use crate::query::traits::LdtkItem;
use crate::query::traits::LdtkItemIteratorUniqueIdentifierExt;

use super::traits::LdtkAssetQuery;

pub type LdtkLevel<'w, 's> = LdtkItem<'w, LdtkLevelAsset>;

pub type LdtkLevelsQuery<'w, 's> = LdtkAssetQuery<'w, 's, LdtkLevelAsset>;

impl<'w, 's, F, I> LdtkItemIteratorUniqueIdentifierExt<'w, 's, LdtkLevelAsset, F> for I
where
    F: QueryFilter,
    Self: Iterator<Item = LdtkItem<'w, LdtkLevelAsset>> + Sized,
{
}
