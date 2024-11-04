use bevy_asset::Handle;
use bevy_ecs::component::Component;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_reflect::Reflect;

use crate::project_config::ProjectConfig;

#[derive(Component, Debug, Default, Reflect)]
pub struct Entity {
    pub handle: Handle<EntityAsset>,
    pub config: Handle<ProjectConfig>,
}
