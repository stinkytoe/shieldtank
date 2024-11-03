use bevy::ecs::component::Component;
use bevy::reflect::Reflect;

#[derive(Component, Debug, Reflect)]
pub struct TransformAutomation;

#[derive(Component, Debug, Reflect)]
pub struct LevelBackgroundAutomation;

#[derive(Component, Debug, Reflect)]
pub struct IntGridAutomation;

#[derive(Component, Debug, Reflect)]
pub struct TilesAutomation;
