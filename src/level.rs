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

//impl Default for Level {
//    fn default() -> Self {
//        Self {
//            load_pattern: LoadPattern::All,
//            handle: Handle::default(),
//        }
//    }
//}

//impl crate::component_traits::LdtkComponent for Level {
//    type Asset = ldtk_asset::Level;
//
//    fn new(load_pattern: LoadPattern, handle: Handle<Self::Asset>) -> Self {
//        Self {
//            load_pattern,
//            handle,
//        }
//    }
//
//    fn get_handle(&self) -> &Handle<Self::Asset> {
//        &self.handle
//    }
//
//    fn get_load_pattern(&self) -> &LoadPattern {
//        &self.load_pattern
//    }
//}
//
//impl HasChildren for Level {
//    type Child = Layer;
//}
