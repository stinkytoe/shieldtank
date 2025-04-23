use bevy_asset::Assets;
use bevy_color::Color;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::Changed;
use bevy_ecs::removal_detection::RemovedComponents;
use bevy_ecs::system::{Commands, Query, ResMut};
use bevy_log::debug;
use bevy_math::Vec2;
use bevy_math::primitives::Rectangle;
use bevy_reflect::Reflect;
use bevy_render::mesh::{Mesh, Mesh2d};
use bevy_sprite::{ColorMaterial, MeshMaterial2d};
use bevy_transform::components::Transform;

#[derive(Debug, Component, Reflect)]
pub struct LevelBackgroundColor {
    pub color: Color,
    pub size: Vec2,
}

pub fn level_background_color_system(
    query: Query<(Entity, &LevelBackgroundColor), Changed<LevelBackgroundColor>>,
    mut removed: RemovedComponents<LevelBackgroundColor>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    query.iter().for_each(|(entity, component)| {
        let width = component.size.x;
        let height = -component.size.y;

        let mesh_2d = Mesh2d(meshes.add(Rectangle::new(width, height)));
        let material = MeshMaterial2d(materials.add(component.color));
        let transform = Transform::from_xyz(width / 2.0, height / 2.0, 0.0);
        let child = commands.spawn((mesh_2d, material, transform)).id();

        debug!("Processing LevelBackgroundColor for {entity:?}");

        commands.entity(entity).add_child(child);
    });

    removed.read().for_each(|entity| {
        commands.entity(entity).remove::<Mesh2d>();
    });
}
