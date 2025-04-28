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
use bevy_ldtk_asset::layer_definition::LayerDefinition as LayerDefinitionAsset;
use bevy_math::I64Vec2;
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;

use crate::shieldtank_error;

use super::layer::LdtkLayer;
use super::layer_definition::LdtkLayerDefinition;
use super::shieldtank_component::ShieldtankComponentSystemSet;
use super::tile::LdtkTile;

#[derive(Clone, Debug, Reflect)]
pub struct LdtkGridValue {
    pub color: Color,
    pub identifier: Option<String>,
    pub tile: Option<LdtkTile>,
    pub value: i64,
}

impl LdtkGridValue {
    pub(crate) fn new(value: &LdtkIntGridValue) -> Self {
        let color = value.color;
        let identifier = value.identifier.clone();
        let tile = value.tile.as_ref().map(LdtkTile::new);
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
pub struct LdtkGridValues {
    size: I64Vec2,
    grid_cell_size: f32,
    values: HashMap<I64Vec2, LdtkGridValue>,
}

impl LdtkGridValues {
    pub fn new(
        size: I64Vec2,
        int_grid: &[i64],
        layer_definition_asset: &LayerDefinitionAsset,
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

                let value = layer_definition_asset
                    .int_grid_values
                    .get(i)
                    .ok_or(shieldtank_error!("bad int grid value: {i}"))?;

                Ok((key, LdtkGridValue::new(value)))
            })
            .collect::<bevy_ecs::error::Result<HashMap<_, _>>>()?;

        let grid_cell_size = layer_definition_asset.grid_cell_size as f32;

        Ok(Self {
            size,
            grid_cell_size,
            values,
        })
    }

    pub fn get(&self, grid: I64Vec2) -> Option<&LdtkGridValue> {
        self.values.get(&grid)
    }

    pub fn grid_cell_size(&self) -> f32 {
        self.grid_cell_size
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (I64Vec2, &LdtkGridValue)> {
        self.values.iter().map(|(index, value)| (*index, value))
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn grid_values_system(
    query: Query<
        (Entity, &LdtkLayer, &LdtkLayerDefinition),
        Or<(
            Changed<LdtkLayer>,
            AssetChanged<LdtkLayer>,
            Changed<LdtkLayerDefinition>,
            AssetChanged<LdtkLayerDefinition>,
        )>,
    >,
    component_assets: Res<Assets<LayerInstance>>,
    layer_definition_assets: Res<Assets<LayerDefinitionAsset>>,
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

                let grid_values = LdtkGridValues::new(size, int_grid, ldtk_layer_definition)?;

                commands.entity(entity).insert(grid_values);

                Ok(())
            },
        )
}

pub struct GridValuesPlugin;
impl Plugin for GridValuesPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LdtkGridValues>();
        app.add_systems(ShieldtankComponentSystemSet, grid_values_system);
    }
}
