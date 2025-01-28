use bevy_ldtk_asset::world::World as WorldAsset;

use crate::commands::ShieldtankItemCommands;
use crate::component::world::WorldComponentQueryData;

pub type WorldCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, WorldAsset, WorldComponentQueryData<'w>>;
