use bevy_app::{Plugin, PostUpdate};

use crate::item::level::systems::level_spawn_system;

pub struct LevelItemPlugin;
impl Plugin for LevelItemPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(PostUpdate, level_spawn_system);
    }
}
