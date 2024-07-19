use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use thiserror::Error;

use crate::assets::project::LdtkProject;
use crate::iid::Iid;

#[derive(Debug, Error)]
pub enum LdtkProjectCommandsError {}

#[derive(SystemParam)]
pub struct LdtkProjectCommands<'w, 's> {
    _commands: Commands<'w, 's>,
    // _asset_server: Res<'w, AssetServer>,
    // project_loader: ResMut<'w, ProjectLoadManager>,
    project_assets: Res<'w, Assets<LdtkProject>>,
    // projects_query: Query<'w, 's, (Entity, &'static Handle<ProjectAsset>, &'static Iid)>,
}

impl<'w> LdtkProjectCommands<'w, '_> {
    pub fn get(&self, iid: Iid) -> Option<&LdtkProject> {
        self.project_assets
            .iter()
            .find(|(_, project_asset)| project_asset.iid == iid)
            .map(|(_, project_asset)| project_asset)
    }
}
