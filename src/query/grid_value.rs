use std::cmp::Ordering;

use bevy_ecs::entity::Entity;
use bevy_ecs::query::{QueryData, With};
use bevy_ecs::system::{Query, SystemParam};
use bevy_math::{I64Vec2, Rect, Vec2};
use bevy_transform::components::GlobalTransform;

use crate::component::global_bounds::LdtkGlobalBounds;
use crate::component::grid_values::{LdtkGridValue, LdtkGridValues};
use crate::component::layer::ShieldtankLayer;

#[derive(QueryData)]
pub struct GridValueQueryData {
    global_transform: &'static GlobalTransform,
    global_bounds: &'static LdtkGlobalBounds,
    grid_values: &'static LdtkGridValues,
}

#[derive(SystemParam)]
pub struct GridValueQuery<'w, 's> {
    query: Query<'w, 's, GridValueQueryData, With<ShieldtankLayer>>,
}

impl GridValueQuery<'_, '_> {
    pub fn grid_value_at(&self, location: Vec2) -> Option<&LdtkGridValue> {
        let mut layers = self
            .query
            .iter()
            .filter(|data| data.global_bounds.contains(location))
            .map(|data| (data.global_transform, data.grid_values))
            .collect::<Vec<_>>();

        // sort with highest Z first
        layers.sort_by(|(global_transform_a, ..), (global_transform_b, ..)| {
            global_transform_b
                .translation()
                .z
                .partial_cmp(&global_transform_a.translation().z)
                .unwrap_or(Ordering::Less)
        });

        layers
            .into_iter()
            .find_map(|(global_transform, grid_values)| {
                let local_location = location - global_transform.translation().truncate();
                let local_location = I64Vec2::new(1, -1) * local_location.as_i64vec2() / 16;

                grid_values.get(local_location)
            })
    }

    pub fn identifier_at(&self, location: Vec2) -> Option<&str> {
        let grid = self.grid_value_at(location)?;
        let identifier = grid.identifier.as_ref()?;
        Some(identifier.as_str())
    }

    pub fn enumerate_layer(&self, layer: Entity) -> impl Iterator<Item = (Rect, &LdtkGridValue)> {
        let Ok(data) = self.query.get(layer) else {
            todo!()
        };

        let grid_cell_size = data.grid_values.grid_cell_size();
        let size = Vec2::new(1.0, -1.0) * Vec2::splat(grid_cell_size);

        let global_offset = data.global_transform.translation().truncate();

        data.grid_values.enumerate().map(move |(index, value)| {
            let x = index.x as f32 * grid_cell_size;
            let y = index.y as f32 * grid_cell_size;

            let corner = global_offset + Vec2::new(x, -y);

            let rect = Rect::from_corners(corner, corner + size);

            (rect, value)
        })
    }
}
