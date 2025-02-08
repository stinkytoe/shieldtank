use bevy_app::{Plugin, PostUpdate};
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::component::Component;
use bevy_ecs::system::{Commands, ResMut};
use bevy_image::Image;
use bevy_ldtk_asset::layer::{Layer as LayerAsset, TilesLayer};
use bevy_ldtk_asset::tile_instance::TileInstance;
use bevy_log::{debug, error};
use bevy_reflect::Reflect;
use bevy_sprite::{Anchor, Sprite};

use crate::error::Result;
use crate::item::layer::iter::LayerItemIteratorExt;
use crate::query::ShieldtankQuery;
use crate::shieldtank_error;

#[derive(Component, Debug, Reflect)]
pub struct Tiles {
    tiles: Vec<TileInstance>,
}

impl Tiles {
    pub(crate) fn new(tiles_layer: &TilesLayer) -> Self {
        Self {
            tiles: tiles_layer.tiles.clone(),
        }
    }

    pub(crate) fn generate_layer_image(
        &self,
        image_assets: &mut Assets<Image>,
        layer_instance: &LayerAsset,
    ) -> Result<Handle<Image>> {
        let tiles_layer = layer_instance
            .layer_type
            .get_tiles_layer()
            .ok_or(shieldtank_error!("bad tiles layer!"))?;

        let size = (layer_instance.grid_size * layer_instance.grid_cell_size).as_uvec2();

        let mut layer_image = image::RgbaImage::new(size.x, size.y);

        let tileset_image = image_assets
            .get(tiles_layer.tileset_image.id())
            .ok_or(shieldtank_error!(
                "bad handle! {:?}",
                tiles_layer.tileset_image
            ))?
            .clone() // TODO: Can we get rid of this clone somehow?
            .try_into_dynamic()?
            .to_rgba8();

        self.tiles.iter().for_each(|tile| {
            let corner = tile.source.as_uvec2();
            let size = layer_instance.grid_size.as_uvec2();
            let mut tile_image =
                image::imageops::crop_imm(&tileset_image, corner.x, corner.y, size.x, size.y)
                    .to_image();

            if tile.flip_x {
                tile_image = image::imageops::flip_horizontal(&tile_image);
            }

            if tile.flip_y {
                tile_image = image::imageops::flip_vertical(&tile_image);
            }

            let opacity = ((layer_instance.opacity as f32) * tile.opacity * 255.0) as u16;
            tile_image.enumerate_pixels_mut().for_each(|(_, _, pixel)| {
                let register: u16 = (opacity * (pixel[3] as u16)) / 255;

                pixel[3] = register as u8;
            });

            let location = tile.offset;
            image::imageops::overlay(&mut layer_image, &tile_image, location.x, location.y);
        });

        let layer_image = Image::from_dynamic(
            image::DynamicImage::from(layer_image),
            true,
            RenderAssetUsages::default(),
        );

        Ok(image_assets.add(layer_image))
    }
}

pub(crate) fn tiles_system(
    mut commands: Commands,
    mut image_assets: ResMut<Assets<Image>>,
    shieldtank_query: ShieldtankQuery,
) {
    shieldtank_query
        .iter_layers()
        .filter_tiles_layer()
        .filter(|item| {
            item.get_tiles()
                .as_ref()
                .and_then(|tiles| tiles.is_changed().then_some(()))
                .is_some()
        })
        .inspect(|item| debug!("Tiles loaded for: {}", item.get_identifier()))
        .map(|item| -> Result<()> {
            let Some(tiles) = item.get_tiles() else {
                return Ok(());
            };

            let asset = item.get_asset();
            let image = tiles.generate_layer_image(&mut image_assets, asset)?;
            let anchor = Anchor::TopLeft;
            let sprite = Sprite {
                image,
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
                error!("failed to load tiles: {e}");
            }
        });
}

pub struct TilesPlugin;
impl Plugin for TilesPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<Tiles>()
            .add_systems(PostUpdate, tiles_system);
    }
}
