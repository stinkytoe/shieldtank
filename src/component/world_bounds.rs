use bevy_app::Plugin;
use bevy_derive::Deref;
use bevy_ecs::component::Component;
use bevy_math::{Rect, Vec2};
use bevy_reflect::Reflect;

#[derive(Clone, Debug, Deref, Component, Reflect)]
pub struct ShieldtankWorldBounds {
    #[deref]
    bounds: Rect,
}

impl ShieldtankWorldBounds {
    pub(crate) fn new(p0: Vec2, p1: Vec2) -> Self {
        let bounds = Rect::from_corners(p0, p1);

        Self { bounds }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.bounds.contains(point)
    }

    pub fn bounds(&self) -> Rect {
        self.bounds
    }
}

impl From<Rect> for ShieldtankWorldBounds {
    fn from(bounds: Rect) -> Self {
        Self { bounds }
    }
}

pub struct GlobalBoundsPlugin;
impl Plugin for GlobalBoundsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ShieldtankWorldBounds>();
    }
}
