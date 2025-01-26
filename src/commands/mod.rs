use bevy_ecs::bundle::Bundle;
use bevy_ecs::query::QueryData;
use bevy_ecs::system::{Commands, SystemParam};
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_reflect::Reflect;

use crate::commands::entity::EntityCommands;
use crate::commands::layer::LayerCommands;
use crate::commands::level::LevelCommands;
use crate::commands::project::ProjectCommands;
use crate::commands::world::WorldCommands;
use crate::item::entity::EntityItem;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;

pub mod entity;
pub mod layer;
pub mod level;
pub mod project;
pub mod world;

#[derive(SystemParam)]
pub struct ShieldtankCommands<'w, 's> {
    commands: Commands<'w, 's>,
}

impl ShieldtankCommands<'_, '_> {}

macro_rules! make_getter {
    ($name:tt, $item_type:tt, $commands_type:tt) => {
        pub fn $name<'w, 's>(&'w mut self, item: &'w $item_type<'w, 's>) -> $commands_type<'w, 's>
        where
            'w: 's,
        {
            $commands_type {
                commands: self.commands.reborrow(),
                item,
            }
        }
    };
}

impl ShieldtankCommands<'_, '_> {
    make_getter!(entity, EntityItem, EntityCommands);
    make_getter!(layer, LayerItem, LayerCommands);
    make_getter!(level, LevelItem, LevelCommands);
    make_getter!(world, WorldItem, WorldCommands);
    make_getter!(project, ProjectItem, ProjectCommands);
}

#[derive(Reflect)]
pub struct ShieldtankItemCommands<'w, 's, A: LdtkAsset, D: QueryData> {
    pub(crate) commands: Commands<'w, 's>,
    pub(crate) item: &'w Item<'w, 's, A, D>,
}

impl<A: LdtkAsset, D: QueryData> ShieldtankItemCommands<'_, '_, A, D> {}

impl<A: LdtkAsset, D: QueryData> ShieldtankItemCommands<'_, '_, A, D> {
    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(bundle);

        self
    }

    pub fn remove<T: Bundle>(&mut self) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .remove::<T>();

        self
    }
}
