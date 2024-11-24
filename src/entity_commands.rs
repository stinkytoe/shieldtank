use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_math::Vec2;

use crate::item::LdtkItemTrait;
use crate::item_commands::LdtkItemCommands;

pub type EntityCommands<'a> = LdtkItemCommands<'a, EntityAsset>;

impl EntityCommands<'_> {
    pub fn set_global_location(&mut self, global_location: Vec2) -> &mut Self {
        let ecs_entity = self.item.get_ecs_entity();
        if let (Some(current_global_location), Ok(current_transform)) = (
            self.item.get_global_location(),
            self.item.query.transform_query.get(ecs_entity),
        ) {
            let offset = global_location - current_global_location;
            let new_transform = current_transform
                .with_translation(current_transform.translation + offset.extend(0.0));

            self.commands.entity(ecs_entity).insert(new_transform);
        }

        self
    }

    pub fn set_tile_to_field_instance(&mut self, identifier: &str) -> &mut Self {
        let ecs_entity = self.item.get_ecs_entity();
        if let Some(tile) = self.item.get_field_tile(identifier) {
            self.commands.entity(ecs_entity).insert(tile);
        }

        self
    }

    pub fn set_tile_to_field_instance_array_index(
        &mut self,
        identifier: &str,
        index: usize,
    ) -> &mut Self {
        let ecs_entity = self.item.get_ecs_entity();
        if let Some(tiles) = self.item.get_field_array_tiles(identifier) {
            if let Some(tile) = tiles.get(index) {
                self.commands.entity(ecs_entity).insert(tile.clone());
            }
        }

        self
    }

    pub fn set_sprite_flip_x(&mut self, flip_x: bool) -> &mut Self {
        let ecs_entity = self.item.get_ecs_entity();
        if let Ok(sprite) = self.item.query.sprite_query.get(ecs_entity) {
            let mut sprite = sprite.clone();
            sprite.flip_x = flip_x;
            self.commands.entity(ecs_entity).insert(sprite);
        }

        self
    }

    pub fn set_sprite_flip_y(&mut self, flip_y: bool) -> &mut Self {
        let ecs_entity = self.item.get_ecs_entity();
        if let Ok(sprite) = self.item.query.sprite_query.get(ecs_entity) {
            let mut sprite = sprite.clone();
            sprite.flip_y = flip_y;
            self.commands.entity(ecs_entity).insert(sprite);
        }

        self
    }
}
