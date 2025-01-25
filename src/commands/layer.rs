use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_sprite::Sprite;

use crate::commands::ShieldtankItemCommands;
use crate::component::layer::LayerComponentQueryData;
use crate::int_grid::IntGrid;
use crate::tiles::Tiles;

pub type LayerCommands<'w, 's> =
    ShieldtankItemCommands<'w, 's, LayerAsset, LayerComponentQueryData<'w>>;

impl LayerCommands<'_, '_> {
    pub fn insert_tiles(&mut self, tiles: Tiles) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(tiles);

        self
    }

    pub fn insert_int_grid(&mut self, int_grid: IntGrid) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(int_grid);

        self
    }

    pub fn insert_sprite(&mut self, sprite: Sprite) -> &mut Self {
        self.commands
            .entity(self.item.get_ecs_entity())
            .insert(sprite);

        self
    }
}
