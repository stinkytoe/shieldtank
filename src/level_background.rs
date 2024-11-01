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
use bevy::state::commands;
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
        let mut image = image::RgbaImage::new(self.size.x as u32, self.size.y as u32);

        image.enumerate_pixels_mut().for_each(|(_, _, pixel)| {
            let c = self.color.to_srgba().to_u8_array();
            *pixel = image::Rgba(c);
        });

        if let Some(background) = self.background.as_ref() {}

        let image = bevy::render::texture::Image::from_dynamic(
            image::DynamicImage::from(image),
            true,
            RenderAssetUsages::default(),
        );

        Ok(assets.add(image))
    }
}

/// The presence of this component signifies that [shieldtank] is responsible for loading/updating
/// the [LevelBackground] component.
#[derive(Component, Debug, Reflect)]
pub struct LevelBackgroundAutomation;

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
                //texture_atlas: todo!(),
                //color: todo!(),
                //flip_x: todo!(),
                //flip_y: todo!(),
                //custom_size: todo!(),
                //rect: todo!(),
                anchor: Anchor::TopLeft,
                ..Default::default()
            });

            debug!("Sprite inserted!");
            Ok(())
        })?;

    Ok(())
}
