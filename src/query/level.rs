use bevy_ecs::query::{QueryData, QueryFilter};
use bevy_math::Vec2;
use bevy_transform::components::GlobalTransform;

use crate::component::{global_bounds::LdtkGlobalBounds, level::LdtkLevel};

use super::component::ShieldtankComponentQuery;

#[derive(QueryData)]
pub struct LevelComponentData {
    global_transform: &'static GlobalTransform,
    global_bounds: &'static LdtkGlobalBounds,
}

pub type LdtkLevelQuery<'w, 's, D, F = ()> =
    ShieldtankComponentQuery<'w, 's, LdtkLevel, LevelComponentData, D, F>;

impl<D, F> LdtkLevelQuery<'_, '_, D, F>
where
    D: QueryData<ReadOnly = D>,
    F: QueryFilter,
{
    pub fn location_in_bounds(
        &self,
        location: Vec2,
    ) -> impl Iterator<Item = <D as QueryData>::Item<'_, '_>> {
        self.query
            .iter()
            .filter(move |(_, data, _)| data.global_bounds.contains(location))
            .map(|(_, _, data)| data)
    }
}
