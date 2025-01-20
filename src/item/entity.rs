use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_sprite::Sprite;

use crate::component::entity::EntityComponentQueryData;
use crate::tileset_rectangle::TilesetRectangle;

use super::Item;

pub type EntityItem<'w, 's> = Item<'w, 's, EntityAsset, EntityComponentQueryData<'w>>;

impl EntityItem<'_, '_> {
    pub fn get_tileset_rectangle(&self) -> Option<&TilesetRectangle> {
        self.component_query_data.0.as_deref()
    }

    pub fn get_sprite(&self) -> Option<&Sprite> {
        self.component_query_data.1.as_deref()
    }
}
