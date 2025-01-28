use bevy_app::{Plugin, PostUpdate};

use crate::item::entity::systems::entity_override_transform_system;
use crate::item::entity::systems::entity_spawn_system;

pub struct EntityItemPlugin;
impl Plugin for EntityItemPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            PostUpdate,
            (entity_spawn_system, entity_override_transform_system),
        );
    }
}
