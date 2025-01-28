use bevy_ldtk_asset::level::Level as LevelAsset;

use crate::commands::ShieldtankItemCommands;
use crate::component::level::LevelComponentQueryData;

pub type LevelCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, LevelAsset, LevelComponentQueryData<'w>>;

impl LevelCommands<'_, '_> {}
