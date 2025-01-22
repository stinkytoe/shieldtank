use bevy_ldtk_asset::level::Level as LevelAsset;

use crate::component::level::LevelComponentQueryData;

use super::ShieldtankItemCommands;

pub type LevelCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, LevelAsset, LevelComponentQueryData<'w>>;
