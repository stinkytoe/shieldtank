use bevy_ldtk_asset::project::Project as ProjectAsset;

use super::ShieldtankComponent;

pub type ProjectComponent = ShieldtankComponent<ProjectAsset>;

pub type ProjectComponentQueryData<'a> = ();
