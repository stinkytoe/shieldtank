use bevy_app::Plugin;
use bevy_asset::{AsAssetId, Handle};
use bevy_ecs::component::Component;
use bevy_ldtk_asset::entity_definition::EntityDefinition as EntityDefinitionAsset;
use bevy_reflect::Reflect;

#[derive(Debug, Component, Reflect)]
pub struct ShieldtankEntityDefinition {
    handle: Handle<EntityDefinitionAsset>,
}

impl ShieldtankEntityDefinition {
    pub fn new(handle: Handle<EntityDefinitionAsset>) -> Self {
        Self { handle }
    }
}

impl AsAssetId for ShieldtankEntityDefinition {
    type Asset = EntityDefinitionAsset;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

pub struct EntityDefinitionPlugin;
impl Plugin for EntityDefinitionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ShieldtankEntityDefinition>();
    }
}
