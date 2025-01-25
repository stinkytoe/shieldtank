use bevy_app::Plugin;
use bevy_ldtk_asset::layer_definition::IntGridValue;
use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_math::Vec2;

use crate::component::LdtkComponent;
use crate::entity::EntityItem;
use crate::impl_unique_identifer_iterator;
use crate::item::LdtkItem;
use crate::item::LdtkItemTrait;
use crate::layer::LayerItem;
use crate::level::LevelItem;
use crate::project::ProjectItem;

pub type WorldComponent = LdtkComponent<WorldAsset>;
pub type WorldItem<'a> = LdtkItem<'a, WorldAsset>;
impl_unique_identifer_iterator!(WorldAsset);

impl WorldItem<'_> {
    /// Returns the [ProjectItem] represent the parent project.
    ///
    /// This represents the Bevy ECS scructure, which may be different than the `.ldtk` project,
    /// if the ECS entity was either moved, removed, or filtered out during loading.
    pub fn get_project(&self) -> Option<ProjectItem> {
        self.get_query()
            .get_project(
                self.get_query()
                    .parent_query
                    .get(self.get_ecs_entity())
                    .ok()?
                    .get(),
            )
            .ok()
    }

    /// Returns an iterator over the levels which belong to this world, if any.
    ///
    /// This represents the Bevy ECS scructure, which may be different than the `.ldtk` project,
    /// if the ECS entity was either moved, removed, or filtered out during loading.
    pub fn iter_levels(&self) -> impl Iterator<Item = LevelItem> {
        self.get_query().iter_levels().filter(|item| {
            item.get_world()
                .filter(|item| item.get_ecs_entity() == self.get_ecs_entity())
                .is_some()
        })
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = LayerItem> {
        self.get_query().iter_layers().filter(|item| {
            item.get_world()
                .filter(|item| item.get_ecs_entity() == self.get_ecs_entity())
                .is_some()
        })
    }

    pub fn iter_entities(&self) -> impl Iterator<Item = EntityItem> {
        self.get_query().iter_entities().filter(|item| {
            item.get_world()
                .filter(|item| item.get_ecs_entity() == self.get_ecs_entity())
                .is_some()
        })
    }

    pub fn levels_at(&self, location: Vec2) -> impl Iterator<Item = LevelItem> {
        self.iter_levels()
            .filter(move |item| item.contains_world_location(location))
    }

    pub fn layers_at(&self, location: Vec2) -> impl Iterator<Item = LayerItem> {
        self.iter_layers()
            .filter(move |item| item.contains_world_location(location))
    }

    pub fn entities_at(&self, location: Vec2) -> impl Iterator<Item = EntityItem> {
        self.iter_entities()
            .filter(move |item| item.contains_world_location(location))
    }

    pub fn location(&self) -> Vec2 {
        self.get_transform().translation.truncate()
    }

    pub fn location_in_project(&self) -> Vec2 {
        let location = self.location();
        let project_location = self.get_project().expect("no project!").location();

        location - project_location
    }

    pub fn location_from_project_location(&self, location: Vec2) -> Vec2 {
        let location = self.location();
        let project_location = self.get_project().expect("no project!").location();

        project_location + location
    }

    pub fn int_grid_values_at(&self, location: Vec2) -> impl Iterator<Item = IntGridValue> {
        todo!();
        vec![].into_iter()
    }

    pub fn int_grid_value_at(&self, location: Vec2) -> Option<IntGridValue> {
        todo!()
    }
}
// impl WorldItem<'_> {
//     pub fn get_levels(&self) -> impl Iterator<Item = LevelItem> {
//         self.query
//             .levels()
//             .filter_map(|item| {
//                 let ecs_entity = item.get_ecs_entity();
//                 Some((item, self.query.parent_query.get(ecs_entity).ok()?))
//             })
//             .filter(|(_, parent)| parent.get() == self.get_ecs_entity())
//             .map(|(item, _)| item)
//     }
// }
//
// impl WorldItem<'_> {
//     pub fn get_project(&self) -> Option<ProjectItem> {
//         let project_ecs_entity = self
//             .query
//             .parent_query
//             .get(self.get_ecs_entity())
//             .ok()
//             .map(|parent| parent.get())?;
//
//         self.query.get_project(project_ecs_entity).ok()
//     }
// }
//
// impl WorldItem<'_> {
//     pub fn int_grid_value_at_world_location(&self, world_location: Vec2) -> Option<IntGridValue> {
//         // let self_location = self.get_transform()?.translation.truncate();
//         // let global_location = self_location + world_location;
//         //
//         // let mut levels: Vec<_> = self
//         //     .get_levels()
//         //     .filter_global_location(global_location)
//         //     .collect();
//         //
//         // #[allow(clippy::unwrap_used)]
//         // levels.sort_by(|a, b| {
//         //     // unwrap is OK here because the above collect wouldn't have yielded anything that didn't
//         //     // have a global_transform component.
//         //     let a_z = a.get_global_transform().unwrap().translation().z;
//         //     let b_z = b.get_global_transform().unwrap().translation().z;
//         //     // intentionally reversed, so we will search nearest to farthest when looking down in
//         //     // the world from above.
//         //     b_z.partial_cmp(&a_z).unwrap()
//         // });
//         //
//         // levels
//         //     .iter()
//         //     .find_map(|level_item| level_item.int_grid_value_at_global_location(global_location))
//         todo!()
//     }
// }

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut bevy_app::App) {}
}
