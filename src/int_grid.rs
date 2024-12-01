use bevy_ecs::component::Component;
use bevy_ldtk_asset::layer::{Layer as LayerAsset, LayerType, TilesLayer};
use bevy_ldtk_asset::layer_definition::{IntGridValue, LayerDefinition};
use bevy_math::I64Vec2;
use bevy_reflect::Reflect;
use bevy_utils::HashMap;

use crate::{Error, Result};

#[derive(Component, Debug, Reflect)]
pub struct IntGrid {
    size: I64Vec2,
    values: HashMap<I64Vec2, IntGridValue>,
}

impl IntGrid {
    pub fn new(size: I64Vec2) -> Self {
        Self {
            size,
            values: HashMap::default(),
        }
    }

    pub fn from_layer(
        layer_instance: &LayerAsset,
        layer_definition: &LayerDefinition,
    ) -> Result<Self> {
        let LayerType::Tiles(TilesLayer { int_grid, .. }) = &layer_instance.layer_type else {
            return Err(Error::BadTilesLayer);
        };

        let size = layer_instance.grid_size;

        if int_grid.is_empty() {
            return Ok(Self::new(size));
        }

        let total_grids = (size.x * size.y) as usize;

        if total_grids != int_grid.len() {
            return Err(Error::BadIntGrid(
                "total grids and int grid length do not match!".to_string(),
            ));
        }

        let columns = size.x;

        let values = int_grid
            .iter()
            .enumerate()
            .filter(|(_, value)| **value != 0)
            .map(|(index, value)| (index as i64, value))
            .map(|(index, value)| {
                let grid = I64Vec2::new(index % columns, index / columns);
                let int_grid_value = layer_definition
                    .int_grid_values
                    .get(value)
                    .ok_or(Error::BadIntGrid(format!(
                        "int grid value of {value} not found in layer {}!",
                        layer_instance.identifier
                    )))?
                    .clone();

                Ok((grid, int_grid_value))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Self { size, values })
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if grid is in the region defined for this particular grid.
    ///
    /// Inclusive for the top left corner (0, 0)
    /// Exclusive for the bottom right corner (self.size)
    pub fn in_bounds(&self, grid: I64Vec2) -> bool {
        self.size.x > grid.x && self.size.y > grid.y
    }

    /// Gets Some(IntGridValue) if present, or None if either out of bounds or not present.
    pub fn get(&self, grid: I64Vec2) -> Option<IntGridValue> {
        self.values.get(&grid).cloned()
    }

    /// If grid is in bounds, then sets the value at that location to the given value.
    ///
    /// Returns true if value is accepted and set, or false if the grid is out of bounds.
    pub fn set(&mut self, grid: I64Vec2, value: IntGridValue) -> bool {
        self.in_bounds(grid)
            .then(|| self.values.insert(grid, value))
            .is_some()
    }
}
