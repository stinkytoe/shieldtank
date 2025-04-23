use bevy_asset::prelude::AssetChanged;
use bevy_asset::{AsAssetId, Assets, Handle, RenderAssetUsages};
use bevy_color::{Color, ColorToPacked as _};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::removal_detection::RemovedComponents;
use bevy_ecs::system::{Commands, Query, ResMut};
use bevy_image::Image;
use bevy_ldtk_asset::level::LevelBackground as LdtkLevelBackground;
use bevy_log::{debug, error};
use bevy_math::{I64Vec2, UVec2};
use bevy_reflect::Reflect;
use bevy_sprite::{Anchor, Sprite};
use image::imageops::{crop_imm, overlay, resize};

use crate::result::Result;

#[derive(Debug, Component, Reflect)]
pub struct LevelBackgroundImage {
    pub color: Color,
    pub size: UVec2,
    pub image: Handle<Image>,
    pub crop_corner: UVec2,
    pub crop_size: UVec2,
    pub scale: UVec2,
    pub corner: I64Vec2,
}

impl LevelBackgroundImage {
    pub(crate) fn new(
        color: Color,
        size: UVec2,
        ldtk_level_background: &LdtkLevelBackground,
    ) -> Self {
        let image = ldtk_level_background.image.clone();
        let crop_corner = ldtk_level_background.crop_corner.as_uvec2();
        let crop_size = ldtk_level_background.crop_size.as_uvec2();

        let scale = ldtk_level_background.scale.as_uvec2();
        let corner = ldtk_level_background.corner;

        Self {
            color,
            size,
            image,
            crop_corner,
            crop_size,
            scale,
            corner,
        }
    }

    pub(crate) fn generate_sprite_image(&self, background_image: Image) -> Result<Image> {
        let mut new_background_image = image::RgbaImage::new(self.size.x, self.size.y);

        new_background_image
            .enumerate_pixels_mut()
            .for_each(|(_, _, pixel)| {
                let c = self.color.to_srgba().to_u8_array();
                *pixel = image::Rgba(c);
            });

        let background_image = background_image.try_into_dynamic()?;

        let crop = crop_imm(
            &background_image,
            self.crop_corner.x,
            self.crop_corner.y,
            self.crop_size.x,
            self.crop_size.y,
        )
        .to_image();

        let old_size = self.crop_size;
        let new_size = old_size * self.scale;

        let scale = resize(
            &crop,
            new_size.x,
            new_size.y,
            image::imageops::FilterType::Nearest,
        );

        overlay(
            &mut new_background_image,
            &scale,
            self.corner.x,
            self.corner.y,
        );

        let new_background_image = Image::from_dynamic(
            image::DynamicImage::from(new_background_image),
            true,
            RenderAssetUsages::default(),
        );

        Ok(new_background_image)
    }
}

impl AsAssetId for LevelBackgroundImage {
    type Asset = Image;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.image.id()
    }
}

#[allow(clippy::type_complexity)]
pub fn level_background_image_system(
    query: Query<
        (Entity, &LevelBackgroundImage),
        Or<(
            Changed<LevelBackgroundImage>,
            AssetChanged<LevelBackgroundImage>,
        )>,
    >,
    mut images: ResMut<Assets<Image>>,
    mut removed: RemovedComponents<LevelBackgroundImage>,
    mut commands: Commands,
) -> bevy_ecs::error::Result<()> {
    query
        .iter()
        .try_for_each(|(entity, component)| -> bevy_ecs::error::Result<()> {
            let Some(image) = images.get(component.as_asset_id()).cloned() else {
                error!(
                    "Bad background image handle! {entity:?} {:?}",
                    component.image
                );
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

            debug!("Processing LevelBackgroundImage for {entity:?}");

            commands.entity(entity).insert(sprite);

            Ok(())
        })?;

    removed.read().for_each(|entity| {
        commands.entity(entity).remove::<Sprite>();
    });

    Ok(())
}
