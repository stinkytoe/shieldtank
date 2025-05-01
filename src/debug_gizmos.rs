use bevy_app::{Plugin, Update};
use bevy_asset::{AsAssetId, Assets};
use bevy_color::palettes::css::{LIGHT_SEA_GREEN, RED, YELLOW};
use bevy_ecs::query::With;
use bevy_ecs::system::Res;
use bevy_ecs::{entity::Entity, resource::Resource};
use bevy_gizmos::gizmos::Gizmos;
use bevy_ldtk_asset::entity::EntityInstance;
use bevy_reflect::Reflect;

use crate::component::entity::LdtkEntity;
use crate::component::grid_values::LdtkGridValues;
use crate::query::grid_value::GridValueQuery;
use crate::{
    component::global_bounds::LdtkGlobalBounds,
    query::{entity::LdtkEntityQuery, layer::LdtkLayerQuery, level::LdtkLevelQuery},
};

#[derive(Debug, Default, Resource, Reflect)]
pub struct DebugGizmos {
    pub level_gizmos: bool,
    pub layer_gizmos: bool,
    pub grid_values_query: bool,
    pub entity_gizmos: bool,
}

impl DebugGizmos {}

#[allow(clippy::too_many_arguments)]
fn debug_gizmos_system(
    debug_gizmos: Res<DebugGizmos>,
    level_query: LdtkLevelQuery<&LdtkGlobalBounds>,
    layer_query: LdtkLayerQuery<&LdtkGlobalBounds>,
    layer_with_grid_values_query: LdtkLayerQuery<Entity, With<LdtkGridValues>>,
    grid_values_query: GridValueQuery,
    entity_query: LdtkEntityQuery<(&LdtkEntity, &LdtkGlobalBounds)>,
    entity_assets: Res<Assets<EntityInstance>>,
    mut gizmos: Gizmos,
) {
    if debug_gizmos.level_gizmos {
        for global_bounds in level_query {
            let rect = global_bounds.bounds();
            let center = rect.center();
            let size = rect.size();
            gizmos.rect_2d(center, size, LIGHT_SEA_GREEN);
        }
    }

    if debug_gizmos.layer_gizmos {
        for global_bounds in layer_query.iter() {
            let rect = global_bounds.bounds();
            let center = rect.center();
            let size = rect.size();
            gizmos.rect_2d(center, size, YELLOW);
        }
    }

    if debug_gizmos.grid_values_query {
        for layer in layer_with_grid_values_query.iter() {
            grid_values_query
                .enumerate_layer(layer)
                .for_each(|(rect, grid_value)| {
                    let color = grid_value.color;
                    let center = rect.center();
                    let size = rect.size();
                    gizmos.rect_2d(center, size, color);
                });
        }
    }

    if debug_gizmos.entity_gizmos {
        for (ldtk_entity, global_bounds) in entity_query {
            let rect = global_bounds.bounds();
            let center = rect.center();
            let size = rect.size();
            let color = if let Some(asset) = entity_assets.get(ldtk_entity.as_asset_id()) {
                asset.smart_color.to_srgba()
            } else {
                RED
            };
            gizmos.rect_2d(center, size, color);
        }
    }
}

pub struct DebugGizmosPlugin;
impl Plugin for DebugGizmosPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<DebugGizmos>();
        app.insert_resource(DebugGizmos::default());
        app.add_systems(Update, debug_gizmos_system);
    }
}
