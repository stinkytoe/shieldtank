use bevy_app::{Plugin, PostUpdate};

use crate::component::entity::EntityComponent;
use crate::component::layer::LayerComponent;
use crate::component::level::LevelComponent;
use crate::component::project::ProjectComponent;
use crate::component::systems::{
    find_and_mark_loaded_components, find_and_unmark_just_loaded_components, insert_name_component,
    spawn_children,
};
use crate::component::world::WorldComponent;
use crate::component::ShieldtankComponentFinalized;

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
