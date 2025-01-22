use bevy_ldtk_asset::layer::Layer as LayerAsset;

use crate::component::layer::LayerComponentQueryData;

use super::ShieldtankItemCommands;

pub type LayerCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, LayerAsset, LayerComponentQueryData<'w>>;
