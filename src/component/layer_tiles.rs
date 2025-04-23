use bevy_app::Plugin;
use bevy_asset::RenderAssetUsages;
use bevy_asset::prelude::AssetChanged;
use bevy_asset::{AsAssetId, Assets, Handle};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::{Commands, Query, ResMut};
use bevy_image::Image;
use bevy_ldtk_asset::layer::LayerInstance;
use bevy_ldtk_asset::layer::TilesLayer;
use bevy_ldtk_asset::tile_instance::TileInstance;
use bevy_log::{debug, error};
use bevy_math::{I64Vec2, UVec2};
use bevy_reflect::Reflect;
use bevy_sprite::{Anchor, Sprite};
use image::imageops::{crop_imm, flip_horizontal, flip_vertical, overlay};
use image::{DynamicImage, RgbaImage};

use super::shieldtank_component::ShieldtankComponentSystemSet;

#[derive(Debug, Reflect)]
pub struct LayerTile {
    pub opacity: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub offset: I64Vec2,
    pub source: UVec2,
    pub size: UVec2,
}

impl LayerTile {
    pub(crate) fn new(tile_instance: &TileInstance, size: UVec2) -> Self {
        let opacity = tile_instance.opacity;
        let flip_x = tile_instance.flip_x;
        let flip_y = tile_instance.flip_y;
        let offset = tile_instance.offset;
        let source = tile_instance.source.as_uvec2();

        Self {
            opacity,
            flip_x,
            flip_y,
            offset,
            source,
            size,
        }
    }
}

#[derive(Debug, Component, Reflect)]
pub struct LayerTiles {
    pub tiles: Vec<LayerTile>,
    pub image: Handle<Image>,
    pub grid_cell_size: u32,
    pub size: UVec2,
    pub opacity: f32,
}

impl AsAssetId for LayerTiles {
    type Asset = Image;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.image.id()
    }
}

impl LayerTiles {
    pub(crate) fn new(layer_asset: &LayerInstance, tiles_layer: &TilesLayer) -> Self {
        let tile_size = UVec2::splat(layer_asset.grid_cell_size as u32);
        let tiles = tiles_layer
            .tiles
            .iter()
            .map(|tile| LayerTile::new(tile, tile_size))
            .collect();
        let image = tiles_layer.tileset_image.clone();
        let grid_cell_size = layer_asset.grid_cell_size as u32;
        let size = (layer_asset.grid_cell_size * layer_asset.grid_size).as_uvec2();
        let opacity = layer_asset.opacity as f32;

        Self {
            tiles,
            image,
            grid_cell_size,
            size,
            opacity,
        }
    }

    pub(crate) fn generate_sprite_image(
        &self,
        tileset_image: Image,
    ) -> bevy_ecs::error::Result<Image> {
        let mut new_image = RgbaImage::new(self.size.x, self.size.y);

        let tileset_image = tileset_image.try_into_dynamic()?.to_rgba8();

        self.tiles.iter().for_each(|tile| {
            let mut tile_image = crop_imm(
                &tileset_image,
                tile.source.x,
                tile.source.y,
                tile.size.x,
                tile.size.y,
            )
            .to_image();

            if tile.flip_x {
                tile_image = flip_horizontal(&tile_image);
            }

            if tile.flip_y {
                tile_image = flip_vertical(&tile_image);
            }

            // Opacity from the tile itself
            tile_image.enumerate_pixels_mut().for_each(|(_, _, pixel)| {
                // TODO: Should we do some bounds checking here?
                let pixel_opacity = pixel[3] as u16;
                let tile_opacity = (tile.opacity * 255.0) as u16;
                let new_opacity = ((pixel_opacity * tile_opacity) / 255) as u8;

                pixel[3] = new_opacity;
            });

            overlay(&mut new_image, &tile_image, tile.offset.x, tile.offset.y);
        });

        // Overall opacity of the layer
        new_image.enumerate_pixels_mut().for_each(|(_, _, pixel)| {
            // TODO: Should we do some bounds checking here?
            let pixel_opacity = pixel[3] as u16;
            let layer_opacity = (self.opacity * 255.0) as u16;
            let new_opacity = ((pixel_opacity * layer_opacity) / 255) as u8;

            pixel[3] = new_opacity;
        });

        let new_image = DynamicImage::from(new_image);

        let new_image = Image::from_dynamic(new_image, true, RenderAssetUsages::default());

        Ok(new_image)
    }
}

#[allow(clippy::type_complexity)]
fn layer_tile_system(
    query: Query<(Entity, &LayerTiles), Or<(Changed<LayerTiles>, AssetChanged<LayerTiles>)>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) -> bevy_ecs::error::Result<()> {
    query
        .iter()
        .try_for_each(|(entity, component)| -> bevy_ecs::error::Result<()> {
            let Some(image) = images.get(component.as_asset_id()).cloned() else {
                error!("Bad layer image handle! {entity:?} {:?}", component.image);
                return Ok(());
            };

            let image = component.generate_sprite_image(image)?;
            let image = images.add(image);
            let anchor = Anchor::TopLeft;
            let sprite = Sprite {
                image,
                anchor,
                ..Default::default()
            };

            debug!("Processing LayerTiles for {entity:?}");

            commands.entity(entity).insert(sprite);

            Ok(())
        })
}

pub struct LayerTilePlugin;
impl Plugin for LayerTilePlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LayerTile>();
        app.register_type::<LayerTiles>();
        app.add_systems(ShieldtankComponentSystemSet, layer_tile_system);
    }
}
