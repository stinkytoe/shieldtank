use bevy_app::Plugin;
use bevy_asset::{AsAssetId, Handle};
use bevy_ecs::component::Component;
use bevy_ldtk_asset::tileset_definition::TilesetDefinition as TilesetDefinitionAsset;
use bevy_reflect::Reflect;

#[derive(Debug, Component, Reflect)]
pub struct ShieldtankTilesetDefinition {
    handle: Handle<TilesetDefinitionAsset>,
}

impl ShieldtankTilesetDefinition {
    pub fn new(handle: Handle<TilesetDefinitionAsset>) -> Self {
        Self { handle }
    }
}

impl AsAssetId for ShieldtankTilesetDefinition {
    type Asset = TilesetDefinitionAsset;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

pub struct TilesetDefinitionPlugin;
impl Plugin for TilesetDefinitionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ShieldtankTilesetDefinition>();
    }
}
