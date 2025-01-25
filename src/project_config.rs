use bevy_app::Plugin;
use bevy_asset::{Asset, AssetApp};
use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};

use crate::load_pattern::LoadPattern;

#[derive(Debug, Default, Asset, Reflect, Deserialize, Serialize)]
pub struct ProjectConfig {
    load_pattern: LoadPattern,
    load_level_background: LoadPattern,
}

impl ProjectConfig {
    pub fn get_load_pattern(&self) -> &LoadPattern {
        &self.load_pattern
    }

    pub fn get_load_level_background_pattern(&self) -> &LoadPattern {
        &self.load_level_background
    }
}

pub struct ProjectConfigPlugin;
impl Plugin for ProjectConfigPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_asset::<ProjectConfig>()
            .register_asset_reflect::<ProjectConfig>();
    }
}
