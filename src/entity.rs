use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::reflect::Reflect;
use bevy_ldtk_asset::prelude::ldtk_asset;

use crate::project_config::ProjectConfig;

#[derive(Component, Debug, Default, Reflect)]
pub struct Entity {
    pub handle: Handle<ldtk_asset::Entity>,
    pub config: Handle<ProjectConfig>,
}
