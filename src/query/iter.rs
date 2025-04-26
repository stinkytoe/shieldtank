use bevy_asset::AsAssetId;
use bevy_ecs::query::{QueryData, QueryFilter, QueryIter};
use bevy_ldtk_asset::prelude::LdtkAsset;

use crate::component::shieldtank_component::ShieldtankComponent;

use super::component::ShieldtankComponentData;

#[allow(private_bounds)]
pub struct ShieldtankComponentIter<'w, 's, S, E, D, F>
where
    S: ShieldtankComponent + AsAssetId,
    <S as AsAssetId>::Asset: LdtkAsset,
    E: QueryData<ReadOnly = E> + 'static,
    D: QueryData<ReadOnly = D> + 'static,
    F: QueryFilter + 'static,
{
    pub(crate) iter: QueryIter<'w, 's, (ShieldtankComponentData<S>, E, D), F>,
}

impl<'w, S, E, D, F> Iterator for ShieldtankComponentIter<'w, '_, S, E, D, F>
where
    S: ShieldtankComponent + AsAssetId,
    <S as AsAssetId>::Asset: LdtkAsset,
    E: QueryData<ReadOnly = E> + 'static,
    D: QueryData<ReadOnly = D> + 'static,
    F: QueryFilter + 'static,
{
    type Item = D::Item<'w>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, _, data)| data)
    }
}
