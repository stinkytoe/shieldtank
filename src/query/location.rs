use bevy_ecs::query::{Changed, QueryData};
use bevy_ecs::world::{Mut, Ref};
use bevy_math::Vec2;
use bevy_transform::components::{GlobalTransform, Transform};

pub type ShieldtankWorldLocation = ShieldtankLocationDataReadOnly;
pub type ShieldtankWorldLocationMut = ShieldtankLocationData;
pub type ShieldtankWorldLocationChanged = Changed<GlobalTransform>;

#[derive(QueryData)]
#[query_data(mutable)]
#[query_data(derive(Debug))]
pub struct ShieldtankLocationData {
    global_transform: Ref<'static, GlobalTransform>,
    transform: Mut<'static, Transform>,
}

impl ShieldtankLocationDataItem<'_, '_> {
    pub fn get(&self) -> Vec2 {
        self.global_transform.translation().truncate()
    }

    pub fn set(&mut self, location: Vec2) {
        let diff = location - self.get();
        let diff = diff.extend(0.0);
        self.transform.translation += diff;
    }

    pub fn add(&mut self, location: Vec2) {
        self.transform.translation += location.extend(0.0);
    }
}

impl ShieldtankLocationDataReadOnlyItem<'_, '_> {
    pub fn get(&self) -> Vec2 {
        self.global_transform.translation().truncate()
    }
}
