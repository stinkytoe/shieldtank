use bevy::prelude::*;
use thiserror::Error;

use crate::assets::layer::LdtkLayerAssetError;
use crate::assets::layer::LdtkLayerType;
use crate::ldtk;
use crate::reexports::int_grid_value::IntGridValue;
use crate::reexports::int_grid_value::IntGridValueFromError;
use crate::reexports::int_grid_value_group::IntGridValueGroup;
use crate::reexports::int_grid_value_group::IntGridValueGroupFromError;

#[derive(Debug, Reflect)]
pub struct LayerDefinition {
    pub layer_definition_type: LdtkLayerType,
    pub auto_source_layer_def_uid: Option<i64>,
    pub display_opacity: f64,
    pub grid_cell_size: i64,
    pub identifier: String,
    pub int_grid_values: Vec<IntGridValue>,
    pub int_grid_values_groups: Vec<IntGridValueGroup>,
    pub parallax_factor_x: f64,
    pub parallax_factor_y: f64,
    pub parallax_scaling: bool,
    pub offset: Vec2,
    pub tileset_def_uid: Option<i64>,
    pub uid: i64,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum LayerDefinitionFromError {
    #[error(transparent)]
    LayerTypeError(#[from] LdtkLayerAssetError),
    #[error(transparent)]
    IntGridValueFromError(#[from] IntGridValueFromError),
    #[error(transparent)]
    IntGridValueGroupFromError(#[from] IntGridValueGroupFromError),
}

impl LayerDefinition {
    pub(crate) fn new(value: &ldtk::LayerDefinition) -> Result<Self, LayerDefinitionFromError> {
        Ok(Self {
            layer_definition_type: LdtkLayerType::new(&value.layer_definition_type)?,
            auto_source_layer_def_uid: value.auto_source_layer_def_uid,
            display_opacity: value.display_opacity,
            grid_cell_size: value.grid_size,
            identifier: value.identifier.clone(),
            int_grid_values: value
                .int_grid_values
                .iter()
                .map(IntGridValue::new)
                .collect::<Result<_, _>>()?,
            int_grid_values_groups: value
                .int_grid_values_groups
                .iter()
                .map(IntGridValueGroup::new)
                .collect::<Result<_, _>>()?,
            parallax_factor_x: value.parallax_factor_x,
            parallax_factor_y: value.parallax_factor_y,
            parallax_scaling: value.parallax_scaling,
            offset: (value.px_offset_x as f32, value.px_offset_y as f32).into(),
            tileset_def_uid: value.tileset_def_uid,
            uid: value.uid,
        })
    }
}
