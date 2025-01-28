use bevy_ecs::query::QueryData;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_ldtk_asset::world::World as WorldAsset;

use crate::component::level::LevelComponentQueryData;
use crate::component::project::ProjectComponentQueryData;
use crate::component::world::WorldComponentQueryData;
use crate::item::level::LevelItem;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;

pub trait UniqueIdentifierExt<'w, 's, A, D>
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

impl<'w, 's: 'w, I: Iterator<Item = ProjectItem<'w, 's>>>
    UniqueIdentifierExt<'w, 's, ProjectAsset, ProjectComponentQueryData<'w>> for I
{
}

impl<'w, 's: 'w, I: Iterator<Item = WorldItem<'w, 's>>>
    UniqueIdentifierExt<'w, 's, WorldAsset, WorldComponentQueryData<'w>> for I
{
}

impl<'w, 's: 'w, I: Iterator<Item = LevelItem<'w, 's>>>
    UniqueIdentifierExt<'w, 's, LevelAsset, LevelComponentQueryData<'w>> for I
{
}
