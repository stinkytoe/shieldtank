use bevy_app::{Plugin, PostUpdate};

use crate::item::layer::systems::{layer_override_transform_system, layer_spawn_system};

pub struct LayerItemPlugin;
impl Plugin for LayerItemPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            PostUpdate,
            (layer_spawn_system, layer_override_transform_system),
        );
    }
}
