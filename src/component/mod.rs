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
pub mod project;
pub mod world;

pub type ShieldtankQueryData<'a, A> = (
    Entity,
    Ref<'a, ShieldtankComponent<A>>,
    Ref<'a, Transform>,
    Ref<'a, Visibility>,
);

#[derive(Component, Debug, Reflect)]
#[require(Transform)]
#[require(Visibility)]
pub struct ShieldtankComponent<Asset: LdtkAsset> {
    pub handle: Handle<Asset>,
    pub config: Handle<ProjectConfig>,
}

pub trait ShieldtankComponentTrait<A: LdtkAsset> {}
