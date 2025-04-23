use bevy_derive::Deref;
use bevy_ecs::component::Component;
use bevy_ldtk_asset::iid::Iid;
use bevy_reflect::Reflect;

#[derive(Clone, Copy, Debug, Deref, PartialEq, Eq, Component, Reflect)]
pub struct LdtkIid(#[deref] Iid);

impl LdtkIid {
    pub fn new(iid: Iid) -> Self {
        Self(iid)
    }
}

impl PartialEq<Iid> for LdtkIid {
    fn eq(&self, other: &Iid) -> bool {
        self.0 == *other
    }
}

impl From<Iid> for LdtkIid {
    fn from(value: Iid) -> Self {
        Self(value)
    }
}
