use bevy_app::Plugin;
use bevy_ldtk_asset::world::World as WorldAsset;

use crate::{component::LdtkComponent, item::LdtkItem};

pub type World = LdtkComponent<WorldAsset>;
pub type WorldItem<'a> = LdtkItem<'a, WorldAsset>;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut bevy_app::App) {}
}
