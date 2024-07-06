use bevy::prelude::*;

use crate::assets::project::LdtkProject;

pub struct ShieldTankPlugin;

impl Plugin for ShieldTankPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LdtkProject>();
    }
}
