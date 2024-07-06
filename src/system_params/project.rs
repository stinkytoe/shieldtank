use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::assets::project::LdtkProject;
use crate::iid::Iid;

#[derive(SystemParam)]
pub struct LdtkProjectCommands<'w, 's> {
    _commands: Commands<'w, 's>,
    _asset_server: Res<'w, AssetServer>,
    // project_loader: ResMut<'w, ProjectLoadManager>,
    // project_assets: Res<'w, Assets<ProjectAsset>>,
    // projects_query: Query<'w, 's, (Entity, &'static Handle<ProjectAsset>, &'static Iid)>,
}

impl<'w> LdtkProjectCommands<'w, '_> {
    pub(crate) fn get(iid: Iid) -> Option<&'w LdtkProject> {
        todo!()
    }
}
