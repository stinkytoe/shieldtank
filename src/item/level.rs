use bevy_ldtk_asset::level::Level as LevelAsset;

use crate::component::level::LevelComponentQueryData;

use super::Item;

pub type LevelItem<'w, 's> = Item<'w, 's, LevelAsset, LevelComponentQueryData<'w>>;
