use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin;

use crate::component::plugin::ComponentPlugin;
use crate::project_config::ProjectConfigPlugin;

pub struct ShieldtankPlugins;

impl PluginGroup for ShieldtankPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BevyLdtkAssetPlugin)
            .add(ProjectConfigPlugin)
            .add(ComponentPlugin)
    }
}
