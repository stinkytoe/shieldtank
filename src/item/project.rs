use bevy_ldtk_asset::project::Project as ProjectAsset;

use crate::component::project::ProjectComponentQueryData;

use super::Item;

pub type ProjectItem<'w, 's> = Item<'w, 's, ProjectAsset, ProjectComponentQueryData<'w>>;

impl ProjectItem<'_, '_> {}
