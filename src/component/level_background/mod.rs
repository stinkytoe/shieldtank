use bevy_app::Plugin;
use color::{level_background_color_system, LevelBackgroundColor};
use image::{level_background_image_system, LevelBackgroundImage};

use super::shieldtank_component::ShieldtankComponentSystemSet;

pub mod color;
pub mod image;

pub struct LevelBackgroundPlugin;
impl Plugin for LevelBackgroundPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LevelBackgroundColor>();
        app.add_systems(ShieldtankComponentSystemSet, level_background_color_system);

        app.register_type::<LevelBackgroundImage>();
        app.add_systems(ShieldtankComponentSystemSet, level_background_image_system);
    }
}
