use bevy::asset::LoadContext;
use bevy::asset::LoadDirectError;
use bevy::math::I64Vec2;
use bevy::math::I64Vec3;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::Extent3d;
use bevy::render::render_resource::TextureDimension;
use bevy::render::render_resource::TextureFormat;
use bevy::sprite::Anchor;
use bevy::tasks::block_on;
use image::imageops::overlay;
use image::imageops::FilterType;
use image::RgbaImage;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::Iid;
use crate::iid::IidError;
use crate::iid::IidSet;
use crate::ldtk;
use crate::reexports::field_instance::FieldInstance;
use crate::reexports::field_instance::FieldInstanceError;
use crate::reexports::level_background_position::LevelBackgroundPosition;
use crate::reexports::neighbour::Neighbour;
use crate::reexports::neighbour::NeighbourError;
use crate::util::bevy_color_from_ldtk;
use crate::util::ldtk_path_to_bevy_path;
use crate::util::ColorParseError;

use super::event::LdkAssetEvent;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum LdtkLevelError {
    #[error(transparent)]
    IidError(#[from] IidError),
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
    #[error(transparent)]
    NeighbourError(#[from] NeighbourError),
    #[error(transparent)]
    FieldInstanceError(#[from] FieldInstanceError),
    #[error(transparent)]
    LoadDirectError(#[from] LoadDirectError),
    #[error(transparent)]
    IntoDynamicImageError(#[from] IntoDynamicImageError),
    #[error("level contains Some(bg_rel_path), but bg_pos is None")]
    BgPosIsNone,
    #[error("level contains Some(bg_pos), but bg_rel_path is None")]
    BgRelPathIsNone,
    #[error("bad handle? {0:?}")]
    BadHandle(Handle<LdtkLevel>),
}

#[derive(Asset, Debug, Reflect)]
pub struct LdtkLevel {
    // NOTE: Internal fields
    pub(crate) iid: Iid,
    pub(crate) identifier: String,
    pub(crate) children: IidSet,
    pub(crate) bg_image: Handle<Image>,
    pub(crate) level_separation: f32,
    // NOTE: LDtk exports
    pub(crate) bg_color: Color,
    pub(crate) neighbours: Vec<Neighbour>,
    pub(crate) field_instances: Vec<FieldInstance>,
    pub(crate) size: I64Vec2,
    pub(crate) location: I64Vec3,
}

#[allow(clippy::result_large_err)]
impl LdtkLevel {
    pub(crate) fn new(
        value: &ldtk::Level,
        level_separation: f32,
        load_context: &mut LoadContext,
        base_directory: &Path,
    ) -> Result<Self, LdtkLevelError> {
        let iid = Iid::from_str(&value.iid)?;

        let children = value
            .layer_instances
            .iter()
            .flatten()
            .map(|child| Iid::from_str(&child.iid))
            .collect::<Result<_, _>>()?;

        let size: I64Vec2 = (value.px_wid, value.px_hei).into();

        Ok(Self {
            iid,
            identifier: value.identifier.clone(),
            children,
            bg_image: Self::create_bg_image(
                bevy_color_from_ldtk(&value.bg_color)?,
                value.bg_pos.as_ref().map(LevelBackgroundPosition::new),
                value.bg_rel_path.clone(),
                size.as_vec2(),
                load_context,
                iid,
                base_directory,
            )?,
            level_separation,
            bg_color: bevy_color_from_ldtk(&value.bg_color)?,
            neighbours: value
                .neighbours
                .iter()
                .map(Neighbour::new)
                .collect::<Result<_, _>>()?,
            field_instances: value
                .field_instances
                .iter()
                .map(FieldInstance::new)
                .collect::<Result<_, _>>()?,
            size,
            location: (value.world_x, value.world_y, value.world_depth).into(),
        })
    }

    pub(crate) fn level_background_system(
        mut commands: Commands,
        mut events: EventReader<LdkAssetEvent<LdtkLevel>>,
        level_assets: Res<Assets<LdtkLevel>>,
    ) -> Result<(), LdtkLevelError> {
        for LdkAssetEvent::<Self>::Modified { entity, handle } in events.read() {
            let level_asset = level_assets
                .get(handle)
                .ok_or(LdtkLevelError::BadHandle(handle.clone()))?;

            commands.entity(*entity).insert((
                level_asset.bg_image.clone(),
                Sprite {
                    anchor: Anchor::TopLeft,
                    ..default()
                },
            ));
        }

        Ok(())
    }

    fn create_bg_image(
        bg_color: Color,
        bg_pos: Option<LevelBackgroundPosition>,
        bg_rel_path: Option<String>,
        size: Vec2,
        load_context: &mut LoadContext,
        iid: Iid,
        base_directory: &Path,
        // asset_label: &str,
    ) -> Result<Handle<Image>, LdtkLevelError> {
        Ok(match (bg_pos.as_ref(), bg_rel_path.as_ref()) {
            (None, Some(_)) => return Err(LdtkLevelError::BgPosIsNone),
            (Some(_), None) => return Err(LdtkLevelError::BgRelPathIsNone),
            (None, None) => {
                let color = Srgba::from(bg_color).to_u8_array();

                let background_image = Image::new_fill(
                    Extent3d {
                        width: size.x as u32,
                        height: size.y as u32,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    &color,
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::default(),
                );

                load_context.add_labeled_asset(format!("{}%background", iid), background_image)
            }
            (Some(bg_pos), Some(bg_rel_path)) => {
                let background_image: Image = block_on(async {
                    load_context
                        .loader()
                        .direct()
                        .load(ldtk_path_to_bevy_path(
                            base_directory,
                            Path::new(bg_rel_path),
                        ))
                        .await
                })?
                // .map_err(|err| err.error)?
                .take();

                let background_image = background_image.try_into_dynamic()?;

                let cropped = background_image.crop_imm(
                    bg_pos.crop_top_left.x as u32,
                    bg_pos.crop_top_left.y as u32,
                    (bg_pos.crop_top_left.x + bg_pos.crop_bottom_right.x) as u32,
                    (bg_pos.crop_top_left.y + bg_pos.crop_bottom_right.y) as u32,
                );

                let new_size =
                    ((bg_pos.crop_bottom_right - bg_pos.crop_top_left) * bg_pos.scale).as_uvec2();

                let scaled = cropped.resize(new_size.x, new_size.y, FilterType::Triangle);

                let color = Srgba::from(bg_color).to_u8_array();

                let mut background_color = RgbaImage::new(size.x as u32, size.y as u32);

                // TODO: Is there an implicit way to do this?
                for (_, _, p) in background_color.enumerate_pixels_mut() {
                    *p = image::Rgba(color);
                }

                let dynamic_image = image::DynamicImage::from(scaled);

                overlay(
                    &mut background_color,
                    &dynamic_image,
                    bg_pos.top_left.x as i64,
                    bg_pos.top_left.y as i64,
                );

                let background_image = Image::from_dynamic(
                    background_color.into(),
                    true,
                    RenderAssetUsages::default(),
                );

                load_context.add_labeled_asset(format!("{}%background", iid), background_image)
                // Handle::<Image>::default()
            }
        })
    }
}

impl LdtkAsset for LdtkLevel {
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
            (self.location.z as f32) * self.level_separation,
        )
    }

    fn asset_handle_from_project(
        project: &crate::prelude::LdtkProject,
        iid: Iid,
    ) -> Option<Handle<Self>> {
        project.levels.get(&iid).cloned()
    }
}
