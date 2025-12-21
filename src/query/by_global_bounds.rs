use bevy_ecs::query::{QueryData, QueryFilter};
use bevy_ecs::system::{Query, SystemParam};
use bevy_math::Vec2;

use crate::component::global_bounds::ShieldtankGlobalBounds;

#[derive(SystemParam)]
pub struct QueryByGlobalBounds<'w, 's, D: QueryData + 'static, F: QueryFilter + 'static = ()> {
    pub(crate) inner_query: Query<'w, 's, (&'static ShieldtankGlobalBounds, D), F>,
}

macro_rules! by_location_closure {
    ($self:expr, $iter:tt, $location:expr) => {
        $self
            .inner_query
            .$iter()
            .filter(move |(global_bounds, _)| global_bounds.contains($location))
            .map(|(_, data)| data)
    };
}

impl<'w, 's, D, F> QueryByGlobalBounds<'w, 's, D, F>
where
    D: QueryData<ReadOnly = D> + 'static,
    F: QueryFilter + 'static,
{
    #[inline]
    pub fn by_location(&'w self, location: Vec2) -> impl Iterator<Item = D::Item<'w, 's>> {
        by_location_closure!(self, iter, location)
    }

    #[inline]
    pub fn any(&'w self, location: Vec2) -> bool {
        self.inner_query
            .iter()
            .any(|(global_bounds, _)| global_bounds.contains(location))
    }
}

impl<D, F> QueryByGlobalBounds<'_, '_, D, F>
where
    D: QueryData<ReadOnly = D> + 'static,
    F: QueryFilter + 'static,
{
    #[inline]
    pub fn by_location_mut<'w, 's>(
        &'w mut self,
        location: Vec2,
    ) -> impl Iterator<Item = D::Item<'w, 's>>
    where
        'w: 's,
    {
        by_location_closure!(self, iter_mut, location)
    }
}
