use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_math::Vec2;

use crate::component::LdtkComponent;
use crate::entity::EntityItem;
use crate::impl_unique_identifer_iterator;
use crate::item::{LdtkItem, LdtkItemTrait};
use crate::layer::LayerItem;
use crate::level::LevelItem;
use crate::world::WorldItem;

pub type ProjectComponent = LdtkComponent<ProjectAsset>;
pub type ProjectItem<'a> = LdtkItem<'a, ProjectAsset>;
impl_unique_identifer_iterator!(ProjectAsset);

impl ProjectItem<'_> {
    /// Returns an iterator over all worlds belonging to the project.
    ///
    /// This is representative of worlds which are loaded and exist as a child of this
    /// project in the ECS. This may differ from the `.ldtk` project file, if any worlds were
    /// either moved, removed, or filtered out during loading.
    pub fn iter_worlds(&self) -> impl Iterator<Item = WorldItem> {
        self.get_query().iter_worlds().filter(|item| {
            item.get_project()
                .filter(|parent_item| parent_item.get_ecs_entity() == self.get_ecs_entity())
                .is_some()
        })
    }

    pub fn iter_levels(&self) -> impl Iterator<Item = LevelItem> {
        todo!();
        vec![].into_iter()
    }

    pub fn iter_layers(&self) -> impl Iterator<Item = LayerItem> {
        todo!();
        vec![].into_iter()
    }

    pub fn iter_entities(&self) -> impl Iterator<Item = EntityItem> {
        todo!();
        vec![].into_iter()
    }

    /// Returns the location of the project, relative to the Bevy global space.
    ///
    /// This is normally `(0,0)`, but could be changed by the user by modifying the
    /// [Transform](https://docs.rs/bevy/latest/bevy/prelude/struct.Transform.html)
    /// component.
    pub fn location(&self) -> Vec2 {
        self.get_transform().translation.truncate()
    }

    pub fn levels_at(&self, location: Vec2) -> impl Iterator<Item = LevelItem> {
        self.get_query()
            .iter_levels()
            .filter(move |item| item.contains_project_location(location))
    }

    pub fn layers_at(&self, location: Vec2) -> impl Iterator<Item = LayerItem> {
        self.get_query()
            .iter_layers()
            .filter(move |item| item.contains_project_location(location))
    }

    pub fn entities_at(&self, location: Vec2) -> impl Iterator<Item = EntityItem> {
        self.get_query()
            .iter_entities()
            .filter(move |item| item.contains_project_location(location))
    }
}
