use bevy_ecs::query::QueryData;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;

use super::Item;

pub trait ItemIteratorUniqueIdentifierExt<'w, 's, A, D>
where
    's: 'w,
    Self: Iterator<Item = Item<'w, 's, A, D>> + Sized,
    A: LdtkAsset + 'w,
    D: QueryData + 'w,
{
    fn find_identifier(mut self, identifier: &str) -> Option<Item<'w, 's, A, D>> {
        self.find(|item| item.get_identifier() == identifier)
    }
}

use crate::component::project::ProjectComponentQueryData;
use crate::item::project::ProjectItem;
use bevy_ldtk_asset::project::Project as ProjectAsset;
impl<'w, 's: 'w, I: Iterator<Item = ProjectItem<'w, 's>>>
    ItemIteratorUniqueIdentifierExt<'w, 's, ProjectAsset, ProjectComponentQueryData<'w>> for I
{
}

use crate::component::world::WorldComponentQueryData;
use crate::item::world::WorldItem;
use bevy_ldtk_asset::world::World as WorldAsset;
impl<'w, 's: 'w, I: Iterator<Item = WorldItem<'w, 's>>>
    ItemIteratorUniqueIdentifierExt<'w, 's, WorldAsset, WorldComponentQueryData<'w>> for I
{
}

use crate::component::level::LevelComponentQueryData;
use crate::item::level::LevelItem;
use bevy_ldtk_asset::level::Level as LevelAsset;
impl<'w, 's: 'w, I: Iterator<Item = LevelItem<'w, 's>>>
    ItemIteratorUniqueIdentifierExt<'w, 's, LevelAsset, LevelComponentQueryData<'w>> for I
{
}
