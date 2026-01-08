use bevy_ecs::query::{QueryData, QueryFilter};
use bevy_ecs::system::{Query, Res, SystemParam};
use bevy_ldtk_asset::iid::Iid;

use crate::component::iid::IidRegistry;

#[derive(SystemParam)]
pub struct QueryByIid<'w, 's, D: QueryData + 'static, F: QueryFilter + 'static = ()> {
    pub(crate) inner_query: Query<'w, 's, D, F>,
    pub(crate) iid_registry: Res<'w, IidRegistry>,
}

impl<'w, 's, D, F> QueryByIid<'w, 's, D, F>
where
    D: QueryData<ReadOnly = D> + 'static,
    F: QueryFilter + 'static,
{
    #[inline]
    pub fn get(&'w self, iid: Iid) -> Option<D::Item<'w, 's>> {
        let entity = *self.iid_registry.registry.get(&iid)?;
        self.inner_query.get(entity).ok()
    }
}

impl<'s, D, F> QueryByIid<'_, 's, D, F>
where
    D: QueryData + 'static,
    F: QueryFilter + 'static,
{
    #[inline]
    pub fn get_mut<'w>(&'w mut self, iid: Iid) -> Option<D::Item<'w, 's>> {
        let entity = *self.iid_registry.registry.get(&iid)?;
        self.inner_query.get_mut(entity).ok()
    }
}
