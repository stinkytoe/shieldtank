use bevy_ldtk_asset::world::World as WorldAsset;

use crate::component::world::WorldComponentQueryData;

use super::Item;

pub type WorldItem<'w, 's> = Item<'w, 's, WorldAsset, WorldComponentQueryData<'w>>;
