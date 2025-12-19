use bevy_ecs::query::{QueryData, QueryFilter};
use bevy_ecs::system::{Query, SystemParam};
use bevy_ldtk_asset::iid::Iid;

use crate::prelude::ShieldtankIid;

#[derive(SystemParam)]
pub struct QueryByIid<'w, 's, D: QueryData + 'static, F: QueryFilter + 'static = ()> {
    pub(crate) inner_query: Query<'w, 's, (&'static ShieldtankIid, D), F>,
}

impl<'w, 's, D, F> QueryByIid<'w, 's, D, F>
where
    D: QueryData<ReadOnly = D> + 'static,
    F: QueryFilter + 'static,
{
    pub fn get(&'w self, iid: Iid) -> Option<D::Item<'w, 's>> {
        self.inner_query
            .iter()
            .find(|(inner_iid, _)| **inner_iid == iid)
            .map(|(_, data)| data)
    }
}

impl<D, F> QueryByIid<'_, '_, D, F>
where
    D: QueryData + 'static,
    F: QueryFilter + 'static,
{
    pub fn get_mut<'w, 's>(&'w mut self, iid: Iid) -> Option<D::Item<'w, 's>>
    where
        'w: 's,
    {
        self.inner_query
            .iter_mut()
            .find(|(inner_iid, _)| **inner_iid == iid)
            .map(|(_, data)| data)
    }
}
