use bevy_app::Plugin;
use bevy_derive::Deref;
use bevy_ecs::component::Component;
use bevy_ldtk_asset::field_instance::FieldInstance;
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;

use super::tile::Tile;

#[derive(Deref, Component, Reflect)]
pub struct FieldInstances {
    #[deref]
    pub field_instances: HashMap<String, FieldInstance>,
}

impl FieldInstances {
    pub fn new(field_instances: HashMap<String, FieldInstance>) -> Self {
        Self { field_instances }
    }
}

impl FieldInstances {
    pub fn get_array_tile(&self, identifier: &str, index: usize) -> Option<Tile> {
        self.field_instances
            .get(identifier)
            .and_then(|tiles| tiles.get_array_tile())
            .and_then(|tiles| tiles.get(index))
            .map(Tile::new)
    }

    pub fn get_array_string(&self, identifier: &str, index: usize) -> Option<&str> {
        self.field_instances
            .get(identifier)
            .and_then(|strings| strings.get_array_string())
            .and_then(|strings| strings.get(index))
            .map(|string| string.as_str())
    }
}
pub struct FieldInstancesPlugin;
impl Plugin for FieldInstancesPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<FieldInstances>();
    }
}
