use bevy_app::Plugin;
use bevy_ldtk_asset::layer_definition::IntGridValue;
use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_math::Vec2;

use crate::component::LdtkComponent;
use crate::impl_unique_identifer_iterator;
use crate::item::LdtkItem;
use crate::item::LdtkItemTrait;
use crate::level::LevelItem;
use crate::level::LevelItemIteratorExt;
use crate::project::ProjectItem;

pub type WorldComponent = LdtkComponent<WorldAsset>;
pub type WorldItem<'a> = LdtkItem<'a, WorldAsset>;

impl_unique_identifer_iterator!(WorldAsset);

impl WorldItem<'_> {
    pub fn get_levels(&self) -> impl Iterator<Item = LevelItem> {
        self.query
            .levels()
            .filter_map(|item| {
                let ecs_entity = item.get_ecs_entity();
                Some((item, self.query.parent_query.get(ecs_entity).ok()?))
            })
            .filter(|(_, parent)| parent.get() == self.get_ecs_entity())
            .map(|(item, _)| item)
    }
}

impl WorldItem<'_> {
    pub fn get_project(&self) -> Option<ProjectItem> {
        let project_ecs_entity = self
            .query
            .parent_query
            .get(self.get_ecs_entity())
            .ok()
            .map(|parent| parent.get())?;

        self.query.get_project(project_ecs_entity).ok()
    }
}

impl WorldItem<'_> {
    pub fn int_grid_value_at_world_location(&self, world_location: Vec2) -> Option<IntGridValue> {
        let self_location = self.get_transform()?.translation.truncate();
        let global_location = self_location + world_location;

        let mut levels: Vec<_> = self
            .get_levels()
            .filter_global_location(global_location)
            .collect();

        #[allow(clippy::unwrap_used)]
        levels.sort_by(|a, b| {
            // unwrap is OK here because the above collect wouldn't have yielded anything that didn't
            // have a global_transform component.
            let a_z = a.get_global_transform().unwrap().translation().z;
            let b_z = b.get_global_transform().unwrap().translation().z;
            // intentionally reversed, so we will search nearest to farthest when looking down in
            // the world from above.
            b_z.partial_cmp(&a_z).unwrap()
        });

        levels
            .iter()
            .find_map(|level_item| level_item.int_grid_value_at_global_location(global_location))
    }

    // pub fn int_grid_value_at_global_location(&self, global_location: Vec2) -> Option<IntGridValue> {
    //     let mut levels: Vec<_> = self
    //         .get_levels()
    //         .filter_global_location(global_location)
    //         .collect();
    //
    //     #[allow(clippy::unwrap_used)]
    //     levels.sort_by(|a, b| {
    //         // unwrap is OK here because the above collect wouldn't have yielded anything that didn't
    //         // have a global_transform component.
    //         let a_z = a.get_global_transform().unwrap().translation().z;
    //         let b_z = b.get_global_transform().unwrap().translation().z;
    //         // intentionally reversed, so we will search nearest to farthest when looking down in
    //         // the world from above.
    //         b_z.partial_cmp(&a_z).unwrap()
    //     });
    //
    //     levels
    //         .iter()
    //         .find_map(|level_item| level_item.int_grid_value_at_global_location(global_location))
    // }
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut bevy_app::App) {}
}
