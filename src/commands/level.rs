use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_sprite::Sprite;

use crate::{component::level::LevelComponentQueryData, level_background::LevelBackground};

use super::ShieldtankItemCommands;

pub type LevelCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, LevelAsset, LevelComponentQueryData<'w>>;

impl LevelCommands<'_, '_> {
    pub fn insert_level_background(&mut self, level_background: LevelBackground) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(level_background);

        self
    }

    pub fn insert_sprite(&mut self, sprite: Sprite) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(sprite);

        self
    }
}
