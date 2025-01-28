use bevy_ldtk_asset::layer::Layer as LayerAsset;

use crate::commands::ShieldtankItemCommands;
use crate::component::layer::LayerComponentQueryData;

pub type LayerCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, LayerAsset, LayerComponentQueryData<'w>>;

impl LayerCommands<'_, '_> {}
