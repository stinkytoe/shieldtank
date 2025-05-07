use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin;

use crate::component::entity::LdtkEntityPlugin;
use crate::component::entity_definition::EntityDefinitionPlugin;
use crate::component::field_instances::FieldInstancesPlugin;
use crate::component::global_bounds::GlobalBoundsPlugin;
use crate::component::grid_values::GridValuesPlugin;
use crate::component::iid::IidPlugin;
use crate::component::layer::LdtkLayerPlugin;
use crate::component::layer_definition::LayerDefinitionPlugin;
use crate::component::layer_tiles::LayerTilePlugin;
use crate::component::level::LdtkLevelPlugin;
use crate::component::level_background::LevelBackgroundPlugin;
use crate::component::project::LdtkProjectPlugin;
use crate::component::spawn_children::SpawnChildrenPlugin;
use crate::component::tags::TagsPlugin;
use crate::component::tile::TilePlugin;
use crate::component::tileset_definition::TilesetDefinitionPlugin;
use crate::component::world::LdtkWorldPlugin;
use crate::debug_gizmos::DebugGizmosPlugin;

pub struct ShieldtankPlugins;

impl PluginGroup for ShieldtankPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // Inherit bevy_ldtk_asset
            .add(BevyLdtkAssetPlugin)
            // Core Components
            .add(LdtkProjectPlugin)
            .add(LdtkWorldPlugin)
            .add(LdtkLevelPlugin)
            .add(LdtkLayerPlugin)
            .add(LdtkEntityPlugin)
            .add(SpawnChildrenPlugin)
            // LDtk definitions
            .add(EntityDefinitionPlugin)
            .add(LayerDefinitionPlugin)
            .add(TilesetDefinitionPlugin)
            .add(IidPlugin)
            .add(FieldInstancesPlugin)
            // Visual Components
            .add(LayerTilePlugin)
            .add(LevelBackgroundPlugin)
            .add(GlobalBoundsPlugin)
            .add(GridValuesPlugin)
            .add(TagsPlugin)
            .add(TilePlugin)
            // Debug Gizmos
            .add(DebugGizmosPlugin)
    }
}
