use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_sprite::Sprite;

use crate::{component::entity::EntityComponentQueryData, tileset_rectangle::TilesetRectangle};

use super::ShieldtankItemCommands;

pub type EntityCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, EntityAsset, EntityComponentQueryData<'w>>;

impl EntityCommands<'_, '_> {
    pub fn insert_tile(&mut self, tile: TilesetRectangle) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(tile);

        self
    }

    pub fn insert_sprite(&mut self, sprite: Sprite) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(sprite);

        self
    }

    pub fn flip_x(&mut self, flip: bool) -> &mut Self {
        // TODO: This is a no-op if we don't contain a Sprite component. Is this what we intend?
        if let Some(mut sprite) = self.item.get_sprite().as_deref().cloned() {
            sprite.flip_x = flip;
            self.insert_sprite(sprite);
        }

        self
    }

    pub fn flip_y(&mut self, flip: bool) -> &mut Self {
        // TODO: This is a no-op if we don't contain a Sprite component. Is this what we intend?
        if let Some(mut sprite) = self.item.get_sprite().as_deref().cloned() {
            sprite.flip_y = flip;
            self.insert_sprite(sprite);
        }

        self
    }
}
