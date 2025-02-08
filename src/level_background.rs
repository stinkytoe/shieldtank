use bevy_app::Plugin;
use bevy_app::PostUpdate;
use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_color::Color;
use bevy_color::ColorToPacked;
use bevy_ecs::change_detection::DetectChanges as _;
use bevy_ecs::component::Component;
use bevy_ecs::system::Commands;
use bevy_ecs::system::ResMut;
use bevy_image::Image;
use bevy_ldtk_asset::level::Level as LdtkLevel;
use bevy_ldtk_asset::level::LevelBackground as LdtkLevelBackground;
use bevy_log::debug;
use bevy_log::error;
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_sprite::Anchor;
use bevy_sprite::Sprite;

use crate::error::Result;
use crate::query::ShieldtankQuery;
use crate::shieldtank_error;

#[derive(Component, Debug, Reflect)]
pub struct LevelBackground {
    pub color: Color,
    pub size: Vec2,
    pub background: Option<LdtkLevelBackground>,
}

impl LevelBackground {
    pub(crate) fn new(value: &LdtkLevel) -> Self {
        let color = value.bg_color;
        let size = value.size.as_vec2();
        let background = value.background.clone();
        LevelBackground {
            color,
            size,
            background,
        }
    }

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
                .ok_or(shieldtank_error!("bad handle! {:?}", background.image))?
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
                background.corner.x,
                background.corner.y,
            );
        }

        let background_image = Image::from_dynamic(
            image::DynamicImage::from(background_image),
            true,
            RenderAssetUsages::default(),
        );

        Ok(assets.add(background_image))
    }
}

fn level_background_system(
    mut commands: Commands,
    mut image_assets: ResMut<Assets<Image>>,
    shieldtank_query: ShieldtankQuery,
) {
    shieldtank_query
        .iter_levels()
        .filter(|item| {
            item.get_level_background()
                .as_ref()
                .and_then(|level_background| level_background.is_changed().then_some(()))
                .is_some()
        })
        .inspect(|item| debug!("level background loaded for: {}", item.get_identifier()))
        .map(|item| -> Result<()> {
            let Some(background) = item.get_level_background() else {
                return Ok(());
            };

            let image = background.generate_image(&mut image_assets)?;
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
                error!("failed to load level background: {e}");
            }
        });
}

pub struct LevelBackgroundPlugin;
impl Plugin for LevelBackgroundPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LevelBackground>()
            .add_systems(PostUpdate, level_background_system);
    }
}
