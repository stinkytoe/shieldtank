use bevy_ldtk_asset::layer::Layer as LayerAsset;

use crate::component::layer::LayerComponentQueryData;

use super::Item;

pub type LayerItem<'w, 's> = Item<'w, 's, LayerAsset, LayerComponentQueryData<'w>>;
