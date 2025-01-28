use bevy_ldtk_asset::world::World as WorldAsset;

use crate::component::ShieldtankComponent;

pub type WorldComponent = ShieldtankComponent<WorldAsset>;

pub type WorldComponentQueryData<'a> = ();
