use bevy_app::Plugin;
use bevy_derive::Deref;
use bevy_ecs::component::Component;
use bevy_ldtk_asset::field_instance::FieldInstance;
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;

use super::tile::LdtkTile;

#[derive(Deref, Component, Reflect)]
pub struct LdtkFieldInstances {
    #[deref]
    pub field_instances: HashMap<String, FieldInstance>,
}

impl LdtkFieldInstances {
    pub fn new(field_instances: HashMap<String, FieldInstance>) -> Self {
        Self { field_instances }
    }
}

impl LdtkFieldInstances {
    pub fn get_array_tile(&self, identifier: &str) -> Option<Vec<LdtkTile>> {
        self.field_instances
            .get(identifier)
            .and_then(|tiles| tiles.get_array_tile())
            .map(|tiles| tiles.iter().map(LdtkTile::new).collect())
    }

    pub fn get_array_string(&self, identifier: &str) -> Option<&Vec<String>> {
        self.field_instances
            .get(identifier)
            .and_then(|strings| strings.get_array_string())
    }

    pub fn get_string(&self, identifier: &str) -> Option<&str> {
        self.field_instances
            .get(identifier)
            .and_then(|string| string.get_string())
            .map(|string| string.as_str())
    }

    pub fn get_tile(&self, identifier: &str) -> Option<LdtkTile> {
        self.field_instances
            .get(identifier)
            .and_then(|tile| tile.get_tile())
            .map(LdtkTile::new)
    }
}

pub struct FieldInstancesPlugin;
impl Plugin for FieldInstancesPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LdtkFieldInstances>();
    }
}
