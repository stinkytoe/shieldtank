use bevy_app::{Plugin, PostUpdate};
use bevy_ecs::change_detection::DetectChanges as _;
use bevy_ecs::{component::Component, system::Commands};
use bevy_ldtk_asset::tileset_rectangle::TilesetRectangle as LdtkTilesetRectangle;
use bevy_log::error;
use bevy_math::Rect;
use bevy_reflect::Reflect;
use bevy_sprite::Sprite;

use crate::error::Result;
use crate::query::ShieldtankQuery;
use crate::shieldtank_error;

#[derive(Clone, Debug, Component, Reflect)]
pub struct TilesetRectangle {
    pub tile: LdtkTilesetRectangle,
}

impl TilesetRectangle {
    pub fn new(tile: LdtkTilesetRectangle) -> Self {
        Self { tile }
    }
}

pub(crate) fn tileset_rectangle_system(mut commands: Commands, shieldtank_query: ShieldtankQuery) {
    shieldtank_query
        .iter_entities()
        .filter(|item| {
            item.get_tileset_rectangle()
                .as_ref()
                .and_then(|tileset_rectangle| tileset_rectangle.is_changed().then_some(()))
                .is_some()
        })
        // .inspect(|item| debug!("Tileset rectangle loaded for: {}", item.get_identifier()))
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
            let anchor = item.get_asset().anchor;

            let mut sprite = if let Some(sprite) = item.get_sprite() {
                (*sprite).clone()
            } else {
                Sprite::default()
            };

            sprite.image = image;
            sprite.custom_size = custom_size;
            sprite.rect = rect;
            sprite.anchor = anchor;

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
