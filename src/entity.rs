use bevy_asset::Handle;
use bevy_ecs::component::Component;
use bevy_ldtk_asset::prelude::ldtk_asset;
use bevy_reflect::Reflect;

use crate::project_config::ProjectConfig;

#[derive(Component, Debug, Default, Reflect)]
pub struct Entity {
    pub handle: Handle<ldtk_asset::Entity>,
    pub config: Handle<ProjectConfig>,
}
