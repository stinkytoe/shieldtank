use bevy_asset::Assets;
use bevy_color::Color;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::hierarchy::Children;
use bevy_ecs::lifecycle::RemovedComponents;
use bevy_ecs::name::Name;
use bevy_ecs::query::{Changed, With};
use bevy_ecs::system::{Commands, Query, ResMut};
use bevy_log::debug;
use bevy_math::Vec2;
use bevy_math::primitives::Rectangle;
use bevy_mesh::{Mesh, Mesh2d};
use bevy_reflect::Reflect;
use bevy_sprite_render::{ColorMaterial, MeshMaterial2d};
use bevy_transform::components::Transform;

#[derive(Debug, Component, Reflect)]
pub struct ShieldtankLevelBackgroundColor {
    pub color: Color,
    pub size: Vec2,
}

#[derive(Component)]
pub(crate) struct ShieldtankLevelBackgroundColorMesh;

pub(crate) fn level_background_color_system(
    query: Query<
        (Entity, &ShieldtankLevelBackgroundColor, Option<&Children>),
        Changed<ShieldtankLevelBackgroundColor>,
    >,
    background_children_query: Query<Entity, With<ShieldtankLevelBackgroundColorMesh>>,
    mut removed: RemovedComponents<ShieldtankLevelBackgroundColor>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    query.iter().for_each(|(entity, component, children)| {
        // remove old meshes, if any
        children.into_iter().for_each(|children| {
            children
                .into_iter()
                .copied()
                .filter_map(|child| background_children_query.get(child).ok())
                .for_each(|child| {
                    commands.entity(child).despawn();
                });
        });

        let width = component.size.x;
        let height = -component.size.y;

        let name = Name::new("background color mesh");
        let mesh_2d = Mesh2d(meshes.add(Rectangle::new(width, height)));
        let material = MeshMaterial2d(materials.add(component.color));
        let transform = Transform::from_xyz(width / 2.0, height / 2.0, 0.0);
        let background_color_mesh = ShieldtankLevelBackgroundColorMesh;
        let child = commands
            .spawn((name, mesh_2d, material, transform, background_color_mesh))
            .id();

        debug!("Processing LevelBackgroundColor for {entity:?}");

        commands.entity(entity).add_child(child);
    });

    removed.read().for_each(|entity| {
        commands.entity(entity).remove::<Mesh2d>();
    });
}
