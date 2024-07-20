use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use thiserror::Error;

use crate::assets::project::LdtkProject;
use crate::iid::Iid;

#[derive(Debug, Error)]
pub enum LdtkProjectQueryError {}

#[derive(SystemParam)]
pub struct LdtkProjectQuery<'w, 's> {
    asset_server: Res<'w, AssetServer>,
    project_assets: Res<'w, Assets<LdtkProject>>,
    projects_query: Query<'w, 's, (Entity, &'static Handle<LdtkProject>)>,
}

impl<'w> LdtkProjectQuery<'w, '_> {
    pub fn get(&self, iid: Iid) -> Option<&LdtkProject> {
        self.project_assets
            .iter()
            .find(|(_, project_asset)| project_asset.iid == iid)
            .map(|(_, project_asset)| project_asset)
    }

    pub fn all_projects_loaded(&self) -> bool {
        !self
            .projects_query
            .iter()
            .any(|(_, handle)| !self.asset_server.is_loaded_with_dependencies(handle.id()))
    }
}
