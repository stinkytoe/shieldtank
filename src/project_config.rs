use bevy_app::Plugin;
use bevy_asset::{Asset, AssetApp};
use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};

use crate::load_pattern::LoadPattern;

#[derive(Debug, Asset, Reflect, Deserialize, Serialize)]
pub struct ProjectConfig {
    load_pattern: LoadPattern,
    load_level_background: LoadPattern,
    load_layer_tiles: LoadPattern,
    load_entity_tileset_rectangles: LoadPattern,
    levels_override_transform: bool,
    layers_override_transform: bool,
    entities_override_transform: bool,
    level_separation: f32,
    layer_separation: f32,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            load_pattern: Default::default(),
            load_level_background: Default::default(),
            load_layer_tiles: Default::default(),
            load_entity_tileset_rectangles: Default::default(),
            levels_override_transform: true,
            layers_override_transform: true,
            entities_override_transform: true,
            level_separation: 10.0,
            layer_separation: 1.0,
        }
    }
}

impl ProjectConfig {
    pub fn get_load_pattern(&self) -> &LoadPattern {
        &self.load_pattern
    }

    pub fn get_load_level_background(&self) -> &LoadPattern {
        &self.load_level_background
    }

    pub fn get_layer_tiles(&self) -> &LoadPattern {
        &self.load_layer_tiles
    }

    pub fn get_load_entity_tileset_rectangles(&self) -> &LoadPattern {
        &self.load_entity_tileset_rectangles
    }

    pub fn levels_override_transform(&self) -> bool {
        self.levels_override_transform
    }

    pub fn layers_override_transform(&self) -> bool {
        self.layers_override_transform
    }

    pub fn entities_override_transform(&self) -> bool {
        self.entities_override_transform
    }

    pub fn level_separation(&self) -> f32 {
        self.level_separation
    }

    pub fn layer_separation(&self) -> f32 {
        self.layer_separation
    }
}

pub struct ProjectConfigPlugin;
impl Plugin for ProjectConfigPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_asset::<ProjectConfig>()
            .register_asset_reflect::<ProjectConfig>();
    }
}
