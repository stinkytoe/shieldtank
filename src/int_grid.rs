use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::math::UVec2;
use bevy::prelude::Image;
use bevy::reflect::Reflect;
use bevy::utils::HashMap;

#[derive(Component, Debug, Reflect)]
pub struct IntGrid {
    size: UVec2,
    values: HashMap<UVec2, IntGridValue>,
}

impl IntGrid {
    pub fn new(size: UVec2) -> Self {
        Self {
            size,
            values: HashMap::default(),
        }
    }

    /// Returns true if grid is in the region defined for this particular grid.
    ///
    /// Inclusive for the top left corner (0, 0)
    /// Exclusive for the bottom right corner (self.size)
    pub fn in_bounds(&self, grid: UVec2) -> bool {
        self.size.x > grid.x && self.size.y > grid.y
    }

    /// Gets Some(IntGridValue) if present, or None if either out of bounds or not present.
    pub fn get(&self, grid: UVec2) -> Option<IntGridValue> {
        self.values.get(&grid).cloned()
    }

    /// If grid is in bounds, then sets the value at that location to the given value.
    ///
    /// Returns true if value is accepted and set, or false if the grid is out of bounds.
    pub fn set(&mut self, grid: UVec2, value: IntGridValue) -> bool {
        self.in_bounds(grid)
            .then(|| self.values.insert(grid, value))
            .is_some()
    }
}

#[derive(Clone, Debug, Reflect)]
pub struct IntGridValue {
    handle: Handle<Image>,
}
