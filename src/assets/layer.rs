use bevy::asset::LoadContext;
use bevy::asset::LoadDirectError;
use bevy::math::I64Vec2;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::Mesh2dHandle;
use bevy::tasks::block_on;
use image::imageops::flip_horizontal;
use image::imageops::flip_vertical;
use image::imageops::overlay;
use image::ColorType;
use image::DynamicImage;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

use crate::assets::event::LdkAssetEvent;
use crate::assets::traits::LdtkAsset;
use crate::iid::Iid;
use crate::iid::IidError;
use crate::iid::IidSet;
use crate::ldtk;
use crate::reexports::tile_instance::TileInstance;
use crate::util::ldtk_path_to_bevy_path;

#[derive(Debug, Error)]
pub enum LdtkLayerAssetError {
    #[error(transparent)]
    IidError(#[from] IidError),
    #[error(transparent)]
    LoadDirectError(#[from] LoadDirectError),
    #[error(transparent)]
    IntoDynamicImageError(#[from] IntoDynamicImageError),
    #[error("unknown layer type! {0}")]
    UnknownLayerType(String),
    #[error("tile instances in entity type layer! {0}")]
    EntityLayerWithTiles(Iid),
    #[error("Int Grid/Auto Layer should only have auto tiles!")]
    IntGridWithEntitiesOrGridTiles(Iid),
    #[error("Tiles Layer should only have grid tiles!")]
    TilesWithAutoLayerOrEntities(Iid),
    #[error("bad handle? {0:?}")]
    BadHandle(Handle<LdtkLayerAsset>),
}

#[derive(Clone, Copy, Debug, Reflect)]
pub enum LdtkLayerType {
    IntGrid,
    Entities,
    Tiles,
    Autolayer,
}

#[allow(clippy::result_large_err)]
impl LdtkLayerType {
    pub fn new(ldtk_type: &str) -> Result<LdtkLayerType, LdtkLayerAssetError> {
        Ok(match ldtk_type {
            "IntGrid" => LdtkLayerType::IntGrid,
            "Entities" => LdtkLayerType::Entities,
            "Tiles" => LdtkLayerType::Tiles,
            "AutoLayer" => LdtkLayerType::Autolayer,
            _ => return Err(LdtkLayerAssetError::UnknownLayerType(ldtk_type.to_string())),
        })
    }
}

#[derive(Asset, Debug, Reflect)]
pub struct LdtkLayerAsset {
    // NOTE: Internal fields
    pub(crate) iid: Iid,
    pub(crate) children: IidSet,
    pub(crate) identifier: String,
    pub(crate) index: usize,
    pub(crate) layer_separation: f32,
    pub(crate) layer_image: Option<(Mesh2dHandle, Handle<ColorMaterial>)>,
    // NOTE: LDtk exports
    pub(crate) grid_dimensions: I64Vec2,
    pub(crate) grid_size: i64,
    pub(crate) px_total_offset: I64Vec2,
    pub(crate) tileset_def_uid: Option<i64>,
    pub(crate) layer_type: LdtkLayerType,
    #[reflect(ignore)]
    pub(crate) _int_grid_csv: Vec<i64>,
    pub(crate) layer_def_uid: i64,
    pub(crate) level_id: i64,
    pub(crate) override_tileset_uid: Option<i64>,
    pub(crate) location: I64Vec2,
}

impl LdtkLayerAsset {
    #[allow(clippy::result_large_err)]
    pub(crate) fn new(
        value: &ldtk::LayerInstance,
        index: usize,
        layer_separation: f32,
        load_context: &mut LoadContext,
        base_directory: &Path,
    ) -> Result<Self, LdtkLayerAssetError> {
        let layer_type = LdtkLayerType::new(&value.layer_instance_type)?;

        let iid = Iid::from_str(&value.iid)?;

        let (children, tiles): (IidSet, Vec<TileInstance>) = match (
            layer_type,
            value.auto_layer_tiles.len(),
            value.grid_tiles.len(),
            value.entity_instances.len(),
        ) {
            (LdtkLayerType::IntGrid | LdtkLayerType::Autolayer, _, 0, 0) => (
                IidSet::default(),
                value
                    .auto_layer_tiles
                    .iter()
                    .map(TileInstance::new)
                    .collect(),
            ),
            (LdtkLayerType::Tiles, 0, _, 0) => (
                IidSet::default(),
                value.grid_tiles.iter().map(TileInstance::new).collect(),
            ),
            (LdtkLayerType::Entities, 0, 0, _) => (
                value
                    .entity_instances
                    .iter()
                    .map(|child| Iid::from_str(&child.iid))
                    .collect::<Result<_, _>>()?,
                Vec::default(),
            ),
            (LdtkLayerType::IntGrid | LdtkLayerType::Autolayer, _, _, _) => {
                return Err(LdtkLayerAssetError::IntGridWithEntitiesOrGridTiles(iid))
            }
            (LdtkLayerType::Tiles, _, _, _) => {
                return Err(LdtkLayerAssetError::TilesWithAutoLayerOrEntities(iid))
            }
            (LdtkLayerType::Entities, _, _, _) => {
                return Err(LdtkLayerAssetError::EntityLayerWithTiles(iid))
            }
        };

        let grid_dimensions = (value.c_wid, value.c_hei).into();
        let grid_size = value.grid_size;
        let opacity = value.opacity;
        let tileset_rel_path = value.tileset_rel_path.clone();

        Ok(Self {
            iid,
            children,
            identifier: value.identifier.clone(),
            index,
            layer_separation,
            layer_image: Self::create_layer_image(
                grid_dimensions,
                grid_size,
                opacity,
                tiles,
                tileset_rel_path,
                load_context,
                base_directory,
                iid,
            )?,
            grid_dimensions,
            grid_size,
            px_total_offset: (value.px_total_offset_x, -value.px_total_offset_y).into(),
            tileset_def_uid: value.tileset_def_uid,
            layer_type,
            _int_grid_csv: value.int_grid_csv.clone(),
            layer_def_uid: value.layer_def_uid,
            level_id: value.level_id,
            override_tileset_uid: value.override_tileset_uid,
            location: (value.px_offset_x, value.px_offset_y).into(),
        })
    }

    #[allow(clippy::result_large_err)]
    pub(crate) fn layer_image_system(
        mut commands: Commands,
        mut events: EventReader<LdkAssetEvent<LdtkLayerAsset>>,
        layer_assets: Res<Assets<LdtkLayerAsset>>,
    ) -> Result<(), LdtkLayerAssetError> {
        for LdkAssetEvent::<LdtkLayerAsset>::Modified { entity, handle } in events.read() {
            let layer_asset = layer_assets
                .get(handle)
                .ok_or(LdtkLayerAssetError::BadHandle(handle.clone()))?;

            if let Some((mesh, material)) = &layer_asset.layer_image {
                commands
                    .entity(*entity)
                    .insert((mesh.clone(), material.clone()));
            } else {
                commands
                    .entity(*entity)
                    .remove::<Handle<Mesh>>()
                    .remove::<Handle<ColorMaterial>>();
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    #[allow(clippy::result_large_err)]
    fn create_layer_image(
        grid_size: I64Vec2,
        grid_cell_size: i64,
        opacity: f64,
        tiles: Vec<TileInstance>,
        tileset_rel_path: Option<String>,
        load_context: &mut LoadContext,
        base_directory: &Path,
        iid: Iid,
    ) -> Result<Option<(Mesh2dHandle, Handle<ColorMaterial>)>, LdtkLayerAssetError> {
        Ok(match tileset_rel_path {
            Some(tileset_rel_path) => {
                let tileset: Image = block_on(async {
                    load_context
                        .loader()
                        .direct()
                        .load(ldtk_path_to_bevy_path(
                            base_directory,
                            Path::new(&tileset_rel_path),
                        ))
                        .await
                })?
                .take();

                let canvas_size = grid_size * grid_cell_size;

                let mesh = Mesh2dHandle(load_context.add_labeled_asset(
                    format!("{iid}%mesh"),
                    Self::create_tile_layer_mesh(canvas_size.as_vec2()),
                ));

                let image = Self::build_image_from_tiles(
                    &tileset,
                    canvas_size.as_uvec2(),
                    UVec2::splat(grid_cell_size as u32),
                    &tiles,
                )?;

                let color = Color::srgba(1.0, 1.0, 1.0, opacity as f32);

                let texture_handle =
                    load_context.add_labeled_asset(format!("{iid}%texture"), image);

                let texture = Some(texture_handle);

                let material = load_context
                    .add_labeled_asset(format!("{iid}%material"), ColorMaterial { color, texture });

                Some((mesh, material))
            }
            None => None,
        })
    }

    #[allow(clippy::result_large_err)]
    pub(crate) fn build_image_from_tiles(
        tileset: &Image,
        canvas_size: UVec2,
        tile_size: UVec2,
        tiles: &[TileInstance],
    ) -> Result<Image, LdtkLayerAssetError> {
        let tileset = tileset.clone().try_into_dynamic()?;

        let mut dynamic_image = DynamicImage::new(canvas_size.x, canvas_size.y, ColorType::Rgba8);

        tiles.iter().for_each(|tile| {
            let mut cropped =
                tileset.crop_imm(tile.source.x, tile.source.y, tile_size.x, tile_size.y);

            if tile.flip_h {
                cropped = image::DynamicImage::ImageRgba8(flip_horizontal(&cropped));
            }

            if tile.flip_v {
                cropped = image::DynamicImage::ImageRgba8(flip_vertical(&cropped));
            }

            overlay(
                &mut dynamic_image,
                &cropped,
                tile.location.x,
                tile.location.y,
            );
        });

        Ok(Image::from_dynamic(
            dynamic_image,
            true,
            RenderAssetUsages::default(),
        ))
    }

    pub(crate) fn create_tile_layer_mesh(size: Vec2) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]))
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [0.0, 0.0, 0.0],
                [size.x, 0.0, 0.0],
                [size.x, -size.y, 0.0],
                [0.0, -size.y, 0.0],
            ],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
        )
    }
}

impl LdtkAsset for LdtkLayerAsset {
    fn iid(&self) -> crate::iid::Iid {
        self.iid
    }

    fn children(&self) -> &IidSet {
        &self.children
    }

    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn location(&self) -> Vec3 {
        Vec3::new(
            self.location.x as f32,
            -self.location.y as f32,
            (self.index + 1) as f32 * self.layer_separation,
        )
    }

    fn asset_handle_from_project(
        project: &crate::prelude::LdtkProject,
        iid: Iid,
    ) -> Option<Handle<Self>> {
        project.layers.get(&iid).cloned()
    }
}
