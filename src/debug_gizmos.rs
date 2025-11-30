use bevy_app::{Plugin, Update};
use bevy_asset::{AsAssetId, Assets};
use bevy_color::Color;
use bevy_color::palettes::css::{LIGHT_SEA_GREEN, RED, YELLOW};
use bevy_ecs::entity::Entity;
use bevy_ecs::query::With;
use bevy_ecs::resource::Resource;
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_gizmos::gizmos::Gizmos;
use bevy_input::ButtonInput;
use bevy_input::keyboard::KeyCode;
use bevy_ldtk_asset::entity::EntityInstance;
use bevy_reflect::Reflect;

use crate::component::entity::ShieldtankEntity;
use crate::component::global_bounds::LdtkGlobalBounds;
use crate::component::grid_values::LdtkGridValues;
use crate::component::layer::ShieldtankLayer;
use crate::component::level::ShieldtankLevel;
use crate::query::grid_value::GridValueQuery;

#[derive(Clone, Debug, Resource, Reflect)]
pub struct DebugGizmosSettings {
    pub level_gizmos: bool,
    pub layer_gizmos: bool,
    pub grid_values_query: bool,
    pub entity_gizmos: bool,

    pub levels_key: KeyCode,
    pub layers_key: KeyCode,
    pub grid_values_key: KeyCode,
    pub entities_key: KeyCode,

    pub levels_color: Color,
    pub layers_color: Color,
    pub grid_values_color_override: Option<Color>,
    pub entities_color_override: Option<Color>,
}

impl Default for DebugGizmosSettings {
    fn default() -> Self {
        Self {
            level_gizmos: false,
            layer_gizmos: false,
            grid_values_query: false,
            entity_gizmos: false,

            levels_key: KeyCode::F1,
            layers_key: KeyCode::F2,
            grid_values_key: KeyCode::F3,
            entities_key: KeyCode::F4,

            levels_color: Color::Srgba(LIGHT_SEA_GREEN),
            layers_color: Color::Srgba(YELLOW),
            grid_values_color_override: None,
            entities_color_override: None,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn debug_gizmos_system(
    debug_gizmos: Res<DebugGizmosSettings>,
    // level_query: LdtkLevelQuery<&LdtkGlobalBounds>,
    // layer_query: LdtkLayerQuery<&LdtkGlobalBounds>,
    level_query: Query<&LdtkGlobalBounds, With<ShieldtankLevel>>,
    layer_query: Query<&LdtkGlobalBounds, With<ShieldtankLayer>>,
    layer_with_grid_values_query: Query<Entity, With<LdtkGridValues>>,
    grid_values_query: GridValueQuery,
    entity_query: Query<(&ShieldtankEntity, &LdtkGlobalBounds)>,
    entity_assets: Res<Assets<EntityInstance>>,
    mut gizmos: Gizmos,
) {
    if debug_gizmos.level_gizmos {
        for global_bounds in level_query {
            let rect = global_bounds.bounds();
            let center = rect.center();
            let size = rect.size();
            gizmos.rect_2d(center, size, debug_gizmos.levels_color);
        }
    }

    if debug_gizmos.layer_gizmos {
        for global_bounds in layer_query.iter() {
            let rect = global_bounds.bounds();
            let center = rect.center();
            let size = rect.size();
            gizmos.rect_2d(center, size, debug_gizmos.layers_color);
        }
    }

    if debug_gizmos.grid_values_query {
        for layer in layer_with_grid_values_query.iter() {
            grid_values_query
                .enumerate_layer(layer)
                .for_each(|(rect, grid_value)| {
                    let color = debug_gizmos
                        .grid_values_color_override
                        .unwrap_or(grid_value.color);

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

            let color = debug_gizmos.entities_color_override.unwrap_or_else(|| {
                entity_assets
                    .get(ldtk_entity.as_asset_id())
                    .map(|asset| asset.smart_color)
                    .unwrap_or(Color::from(RED))
            });

            gizmos.rect_2d(center, size, color);
        }
    }
}

fn debug_gizmos_keyboard_commands(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut debug_gizmos: ResMut<DebugGizmosSettings>,
) {
    if keyboard_input.just_pressed(debug_gizmos.levels_key) {
        debug_gizmos.level_gizmos = !debug_gizmos.level_gizmos;
    }

    if keyboard_input.just_pressed(debug_gizmos.layers_key) {
        debug_gizmos.layer_gizmos = !debug_gizmos.layer_gizmos;
    }

    if keyboard_input.just_pressed(debug_gizmos.grid_values_key) {
        debug_gizmos.grid_values_query = !debug_gizmos.grid_values_query;
    }

    if keyboard_input.just_pressed(debug_gizmos.entities_key) {
        debug_gizmos.entity_gizmos = !debug_gizmos.entity_gizmos;
    }
}

#[derive(Clone, Default, Debug)]
pub struct DebugGizmosPlugin {
    settings: DebugGizmosSettings,
}

impl Plugin for DebugGizmosPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<DebugGizmosSettings>();
        app.insert_resource(self.settings.clone());
        app.add_systems(Update, debug_gizmos_system);
        app.add_systems(Update, debug_gizmos_keyboard_commands);
    }
}
