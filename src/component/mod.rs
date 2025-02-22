use bevy_asset::Handle;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::world::Ref;
use bevy_hierarchy::{Children, Parent};
use bevy_ldtk_asset::prelude::LdtkAsset;
use bevy_reflect::Reflect;
use bevy_render::view::Visibility;
use bevy_transform::components::{GlobalTransform, Transform};

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
    Ref<'a, GlobalTransform>,
    Ref<'a, Visibility>,
    Option<Ref<'a, Parent>>,
    Option<Ref<'a, Children>>,
    Option<Ref<'a, ShieldtankComponentFinalized>>,
);

#[derive(Component, Debug, Reflect)]
#[require(Transform)]
#[require(Visibility)]
pub struct ShieldtankComponent<Asset: LdtkAsset> {
    pub handle: Handle<Asset>,
    pub config: Handle<ProjectConfig>,
}

#[derive(Component, Debug, Reflect)]
pub struct ShieldtankComponentFinalized {
    pub(crate) just_finalized: bool,
}
