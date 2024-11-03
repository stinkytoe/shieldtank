use bevy_ecs::component::Component;
use bevy_reflect::Reflect;

#[derive(Component, Debug, Reflect)]
pub struct TransformAutomation;

#[derive(Component, Debug, Reflect)]
pub struct LevelBackgroundAutomation;

#[derive(Component, Debug, Reflect)]
pub struct IntGridAutomation;

#[derive(Component, Debug, Reflect)]
pub struct TilesAutomation;
