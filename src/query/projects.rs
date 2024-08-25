use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::assets::project::LdtkProject;
use crate::iid::Iid;

#[derive(SystemParam)]
pub struct LdtkProjectsQuery<'w, 's> {
    projects_query: Query<'w, 's, (Entity, &'static Handle<LdtkProject>)>,
    project_assets: Res<'w, Assets<LdtkProject>>,
    asset_server: Res<'w, AssetServer>,
}

impl LdtkProjectsQuery<'_, '_> {
    pub fn get_project(&self, iid: Iid) -> Option<&LdtkProject> {
        self.project_assets
            .iter()
            .map(|(_, project)| project)
            .find(|project| project.iid == iid)
    }

    pub fn all_projects_loaded(&self) -> bool {
        !self
            .projects_query
            .iter()
            .map(|(_, handle)| handle)
            .any(|handle| !self.asset_server.is_loaded_with_dependencies(handle.id()))
    }
}
