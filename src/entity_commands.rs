use bevy_ldtk_asset::entity::Entity as EntityAsset;

use crate::item::LdtkItemTrait;
use crate::item_commands::LdtkItemCommands;

pub type EntityCommands<'a> = LdtkItemCommands<'a, EntityAsset>;

impl EntityCommands<'_> {
    pub fn set_tile_to_field_instance(&mut self, identifier: &str) {
        let ecs_entity = self.item.get_ecs_entity();
        if let Some(tile) = self.item.get_field_tile(identifier) {
            self.commands.entity(ecs_entity).insert(tile);
        }
    }
}
