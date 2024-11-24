use bevy_app::Plugin;
use bevy_app::Update;
use bevy_asset::Assets;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::Changed;
use bevy_ecs::system::Commands;
use bevy_ecs::system::IntoSystem;
use bevy_ecs::system::Query;
use bevy_ecs::system::Res;
use bevy_ldtk_asset::tileset_definition::TilesetDefinition;
use bevy_ldtk_asset::tileset_rectangle::TilesetRectangle as LdtkTilesetRectangle;
use bevy_log::trace;
use bevy_math::Rect;
use bevy_reflect::Reflect;
use bevy_sprite::{Anchor, Sprite};
use bevy_utils::error;

use crate::{bad_handle, Result};

#[derive(Clone, Component, Debug, Reflect)]
pub struct TilesetRectangle {
    pub anchor: Anchor,
    pub tile: LdtkTilesetRectangle,
}

pub(crate) fn handle_tileset_rectangle_system(
    mut commands: Commands,
    tileset_definitions: Res<Assets<TilesetDefinition>>,
    mut changed_query: Query<
        (Entity, &TilesetRectangle, Option<&mut Sprite>),
        Changed<TilesetRectangle>,
    >,
) -> Result<()> {
    changed_query.iter_mut().try_for_each(
        |(entity, TilesetRectangle { anchor, tile }, sprite)| -> Result<()> {
            let tileset_definition = tileset_definitions
                .get(tile.tileset_definition.id())
                .ok_or(bad_handle!("bad handle! {:?}", tile.tileset_definition))?;

            let Some(image) = tileset_definition.tileset_image.clone() else {
                // just pretend nothing happened...
                return Ok(());
            };

            let anchor = *anchor;
            let custom_size = Some(tile.size.as_vec2());

            let corner = tile.corner.as_vec2();
            let size = tile.size.as_vec2();

            let rect = Some(Rect::from_corners(corner, corner + size));

            if let Some(mut sprite) = sprite {
                sprite.image = image;
                sprite.custom_size = custom_size;
                sprite.rect = rect;
                sprite.anchor = anchor;
            } else {
                commands.entity(entity).insert(Sprite {
                    image,
                    custom_size,
                    rect,
                    anchor,
                    ..Default::default()
                });
            }

            trace!("Tileset rectangle added!");
            Ok(())
        },
    )
}

pub struct TilesetRectangleSystem;
impl Plugin for TilesetRectangleSystem {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(Update, handle_tileset_rectangle_system.map(error));
    }
}
