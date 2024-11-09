use bevy_ldtk_asset::project::Project as ProjectAsset;

use crate::component::LdtkComponent;
use crate::impl_unique_identifer_iterator;
use crate::item::LdtkItem;

pub type Project = LdtkComponent<ProjectAsset>;
pub type ProjectItem<'a> = LdtkItem<'a, ProjectAsset>;

impl_unique_identifer_iterator!(ProjectAsset);
