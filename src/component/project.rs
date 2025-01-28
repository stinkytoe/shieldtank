use bevy_ldtk_asset::project::Project as ProjectAsset;

use crate::component::ShieldtankComponent;

pub type ProjectComponent = ShieldtankComponent<ProjectAsset>;

pub type ProjectComponentQueryData<'a> = ();
