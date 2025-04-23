use bevy_ecs::query::{QueryData, QueryFilter, With};
use bevy_ecs::system::{Query, SystemParam};
use bevy_math::Vec2;

use crate::component::entity::LdtkEntity;
use crate::component::global_bounds::GlobalBounds;
use crate::component::layer::LdtkLayer;
use crate::component::level::LdtkLevel;

#[derive(SystemParam)]
pub struct InBoundsQuery<'w, 's, D, F>
where
    D: QueryData + 'static,
    F: QueryFilter + 'static,
{
    query: Query<'w, 's, (&'static GlobalBounds, D), F>,
}

impl<D, F> InBoundsQuery<'_, '_, D, F>
where
    D: QueryData<ReadOnly = D>,
    F: QueryFilter,
{
    pub fn in_bounds(&self, location: Vec2) -> impl Iterator<Item = <D as QueryData>::Item<'_>> {
        self.query
            .as_readonly()
            .into_iter()
            .filter(move |(global_bounds, ..)| global_bounds.contains(location))
            .map(|(_, data)| data)
    }
}

impl<D, F> InBoundsQuery<'_, '_, D, F>
where
    D: QueryData,
    F: QueryFilter,
{
    pub fn in_bounds_mut(
        &mut self,
        location: Vec2,
    ) -> impl Iterator<Item = <D as QueryData>::Item<'_>> {
        self.query
            .reborrow()
            .into_iter()
            .filter(move |(global_bounds, ..)| global_bounds.contains(location))
            .map(|(_, data)| data)
    }
}

pub type LdtkEntityInBoundsQuery<'w, 's, D, F = ()> =
    InBoundsQuery<'w, 's, D, (With<LdtkEntity>, F)>;
pub type LdtkLayerInBoundsQuery<'w, 's, D, F = ()> = InBoundsQuery<'w, 's, D, (With<LdtkLayer>, F)>;
pub type LdtkLevelInBoundsQuery<'w, 's, D, F = ()> = InBoundsQuery<'w, 's, D, (With<LdtkLevel>, F)>;
