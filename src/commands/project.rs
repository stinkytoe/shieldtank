use bevy_ldtk_asset::project::Project as ProjectAsset;

use crate::commands::ShieldtankItemCommands;
use crate::component::project::ProjectComponentQueryData;

pub type ProjectCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, ProjectAsset, ProjectComponentQueryData<'w>>;
