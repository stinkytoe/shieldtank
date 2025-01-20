use bevy_ecs::query::QueryData;
use bevy_ecs::system::{Commands, SystemParam};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_reflect::Reflect;

use crate::component::entity::EntityComponentQueryData;
use crate::item::entity::EntityItem;
use crate::item::Item;

#[derive(SystemParam)]
pub struct ShieldtankCommands<'w, 's> {
    commands: Commands<'w, 's>,
}

impl ShieldtankCommands<'_, '_> {
    pub fn entity<'w, 's>(
        &'w mut self,
        entity_item: &'w EntityItem<'w, 's>,
    ) -> EntityCommands<'w, 's>
    where
        'w: 's,
    {
        EntityCommands {
            commands: self.commands.reborrow(),
            item: entity_item,
        }
    }
}

#[derive(Reflect)]
pub struct ShieldtankItemCommands<'w, 's, A: LdtkAsset, D: QueryData> {
    pub(crate) commands: Commands<'w, 's>,
    pub(crate) item: &'w Item<'w, 's, A, D>,
}

impl<A: LdtkAsset, D: QueryData> ShieldtankItemCommands<'_, '_, A, D> {
    pub fn test(&mut self, _a: i32) -> &mut Self {
        self
    }
}

pub type EntityCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, EntityAsset, EntityComponentQueryData<'w>>;
