use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_math::Vec2;
use bevy_sprite::Sprite;

use crate::commands::ShieldtankItemCommands;
use crate::component::entity::EntityComponentQueryData;

pub type EntityCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, EntityAsset, EntityComponentQueryData<'w>>;

impl EntityCommands<'_, '_> {
    pub fn flip_x(&mut self, flip: bool) -> &mut Self {
        let mut sprite = if let Some(sprite) = self.item.get_sprite() {
            (*sprite).clone()
        } else {
            Sprite::default()
        };

        sprite.flip_x = flip;

        self.insert(sprite)
    }

    pub fn flip_y(&mut self, flip: bool) -> &mut Self {
        let mut sprite = if let Some(sprite) = self.item.get_sprite() {
            (*sprite).clone()
        } else {
            Sprite::default()
        };

        sprite.flip_y = flip;

        self.insert(sprite)
    }

    pub fn set_world_location(&mut self, world_location: Vec2) -> &mut Self {
        let self_world_location = self.item.world_location();

        let offset = world_location - self_world_location;

        self.set_location(self.item.location() + offset)
    }
}
