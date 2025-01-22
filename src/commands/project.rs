use bevy_ldtk_asset::project::Project as ProjectAsset;

use crate::component::project::ProjectComponentQueryData;

use super::ShieldtankItemCommands;

pub type ProjectCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, ProjectAsset, ProjectComponentQueryData<'w>>;
