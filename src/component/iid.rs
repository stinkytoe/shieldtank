use bevy_app::Plugin;
use bevy_derive::Deref;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::lifecycle::RemovedComponents;
use bevy_ecs::query::Changed;
use bevy_ecs::resource::Resource;
use bevy_ecs::system::{Query, ResMut};
use bevy_ldtk_asset::iid::Iid;
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;

use crate::component::shieldtank_component::ShieldtankComponentSystemSet;

#[derive(Clone, Copy, Debug, Deref, PartialEq, Eq, Component, Reflect)]
#[component(immutable)]
pub struct ShieldtankIid(#[deref] Iid);

impl ShieldtankIid {
    pub fn new(iid: Iid) -> Self {
        Self(iid)
    }
}

impl PartialEq<Iid> for ShieldtankIid {
    fn eq(&self, other: &Iid) -> bool {
        self.0 == *other
    }
}

impl From<Iid> for ShieldtankIid {
    fn from(value: Iid) -> Self {
        Self(value)
    }
}

#[derive(Debug, Default, Reflect, Resource)]
pub(crate) struct IidRegistry {
    pub(crate) registry: HashMap<Iid, Entity>,
}

fn register_iid(
    added_query: Query<(Entity, &ShieldtankIid), Changed<ShieldtankIid>>,
    mut removed: RemovedComponents<ShieldtankIid>,
    mut iid_registry: ResMut<IidRegistry>,
) {
    added_query.iter().for_each(|(entity, &iid)| {
        iid_registry.registry.insert(*iid, entity);
    });

    removed.read().for_each(|removed_entity| {
        iid_registry
            .registry
            .retain(|_, entity| *entity != removed_entity)
    });
}

pub struct IidPlugin;
impl Plugin for IidPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ShieldtankIid>()
            .insert_resource(IidRegistry::default())
            .register_type::<IidRegistry>()
            .add_systems(ShieldtankComponentSystemSet, register_iid);
    }
}
