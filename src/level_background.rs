use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::color::ColorToPacked;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Query;
use bevy::ecs::system::ResMut;
use bevy::log::debug;
use bevy::math::Vec2;
use bevy::reflect::Reflect;
use bevy::render::texture::Image;
use bevy::sprite::{Anchor, Sprite};
use bevy::{color::Color, prelude::Changed};
use bevy_ldtk_asset::level::LevelBackground as LdtkLevelBackground;

use crate::{Error, Result};

#[derive(Component, Debug, Reflect)]
pub struct LevelBackground {
    pub color: Color,
    pub size: Vec2,
    pub background: Option<LdtkLevelBackground>,
}

impl LevelBackground {
    fn generate_image(&self, assets: &mut Assets<Image>) -> Result<Handle<Image>> {
        let mut background_image = image::RgbaImage::new(self.size.x as u32, self.size.y as u32);

        background_image
            .enumerate_pixels_mut()
            .for_each(|(_, _, pixel)| {
                let c = self.color.to_srgba().to_u8_array();
                *pixel = image::Rgba(c);
            });

        if let Some(background) = self.background.as_ref() {
            let ldtk_background_image = assets
                .get(background.image.id())
                .ok_or(Error::BadHandle)?
                .clone()
                .try_into_dynamic()?;

            let crop = image::imageops::crop_imm(
                &ldtk_background_image,
                background.crop_corner.x as u32,
                background.crop_corner.y as u32,
                background.crop_size.x as u32,
                background.crop_size.y as u32,
            )
            .to_image();

            let old_size = background.crop_size;
            let new_size = old_size * background.scale;

            let scale = image::imageops::resize(
                &crop,
                new_size.x as u32,
                new_size.y as u32,
                image::imageops::FilterType::Nearest,
            );

            image::imageops::overlay(
                &mut background_image,
                &scale,
                background.corner.x as i64,
                background.corner.y as i64,
            );
        }

        let background_image = bevy::render::texture::Image::from_dynamic(
            image::DynamicImage::from(background_image),
            true,
            RenderAssetUsages::default(),
        );

        Ok(assets.add(background_image))
    }
}

pub(crate) fn level_background_system(
    mut commands: Commands,
    mut image_assets: ResMut<Assets<Image>>,
    added_query: Query<(Entity, &LevelBackground), Changed<LevelBackground>>,
) -> Result<()> {
    added_query
        .iter()
        .try_for_each(|(entity, background)| -> Result<()> {
            let image = background.generate_image(&mut image_assets)?;

            commands.entity(entity).insert(Sprite {
                image,
                anchor: Anchor::TopLeft,
                ..Default::default()
            });

            debug!("Level background generated!");
            Ok(())
        })?;

    Ok(())
}
