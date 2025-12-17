use bevy_app::Plugin;
use bevy_asset::{AsAssetId, Handle};
use bevy_ecs::component::Component;
use bevy_ldtk_asset::layer_definition::LayerDefinition as LayerDefinitionAsset;
use bevy_reflect::Reflect;

#[derive(Debug, Default, Component, Reflect)]
pub struct ShieldtankLayerDefinition {
    pub handle: Handle<LayerDefinitionAsset>,
}

impl ShieldtankLayerDefinition {
    pub(crate) fn new(handle: Handle<LayerDefinitionAsset>) -> Self {
        Self { handle }
    }
}

impl AsAssetId for ShieldtankLayerDefinition {
    type Asset = LayerDefinitionAsset;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

pub struct LayerDefinitionPlugin;
impl Plugin for LayerDefinitionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ShieldtankLayerDefinition>();
    }
}
