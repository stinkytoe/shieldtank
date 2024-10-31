use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::reflect::Reflect;
use bevy_ldtk_asset::prelude::ldtk_asset;

use crate::project_config::ProjectConfig;

#[derive(Component, Debug, Default, Reflect)]
pub struct Layer {
    pub handle: Handle<ldtk_asset::Layer>,
    pub config: Handle<ProjectConfig>,
}

//impl LdtkComponent for Layer {
//    type Asset = ldtk_asset::Layer;
//
//    fn new(load_pattern: LoadPattern, handle: Handle<Self::Asset>) -> Self {
//        Layer {
//            handle,
//            load_pattern,
//        }
//    }
//
//    fn get_handle(&self) -> &Handle<Self::Asset> {
//        todo!()
//    }
//
//    fn get_load_pattern(&self) -> &LoadPattern {
//        todo!()
//    }
//}
