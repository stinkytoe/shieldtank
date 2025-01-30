use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_math::Vec2;

use crate::commands::ShieldtankItemCommands;
use crate::component::entity::EntityComponentQueryData;

pub type EntityCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, EntityAsset, EntityComponentQueryData<'w>>;

impl EntityCommands<'_, '_> {
    pub fn flip_x(&mut self, flip: bool) -> &mut Self {
        // TODO: This is a no-op if we don't contain a Sprite component. Is this what we intend?
        if let Some(mut sprite) = self.item.get_sprite().as_deref().cloned() {
            sprite.flip_x = flip;
            self.insert(sprite);
        }

        self
    }

    pub fn flip_y(&mut self, flip: bool) -> &mut Self {
        // TODO: This is a no-op if we don't contain a Sprite component. Is this what we intend?
        if let Some(mut sprite) = self.item.get_sprite().as_deref().cloned() {
            sprite.flip_y = flip;
            self.insert(sprite);
        }

        self
    }

    pub fn set_world_location(&mut self, world_location: Vec2) -> &mut Self {
        let self_world_location = self.item.world_location();

        let offset = world_location - self_world_location;

        self.set_location(self.item.location() + offset)
    }
}
