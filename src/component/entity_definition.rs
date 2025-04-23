use bevy_app::Plugin;
use bevy_asset::{AsAssetId, Handle};
use bevy_ecs::component::Component;
use bevy_ldtk_asset::entity_definition::EntityDefinition as LdtkEntityDefinition;
use bevy_reflect::Reflect;

#[derive(Debug, Component, Reflect)]
pub struct EntityDefinition {
    handle: Handle<LdtkEntityDefinition>,
}

impl EntityDefinition {
    pub fn new(handle: Handle<LdtkEntityDefinition>) -> Self {
        Self { handle }
    }
}

impl AsAssetId for EntityDefinition {
    type Asset = LdtkEntityDefinition;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

pub struct EntityDefinitionPlugin;
impl Plugin for EntityDefinitionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<EntityDefinition>();
    }
}
