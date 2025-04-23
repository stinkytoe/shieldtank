use bevy_app::Plugin;
use bevy_asset::{AsAssetId, Handle};
use bevy_ecs::component::Component;
use bevy_ldtk_asset::tileset_definition::TilesetDefinition as LdtkTilesetDefinition;
use bevy_reflect::Reflect;

#[derive(Debug, Component, Reflect)]
pub struct TilesetDefinition {
    handle: Handle<LdtkTilesetDefinition>,
}

impl TilesetDefinition {
    pub fn new(handle: Handle<LdtkTilesetDefinition>) -> Self {
        Self { handle }
    }
}

impl AsAssetId for TilesetDefinition {
    type Asset = LdtkTilesetDefinition;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

pub struct TilesetDefinitionPlugin;
impl Plugin for TilesetDefinitionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<TilesetDefinition>();
    }
}
