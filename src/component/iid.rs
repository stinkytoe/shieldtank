use bevy_app::Plugin;
use bevy_derive::Deref;
use bevy_ecs::component::Component;
use bevy_ldtk_asset::iid::Iid;
use bevy_reflect::Reflect;

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

pub struct IidPlugin;
impl Plugin for IidPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ShieldtankIid>();
    }
}
