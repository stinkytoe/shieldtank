use bevy_ldtk_asset::project::Project as ProjectAsset;

use crate::component::LdtkComponent;
use crate::item::LdtkItem;

pub type Project = LdtkComponent<ProjectAsset>;
pub type ProjectItem<'a> = LdtkItem<'a, ProjectAsset>;
