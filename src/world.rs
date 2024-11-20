use bevy_app::Plugin;
use bevy_ldtk_asset::world::World as WorldAsset;

use crate::component::LdtkComponent;
use crate::impl_unique_identifer_iterator;
use crate::item::LdtkItem;
use crate::item::LdtkItemTrait;
use crate::level::LevelItem;

pub type WorldComponent = LdtkComponent<WorldAsset>;
pub type WorldItem<'a> = LdtkItem<'a, WorldAsset>;

impl_unique_identifer_iterator!(WorldAsset);

impl WorldItem<'_> {
    pub fn levels(&self) -> impl Iterator<Item = LevelItem> {
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

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut bevy_app::App) {}
}
