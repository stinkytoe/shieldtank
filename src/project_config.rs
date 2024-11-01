use bevy::asset::Asset;
use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::reflect::Reflect;
use bevy::tasks::block_on;
use serde::Deserialize;
use serde::Serialize;

use crate::load_pattern::LoadPattern;
use crate::{Error, Result};

#[derive(Asset, Debug, Default, Reflect, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub load_pattern: LoadPattern,
    pub level_z_scale: f32,
}

#[derive(Default)]
pub struct ProjectConfigLoader;

impl AssetLoader for ProjectConfigLoader {
    type Asset = ProjectConfig;

    type Settings = ();

    type Error = Error;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &(),
        _load_context: &mut bevy::asset::LoadContext<'_>,
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
