use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::commands::entity::LdtkEntityCommands;
use crate::system_params::entity::item::LdtkEntity;

#[derive(SystemParam)]
pub struct LdtkCommands<'w, 's> {
    commands: Commands<'w, 's>,
}

impl<'a> LdtkCommands<'_, '_> {
    pub fn ldtk_entity(
        &'a mut self,
        ldtk_entity: &'a LdtkEntity<'a, 'a>,
    ) -> LdtkEntityCommands<'a, 'a> {
        LdtkEntityCommands {
            commands: self.commands.reborrow(),
            item: ldtk_entity,
        }
    }
}
