use bevy_app::{Plugin, PostUpdate};

use super::project::ProjectComponent;
use super::systems::{
    find_and_mark_loaded_components, find_and_unmark_just_loaded_components, insert_name_component,
};

pub struct ComponentPlugin;
impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ProjectComponent>().add_systems(
            PostUpdate,
            (
                find_and_mark_loaded_components,
                find_and_unmark_just_loaded_components,
                insert_name_component,
            ),
        );
    }
}
