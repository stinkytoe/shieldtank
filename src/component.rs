use bevy_asset::{Asset, AssetServer, Handle};
use bevy_ecs::component::Component;
use bevy_log::debug;
use bevy_reflect::Reflect;

use crate::project_config::ProjectConfig;

#[derive(Component, Debug, Reflect)]
pub struct LdtkComponent<A: Asset> {
    pub handle: Handle<A>,
    pub config: Handle<ProjectConfig>,
}

impl<A: Asset> LdtkComponentExt<A> for LdtkComponent<A> {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        let handle_loaded = asset_server.is_loaded(self.handle.id());
        let config_loaded = asset_server.is_loaded(self.config.id());

        debug!(
            "Project Config load_state: {:?}",
            asset_server.load_state(self.config.id())
        );

        debug!(
            "Project Config get_load_state: {:?}",
            asset_server.get_load_state(self.config.id())
        );

        debug!("Project is_loaded: {handle_loaded} {config_loaded}");

        handle_loaded && config_loaded
    }

    fn get_handle(&self) -> Handle<A> {
        self.handle.clone()
    }

    fn get_config_handle(&self) -> Handle<ProjectConfig> {
        self.config.clone()
    }
}

pub(crate) trait LdtkComponentExt<A: Asset> {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool;
    fn get_handle(&self) -> Handle<A>;
    fn get_config_handle(&self) -> Handle<ProjectConfig>;
}
