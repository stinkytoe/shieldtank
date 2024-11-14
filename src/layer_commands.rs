use bevy_ldtk_asset::layer::Layer as LayerAsset;

use crate::item_commands::LdtkItemCommands;
pub type LayerCommands<'a> = LdtkItemCommands<'a, LayerAsset>;

impl LayerCommands<'_> {}
