use bevy_ldtk_asset::entity::Entity as EntityAsset;

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
}
