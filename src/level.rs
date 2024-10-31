use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::reflect::Reflect;
use bevy_ldtk_asset::prelude::ldtk_asset;

use crate::project_config::ProjectConfig;

#[derive(Component, Debug, Reflect)]
pub struct Level {
    pub handle: Handle<ldtk_asset::Level>,
    pub config: Handle<ProjectConfig>,
}
