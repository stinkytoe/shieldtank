use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use thiserror::Error;

use crate::reexports::tileset_rectangle::TilesetRectangle;
use crate::system_params::entity::item::LdtkEntity;

#[derive(Debug, Error)]
pub enum LdtkEntityCommandsError {}

#[derive(SystemParam)]
pub struct LdtkEntityCommands<'w, 's> {
    commands: Commands<'w, 's>,
}

impl<'w, 's> LdtkEntityCommands<'w, 's> {
    pub fn set_tile(&mut self, ldtk_entity: &LdtkEntity, tile: &TilesetRectangle) {
        self.commands
            .entity(ldtk_entity.entity)
            .insert(tile.clone());
    }
}
