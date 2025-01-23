use bevy_app::{Plugin, PostUpdate};

use super::entity::EntityComponent;
use super::layer::LayerComponent;
use super::level::LevelComponent;
use super::project::ProjectComponent;
use super::systems::{
    find_and_mark_loaded_components, find_and_unmark_just_loaded_components, insert_name_component,
    spawn_children,
};
use super::world::WorldComponent;
use super::ShieldtankComponentFinalized;

pub struct ComponentPlugin;
impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ProjectComponent>()
            .register_type::<WorldComponent>()
            .register_type::<LevelComponent>()
            .register_type::<LayerComponent>()
            .register_type::<EntityComponent>()
            .register_type::<ShieldtankComponentFinalized>()
            .add_systems(
                PostUpdate,
                (
                    find_and_mark_loaded_components,
                    find_and_unmark_just_loaded_components,
                    insert_name_component,
                    spawn_children,
                ),
            );
    }
}
