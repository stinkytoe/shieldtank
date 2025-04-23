use bevy_app::Plugin;
use bevy_asset::prelude::AssetChanged;
use bevy_asset::{AsAssetId, Assets};
use bevy_color::Color;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::{Commands, Query, Res};
use bevy_ldtk_asset::layer::LayerInstance;
use bevy_ldtk_asset::layer_definition::IntGridValue as LdtkIntGridValue;
use bevy_ldtk_asset::layer_definition::LayerDefinition as LdtkLayerDefinition;
use bevy_math::I64Vec2;
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;

use crate::shieldtank_error;

use super::layer::LdtkLayer;
use super::layer_definition::LayerDefinition;
use super::shieldtank_component::ShieldtankComponentSystemSet;
use super::tile::Tile;

#[derive(Clone, Debug, Reflect)]
pub struct GridValue {
    pub color: Color,
    pub identifier: Option<String>,
    pub tile: Option<Tile>,
    pub value: i64,
}

impl GridValue {
    pub(crate) fn new(value: &LdtkIntGridValue) -> Self {
        let color = value.color;
        let identifier = value.identifier.clone();
        let tile = value.tile.as_ref().map(Tile::new);
        let value = value.value;

        Self {
            color,
            identifier,
            tile,
            value,
        }
    }
}

#[derive(Debug, Component, Reflect)]
pub struct GridValues {
    size: I64Vec2,
    values: HashMap<I64Vec2, GridValue>,
}

impl GridValues {
    pub fn new(
        size: I64Vec2,
        int_grid: &[i64],
        ldtk_layer_definition: &LdtkLayerDefinition,
    ) -> bevy_ecs::error::Result<Self> {
        let values = int_grid
            .iter()
            .enumerate()
            .filter(|(_, i)| **i != 0)
            .map(|(index, i)| -> bevy_ecs::error::Result<_> {
                let index = index as i64;
                let x = index % size.y;
                let y = index / size.y;

                let key = I64Vec2::new(x, y);

                let value = ldtk_layer_definition
                    .int_grid_values
                    .get(i)
                    .ok_or(shieldtank_error!("bad int grid value: {i}"))?;

                Ok((key, GridValue::new(value)))
            })
            .collect::<bevy_ecs::error::Result<HashMap<_, _>>>()?;

        Ok(Self { size, values })
    }

    pub fn get(&self, grid: I64Vec2) -> Option<&GridValue> {
        self.values.get(&grid)
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn grid_values_system(
    query: Query<
        (Entity, &LdtkLayer, &LayerDefinition),
        Or<(
            Changed<LdtkLayer>,
            AssetChanged<LdtkLayer>,
            Changed<LayerDefinition>,
            AssetChanged<LayerDefinition>,
        )>,
    >,
    component_assets: Res<Assets<LayerInstance>>,
    layer_definition_assets: Res<Assets<LdtkLayerDefinition>>,
    mut commands: Commands,
) -> bevy_ecs::error::Result<()> {
    query
        .iter()
        .filter_map(|(entity, component, layer_definition)| {
            Some((
                entity,
                component_assets.get(component.as_asset_id())?,
                layer_definition_assets.get(layer_definition.as_asset_id())?,
            ))
        })
        .filter_map(|(entity, layer, layer_definition)| {
            Some((
                entity,
                layer,
                layer.layer_type.get_tiles_layer()?,
                layer_definition,
            ))
        })
        .try_for_each(
            |(entity, layer, tiles_layer, ldtk_layer_definition)| -> bevy_ecs::error::Result<()> {
                let size = layer.grid_size;
                let int_grid = tiles_layer.int_grid.as_slice();

                let grid_values = GridValues::new(size, int_grid, ldtk_layer_definition)?;

                commands.entity(entity).insert(grid_values);

                Ok(())
            },
        )
}

pub struct GridValuesPlugin;
impl Plugin for GridValuesPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<GridValues>();
        app.add_systems(ShieldtankComponentSystemSet, grid_values_system);
    }
}
