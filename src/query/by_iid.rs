use bevy_ecs::query::{QueryData, QueryFilter};
use bevy_ecs::system::{Query, SystemParam};
use bevy_ldtk_asset::iid::Iid;

use crate::component::iid::ShieldtankIid;

#[derive(SystemParam)]
pub struct QueryByIid<'w, 's, D: QueryData + 'static, F: QueryFilter + 'static = ()> {
    pub(crate) inner_query: Query<'w, 's, (&'static ShieldtankIid, D), F>,
}

macro_rules! get_closure {
    ($self:expr, $iter:tt, $iid:expr) => {
        $self
            .inner_query
            .$iter()
            .find(|(iid, _)| **iid == $iid)
            .map(|(_, data)| data)
    };
}

impl<'w, 's, D, F> QueryByIid<'w, 's, D, F>
where
    D: QueryData<ReadOnly = D> + 'static,
    F: QueryFilter + 'static,
{
    #[inline]
    pub fn get(&'w self, iid: Iid) -> Option<D::Item<'w, 's>> {
        get_closure!(self, iter, iid)
    }
}

impl<D, F> QueryByIid<'_, '_, D, F>
where
    D: QueryData + 'static,
    F: QueryFilter + 'static,
{
    #[inline]
    pub fn get_mut<'w, 's>(&'w mut self, iid: Iid) -> Option<D::Item<'w, 's>>
    where
        'w: 's,
    {
        get_closure!(self, iter_mut, iid)
    }
}
