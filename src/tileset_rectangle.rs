use bevy_app::{Plugin, PostUpdate};
use bevy_ecs::{component::Component, system::Commands};
use bevy_ldtk_asset::tileset_rectangle::TilesetRectangle as LdtkTilesetRectangle;
use bevy_log::error;
use bevy_math::Rect;
use bevy_reflect::Reflect;
use bevy_sprite::{Anchor, Sprite};

use crate::error::Result;
use crate::item::entity::iter::EntityItemIteratorExt as _;
use crate::query::ShieldtankQuery;
use crate::shieldtank_error;

#[derive(Clone, Debug, Component, Reflect)]
pub struct TilesetRectangle {
    pub anchor: Anchor,
    pub tile: LdtkTilesetRectangle,
}

impl TilesetRectangle {
    pub fn new(anchor: Anchor, tile: LdtkTilesetRectangle) -> Self {
        Self { anchor, tile }
    }
}

pub(crate) fn tileset_rectangle_system(mut commands: Commands, shieldtank_query: ShieldtankQuery) {
    shieldtank_query
        .iter_entities()
        .filter_tileset_rectangle_changed()
        .map(|item| -> Result<()> {
            let Some(tile) = item.get_tileset_rectangle() else {
                return Ok(());
            };

            let size = tile.tile.size.as_vec2();

            let id = tile.tile.tileset_definition.id();
            let tileset_definition = shieldtank_query
                .get_tileset_definition(id)
                .ok_or(shieldtank_error!("bad tileset definition!"))?;
            let Some(image) = tileset_definition.tileset_image.as_ref().cloned() else {
                return Ok(());
            };

            let custom_size = Some(size);

            let corner = tile.tile.corner.as_vec2();

            let rect = Some(Rect::from_corners(corner, corner + size));
            let anchor = tile.anchor;

            let sprite = Sprite {
                image,
                custom_size,
                rect,
                anchor,
                ..Default::default()
            };

            commands.entity(item.get_ecs_entity()).insert(sprite);

            Ok(())
        })
        .for_each(|ret| {
            // TODO: We're just printing the error and moving on to the next layer.
            // Should we do something else?
            if let Err(e) = ret {
                error!("failed to entity sprite: {e}");
            }
        });
}

pub struct TilesetRectanglePlugin;
impl Plugin for TilesetRectanglePlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<TilesetRectangle>()
            .add_systems(PostUpdate, tileset_rectangle_system);
    }
}
