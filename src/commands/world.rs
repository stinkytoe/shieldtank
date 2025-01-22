use bevy_ldtk_asset::world::World as WorldAsset;

use crate::component::world::WorldComponentQueryData;

use super::ShieldtankItemCommands;

pub type WorldCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, WorldAsset, WorldComponentQueryData<'w>>;
