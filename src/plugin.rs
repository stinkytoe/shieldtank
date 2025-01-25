use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin;

use crate::component::plugin::ComponentPlugin;
use crate::int_grid::IntGridPlugin;
use crate::item::entity::plugin::EntityItemPlugin;
use crate::item::layer::plugin::LayerItemPlugin;
use crate::item::level::plugin::LevelItemPlugin;
use crate::level_background::LevelBackgroundPlugin;
use crate::project_config::ProjectConfigPlugin;
use crate::tiles::TilesPlugin;
use crate::tileset_rectangle::TilesetRectanglePlugin;

pub struct ShieldtankPlugins;

impl PluginGroup for ShieldtankPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BevyLdtkAssetPlugin)
            .add(ComponentPlugin)
            .add(IntGridPlugin)
            .add(EntityItemPlugin)
            .add(LevelItemPlugin)
            .add(LayerItemPlugin)
            .add(LevelBackgroundPlugin)
            .add(ProjectConfigPlugin)
            .add(TilesPlugin)
            .add(TilesetRectanglePlugin)
    }
}
