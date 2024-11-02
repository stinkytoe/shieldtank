use bevy::ecs::component::Component;
use bevy::math::I64Vec2;
use bevy::reflect::Reflect;
use bevy::utils::HashMap;
use bevy_ldtk_asset::layer::{Layer as LayerAsset, LayerType, TilesLayer};
use bevy_ldtk_asset::layer_definition::LayerDefinition;
use bevy_ldtk_asset::tileset_rectangle::TilesetRectangle;

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

        let int_grid_values: &[_] = &layer_definition.int_grid_values;

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
            .map(|(index, value)| (index as i64, value))
            .map(|(index, value)| (I64Vec2::new(index / columns, index % columns), value))
            .map(|(grid, &value)| {
                Ok((
                    grid,
                    int_grid_values
                        .iter()
                        .find(|int_grid_value| int_grid_value.value == value)
                        .map(IntGridValue::new)
                        .ok_or(Error::BadIntGrid(format!(
                            "int grid value of {value} not found in layer {}!",
                            layer_instance.identifier
                        )))?,
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(Self { size, values })
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

#[derive(Clone, Debug, Reflect)]
pub struct IntGridValue {
    pub identifier: Option<String>,
    //pub handle: Handle<Image>,
    pub group_uid: i64,
    pub tile: Option<TilesetRectangle>,
    pub value: i64,
}

impl IntGridValue {
    pub(crate) fn new(int_grid_value: &bevy_ldtk_asset::layer_definition::IntGridValue) -> Self {
        let identifier = int_grid_value.identifier.clone();
        let group_uid = int_grid_value.group_uid;
        let tile = int_grid_value.tile.clone();
        let value = int_grid_value.value;

        Self {
            identifier,
            group_uid,
            tile,
            value,
        }
    }
}
