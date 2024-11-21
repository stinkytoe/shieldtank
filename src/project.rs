use bevy_ldtk_asset::project::Project as ProjectAsset;

use crate::component::LdtkComponent;
use crate::impl_unique_identifer_iterator;
use crate::item::{LdtkItem, LdtkItemTrait};
use crate::world::WorldItem;

pub type ProjectComponent = LdtkComponent<ProjectAsset>;
pub type ProjectItem<'a> = LdtkItem<'a, ProjectAsset>;

impl_unique_identifer_iterator!(ProjectAsset);

impl ProjectItem<'_> {
    pub fn worlds(&self) -> impl Iterator<Item = WorldItem> {
        self.query
            .worlds()
            .filter_map(|item| {
                let ecs_entity = item.get_ecs_entity();
                Some((item, self.query.parent_query.get(ecs_entity).ok()?))
            })
            .filter(|(_, parent)| parent.get() == self.get_ecs_entity())
            .map(|(item, _)| item)
    }
}
