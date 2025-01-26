use bevy_core::Name;
use bevy_ecs::query::QueryData;
use bevy_ecs::system::{Commands, SystemParam};
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_reflect::Reflect;
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::commands::entity::EntityCommands;
use crate::commands::layer::LayerCommands;
use crate::commands::level::LevelCommands;
use crate::commands::project::ProjectCommands;
use crate::commands::world::WorldCommands;
use crate::component::ShieldtankComponentFinalized;
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
    pub fn insert_name_component(&mut self, name: &str) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(Name::new(name.to_string()));

        self
    }

    pub fn insert_transform_component(&mut self, transform: Transform) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(transform);

        self
    }

    pub fn insert_visibility_component(&mut self, visibility: Visibility) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(visibility);

        self
    }
}

impl<A: LdtkAsset, D: QueryData> ShieldtankItemCommands<'_, '_, A, D> {
    pub(crate) fn mark_finalized(&mut self, just_finalized: bool) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(ShieldtankComponentFinalized { just_finalized });

        self
    }

    pub(crate) fn _unmark_finalized(&mut self) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .remove::<ShieldtankComponentFinalized>();

        self
    }
}
