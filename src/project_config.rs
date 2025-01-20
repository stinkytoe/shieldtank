use bevy_app::Plugin;
use bevy_asset::{Asset, AssetApp};
use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Asset, Reflect, Deserialize, Serialize)]
pub struct ProjectConfig {}

pub struct ProjectConfigPlugin;
impl Plugin for ProjectConfigPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_asset::<ProjectConfig>()
            .register_asset_reflect::<ProjectConfig>();
    }
}
