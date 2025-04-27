use bevy_ecs::query::With;
use bevy_ecs::system::{Query, SystemParam};
use bevy_math::{I64Vec2, Vec2};
use bevy_transform::components::GlobalTransform;

use crate::component::global_bounds::GlobalBounds;
use crate::component::grid_values::{GridValue, GridValues};
use crate::component::layer::LdtkLayer;

#[derive(SystemParam)]
pub struct GridValueQuery<'w, 's> {
    query: Query<
        'w,
        's,
        (
            &'static GlobalTransform,
            &'static GlobalBounds,
            &'static GridValues,
        ),
        With<LdtkLayer>,
    >,
}

impl GridValueQuery<'_, '_> {
    pub fn grid_value_at(&self, location: Vec2) -> Option<&GridValue> {
        let mut layers = self
            .query
            .iter()
            .filter(|(_, global_bounds, _)| global_bounds.contains(location))
            .map(|(global_transform, _, grid_values)| (global_transform, grid_values))
            .collect::<Vec<_>>();

        // sort with highest Z first
        layers.sort_by(|(global_transform_a, ..), (global_transform_b, ..)| {
            global_transform_b
                .translation()
                .z
                .partial_cmp(&global_transform_a.translation().z)
                .unwrap()
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
}
