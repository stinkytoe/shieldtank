use bevy::color::Color;
use bevy::ecs::component::Component;
use bevy::reflect::Reflect;
use bevy_ldtk_asset::level::LevelBackground as LdtkLevelBackground;

#[derive(Component, Debug, Reflect)]
pub struct LevelBackground {
    pub color: Color,
    pub background: Option<LdtkLevelBackground>,
}

/// The presence of this component signifies that [shieldtank] is responsible for loading/updating
/// the [LevelBackground] component.
#[derive(Component, Debug, Reflect)]
pub struct LevelBackgroundAutomation;
