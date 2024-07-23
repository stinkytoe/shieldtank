use bevy::prelude::*;

use crate::reexports::tileset_rectangle::TilesetRectangle;
use crate::system_params::entity::item::LdtkEntity;
use crate::system_params::traits::LdtkItem;

pub struct LdtkEntityCommands<'w, 's> {
    pub(crate) commands: Commands<'w, 's>,
    pub(crate) item: &'w LdtkEntity<'w, 's>,
}

impl LdtkEntityCommands<'_, '_> {
    pub fn set_tile(&mut self, tile: TilesetRectangle) {
        self.commands.entity(self.item.ecs_entity()).insert(tile);
    }
}
