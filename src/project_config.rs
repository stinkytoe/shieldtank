use bevy_app::Plugin;
use bevy_asset::Asset;
use bevy_asset::AssetApp;
use bevy_asset::AssetLoader;
use bevy_asset::AsyncReadExt;
use bevy_reflect::Reflect;
use bevy_tasks::block_on;
use serde::Deserialize;
use serde::Serialize;

use crate::load_pattern::LoadPattern;
use crate::{Error, Result};

#[derive(Asset, Debug, Reflect, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub load_pattern: LoadPattern,
    pub layer_z_factor: f32,
    pub level_z_factor: f32,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            load_pattern: Default::default(),
            layer_z_factor: 0.1,
            level_z_factor: 1.0,
        }
    }
}

#[derive(Default)]
pub struct ProjectConfigLoader;

impl AssetLoader for ProjectConfigLoader {
    type Asset = ProjectConfig;

    type Settings = ();

    type Error = Error;

    async fn load(
        &self,
        reader: &mut dyn bevy_asset::io::Reader,
        _settings: &(),
        _load_context: &mut bevy_asset::LoadContext<'_>,
    ) -> Result<Self::Asset> {
        let config = {
            let mut buf = String::new();
            block_on(async { reader.read_to_string(&mut buf).await })?;
            ron::de::from_str(&buf)?
        };

        Ok(config)
    }

    fn extensions(&self) -> &[&str] {
        &["project_config.ron"]
    }
}

pub struct ProjectConfigPlugin;

impl Plugin for ProjectConfigPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_asset::<ProjectConfig>()
            .init_asset_loader::<ProjectConfigLoader>()
            .register_asset_reflect::<ProjectConfig>();
    }
}
