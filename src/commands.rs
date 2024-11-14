use bevy_ecs::system::{Commands, SystemParam};

use crate::entity::EntityItem;
use crate::entity_commands::EntityCommands;
use crate::layer::LayerItem;
use crate::layer_commands::LayerCommands;

#[derive(SystemParam)]
pub struct LdtkCommands<'w, 's> {
    commands: Commands<'w, 's>,
}

impl LdtkCommands<'_, '_> {
    pub fn layer<'a>(&'a mut self, layer_item: &'a LayerItem) -> LayerCommands<'a> {
        LayerCommands {
            commands: self.commands.reborrow(),
            item: layer_item,
        }
    }

    pub fn entity<'a>(&'a mut self, entity_item: &'a EntityItem) -> EntityCommands<'a> {
        EntityCommands {
            commands: self.commands.reborrow(),
            item: entity_item,
        }
    }
}
