use bevy::prelude::*;

use crate::ldtk;

#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct LevelBackgroundPosition {
    pub crop_top_left: Vec2,
    pub crop_bottom_right: Vec2,
    pub scale: Vec2,
    pub top_left: Vec2,
}

impl LevelBackgroundPosition {
    pub(crate) fn new(value: &ldtk::LevelBackgroundPosition) -> Self {
        let crop_top_left = (value.crop_rect[0] as f32, value.crop_rect[1] as f32).into();
        let crop_bottom_right =
            crop_top_left + Vec2::new(value.crop_rect[2] as f32, value.crop_rect[3] as f32);
        let scale = (value.scale[0] as f32, value.scale[1] as f32).into();
        let top_left = (value.top_left_px[0] as f32, value.top_left_px[1] as f32).into();

        Self {
            crop_top_left,
            crop_bottom_right,
            scale,
            top_left,
        }
    }
}
