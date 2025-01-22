use bevy_asset::Handle;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::world::Ref;
use bevy_ldtk_asset::prelude::LdtkAsset;
use bevy_reflect::Reflect;
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::project_config::ProjectConfig;

pub mod entity;
pub mod layer;
pub mod level;
pub mod plugin;
pub mod project;
pub mod world;

pub(crate) mod systems;

pub type ShieldtankQueryData<'a, A> = (
    Entity,
    Ref<'a, ShieldtankComponent<A>>,
    Ref<'a, Transform>,
    Ref<'a, Visibility>,
    Option<Ref<'a, ShieldtankComponentLoaded>>,
);

#[derive(Component, Debug, Reflect)]
#[require(Transform)]
#[require(Visibility)]
pub struct ShieldtankComponent<Asset: LdtkAsset> {
    pub handle: Handle<Asset>,
    pub config: Handle<ProjectConfig>,
}

#[derive(Component, Debug, Reflect)]
pub struct ShieldtankComponentLoaded {
    pub(crate) just_finalized: bool,
}
