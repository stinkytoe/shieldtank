use bevy_app::Plugin;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, With};
use bevy_ecs::system::{Commands, Query};
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_transform::components::GlobalTransform;

use super::entity::LdtkEntity;
use super::shieldtank_component::ShieldtankComponentSystemSet;

#[derive(Debug, Component, Reflect)]
pub struct LdtkLocation {
    location: Vec2,
}

#[allow(clippy::type_complexity)]
fn location_system(
    query: Query<(Entity, &GlobalTransform), (With<LdtkEntity>, Changed<GlobalTransform>)>,
    mut commands: Commands,
) {
    query.iter().for_each(|(entity, global_transform)| {
        let location = global_transform.translation().truncate();
        let location = LdtkLocation { location };
        commands.entity(entity).insert(location);
    });
}

pub struct LocationPlugin;
impl Plugin for LocationPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LdtkLocation>();
        app.add_systems(ShieldtankComponentSystemSet, location_system);
    }
}
