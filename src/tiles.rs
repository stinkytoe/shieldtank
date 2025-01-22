use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_ecs::component::Component;
use bevy_image::Image;
use bevy_ldtk_asset::layer::{Layer as LayerAsset, TilesLayer};
use bevy_ldtk_asset::tile_instance::TileInstance;
use bevy_reflect::Reflect;

use crate::error::Result;
use crate::shieldtank_error;

#[derive(Component, Debug, Reflect)]
pub struct Tiles {
    tiles: Vec<TileInstance>,
}

impl Tiles {
    pub(crate) fn _new(tiles_layer: &TilesLayer) -> Self {
        Self {
            tiles: tiles_layer.tiles.clone(),
        }
    }

    pub(crate) fn _generate_layer_image(
        &self,
        image_assets: &mut Assets<Image>,
        layer_instance: &LayerAsset,
    ) -> Result<Handle<Image>> {
        let tiles_layer = layer_instance
            .layer_type
            .get_tiles_layer()
            .ok_or(shieldtank_error!("Bad Tiles Layer!"))?;

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
                tile_image = image::imageops::flip_vertical(&tile_image);
            }

            if tile.flip_y {
                tile_image = image::imageops::flip_horizontal(&tile_image);
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

// pub(crate) fn handle_tiles_system(
//     mut commands: Commands,
//     assets: Res<Assets<LayerAsset>>,
//     mut image_assets: ResMut<Assets<Image>>,
//     query: Query<(Entity, &LayerComponent, &Tiles), Changed<Tiles>>,
// ) -> Result<()> {
//     query
//         .iter()
//         .try_for_each(|(entity, layer, tiles)| -> Result<()> {
//             let asset = assets
//                 .get(layer.handle.id())
//                 .ok_or(bad_handle!("bad handle! {:?}", layer.handle))?;
//             let image = tiles.generate_layer_image(&mut image_assets, asset)?;
//
//             commands.entity(entity).insert(Sprite {
//                 image,
//                 anchor: Anchor::TopLeft,
//                 //color: bevy::color::Color::srgba_u8(255, 255, 255, 64),
//                 ..Default::default()
//             });
//
//             trace!("Tiles layer generated!");
//
//             Ok(())
//         })
// }
//
// pub struct TilesPlugin;
// impl Plugin for TilesPlugin {
//     fn build(&self, app: &mut bevy_app::App) {
//         app.add_systems(Update, handle_tiles_system.map(error));
//     }
// }
