use bevy_app::Plugin;
use bevy_ldtk_asset::world::World as WorldAsset;

use crate::{component::LdtkComponent, impl_unique_identifer_iterator, item::LdtkItem};

pub type World = LdtkComponent<WorldAsset>;
pub type WorldItem<'a> = LdtkItem<'a, WorldAsset>;

impl_unique_identifer_iterator!(WorldAsset);

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut bevy_app::App) {}
}
