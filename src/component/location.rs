use bevy_app::Plugin;
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With, Without},
    system::{Commands, Query},
};
use bevy_log::debug;
use bevy_math::{IVec2, UVec2};
use bevy_reflect::Reflect;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::component::{
    entity::ShieldtankEntity, shieldtank_component::ShieldtankComponentSystemSet,
};

#[derive(Debug, Reflect)]
enum LocationState {
    Modified(IVec2),
    Static(IVec2),
}

#[derive(Debug, Component, Reflect)]
pub struct Location {
    state: LocationState,
}

impl From<&GlobalTransform> for Location {
    fn from(global_transform: &GlobalTransform) -> Self {
        let location = global_transform.translation().truncate().as_ivec2();
        let state = LocationState::Static(location);
        Location { state }
    }
}

#[allow(clippy::type_complexity)]
fn add_location_state(
    query: Query<(Entity, &GlobalTransform), (With<ShieldtankEntity>, Without<Location>)>,
    mut commands: Commands,
) {
    query.iter().for_each(|(entity, global_transform)| {
        commands
            .entity(entity)
            .insert(Location::from(global_transform));
    });
}

fn update_location_state(mut query: Query<(&mut Location, &GlobalTransform), Changed<Transform>>) {
    query
        .iter_mut()
        .filter(|(location, _)| matches!(location.state, LocationState::Static(_)))
        .for_each(|(mut location, global_transform)| {
            debug!(
                "update_location_state: {:?}",
                global_transform.translation().truncate()
            );
            *location = global_transform.into();
        });
}

pub struct LocationPlugin;
impl Plugin for LocationPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app //
            .register_type::<LocationState>()
            .register_type::<Location>()
            .add_systems(
                ShieldtankComponentSystemSet,
                (add_location_state, update_location_state),
            );
    }
}
