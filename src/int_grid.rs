use bevy_app::Plugin;
use bevy_color::Color;
use bevy_ecs::component::Component;
use bevy_ldtk_asset::layer::{Layer as LayerAsset, LayerType, TilesLayer};
use bevy_ldtk_asset::layer_definition::IntGridValue as LdtkIntGridValue;
use bevy_ldtk_asset::layer_definition::LayerDefinition;
use bevy_math::I64Vec2;
use bevy_reflect::Reflect;
use bevy_utils::HashMap;

use crate::error::Result;
use crate::shieldtank_error;
use crate::tileset_rectangle::TilesetRectangle;

#[derive(Clone, Debug, Reflect)]
pub struct IntGridValue {
    pub color: Color,
    pub group_uid: i64,
    pub identifier: Option<String>,
    pub tile: Option<TilesetRectangle>,
    pub value: i64,
}

impl IntGridValue {
    pub(crate) fn new(value: &LdtkIntGridValue) -> Self {
        let color = value.color;
        let group_uid = value.group_uid;
        let identifier = value.identifier.clone();
        let tile = value
            .tile
            .as_ref()
            .map(|ldtk_tileset_rectangle| TilesetRectangle::new(ldtk_tileset_rectangle.clone()));

        let value = value.value;

        Self {
            color,
            group_uid,
            identifier,
            tile,
            value,
        }
    }
}

// TODO: inspect this whole integration for improvements
#[derive(Component, Debug, Reflect)]
pub struct IntGrid {
    size: I64Vec2,
    values: HashMap<I64Vec2, IntGridValue>,
}

impl IntGrid {
    pub fn from_layer(
        layer_instance: &LayerAsset,
        layer_definition: &LayerDefinition,
    ) -> Result<Self> {
        let LayerType::Tiles(TilesLayer { int_grid, .. }) = &layer_instance.layer_type else {
            return Err(shieldtank_error!("Bad Tiles Layer!"));
        };

        let size = layer_instance.grid_size;

        let total_grids = (size.x * size.y) as usize;

        if total_grids != int_grid.len() {
            return Err(shieldtank_error!(
                "total grids and int grid length do not match!"
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
                let ldtk_int_grid_value =
                    layer_definition
                        .int_grid_values
                        .get(value)
                        .ok_or(shieldtank_error!(
                            "int grid value of {value} not found in layer {}!",
                            layer_instance.identifier
                        ))?;

                let int_grid_value = IntGridValue::new(ldtk_int_grid_value);

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
    pub fn get(&self, grid: I64Vec2) -> Option<&IntGridValue> {
        self.values.get(&grid)
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

pub struct IntGridPlugin;
impl Plugin for IntGridPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<IntGrid>();
    }
}
