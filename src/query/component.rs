use bevy_asset::{AsAssetId, Assets};
use bevy_ecs::entity::Entity;
use bevy_ecs::name::Name;
use bevy_ecs::query::{QueryData, QueryFilter};
use bevy_ecs::system::{Query, Res, SystemParam};
use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::prelude::LdtkAsset;
use bevy_transform::components::GlobalTransform;

use crate::component::iid::LdtkIid;
use crate::component::project::LdtkProject;
use crate::component::shieldtank_component::ShieldtankComponent;
use crate::component::world::LdtkWorld;

use super::iter::ShieldtankComponentIter;

#[derive(QueryData)]
pub(crate) struct ShieldtankComponentData<S>
where
    S: ShieldtankComponent + AsAssetId,
    <S as AsAssetId>::Asset: LdtkAsset,
{
    entity: Entity,
    pub(crate) component: &'static S,
    iid: &'static LdtkIid,
    name: &'static Name,
    global_transform: &'static GlobalTransform,
}

#[allow(clippy::type_complexity)]
#[allow(private_bounds)]
#[derive(SystemParam)]
pub struct ShieldtankComponentQuery<'w, 's, S, E, D, F = ()>
where
    S: ShieldtankComponent + AsAssetId,
    <S as AsAssetId>::Asset: LdtkAsset,
    E: QueryData + 'static,
    D: QueryData + 'static,
    F: QueryFilter + 'static,
{
    pub(crate) query: Query<'w, 's, (ShieldtankComponentData<S>, E, D), F>,
    pub(crate) assets: Res<'w, Assets<<S as AsAssetId>::Asset>>,
}

#[allow(clippy::type_complexity)]
#[allow(private_bounds)]
impl<S, E, D, F> ShieldtankComponentQuery<'_, '_, S, E, D, F>
where
    S: ShieldtankComponent + AsAssetId,
    <S as AsAssetId>::Asset: LdtkAsset,
    E: QueryData<ReadOnly = E> + 'static,
    D: QueryData<ReadOnly = D> + 'static,
    F: QueryFilter + 'static,
{
    pub fn get(&self, entity: Entity) -> Option<<D as QueryData>::Item<'_>> {
        self.query
            .as_readonly()
            .into_iter()
            .find(|(data, ..)| data.entity == entity)
            .map(|(_, _, data)| data)
    }

    pub fn get_iid(&self, iid: Iid) -> Option<<D as QueryData>::Item<'_>> {
        self.query
            .as_readonly()
            .into_iter()
            .find(|(data, ..)| **data.iid == iid)
            .map(|(_, _, data)| data)
    }

    pub fn with_name<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = <D as QueryData>::Item<'a>> {
        self.query
            .as_readonly()
            .into_iter()
            .filter(move |(data, ..)| data.name.as_str() == name)
            .map(|(_, _, data)| data)
    }

    pub fn iter(&self) -> impl Iterator<Item = <D as QueryData>::Item<'_>> {
        self.query
            .as_readonly()
            .into_iter()
            .map(|(_, _, data)| data)
    }
}

#[allow(clippy::type_complexity)]
#[allow(private_bounds)]
impl<S, E, D, F> ShieldtankComponentQuery<'_, '_, S, E, D, F>
where
    S: ShieldtankComponent + AsAssetId,
    <S as AsAssetId>::Asset: LdtkAsset,
    E: QueryData + 'static,
    D: QueryData + 'static,
    F: QueryFilter + 'static,
{
    pub fn get_mut(&mut self, entity: Entity) -> Option<<D as QueryData>::Item<'_>> {
        self.query
            .reborrow()
            .into_iter()
            .find(|(data, ..)| data.entity == entity)
            .map(|(_, _, data)| data)
    }

    pub fn get_iid_mut(&mut self, iid: Iid) -> Option<<D as QueryData>::Item<'_>> {
        self.query
            .reborrow()
            .into_iter()
            .find(|(data, ..)| **data.iid == iid)
            .map(|(_, _, data)| data)
    }

    pub fn with_name_mut<'a>(
        &'a mut self,
        name: &'a str,
    ) -> impl Iterator<Item = <D as QueryData>::Item<'a>> {
        self.query
            .reborrow()
            .into_iter()
            .filter(move |(data, ..)| data.name.as_str() == name)
            .map(|(_, _, data)| data)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = <D as QueryData>::Item<'_>> {
        self.query.reborrow().into_iter().map(|(_, _, data)| data)
    }
}

impl<'w, 's, S, E, D, F> IntoIterator for ShieldtankComponentQuery<'w, 's, S, E, D, F>
where
    S: ShieldtankComponent + AsAssetId,
    <S as AsAssetId>::Asset: LdtkAsset,
    E: QueryData<ReadOnly = E> + 'static,
    D: QueryData<ReadOnly = D> + 'static,
    F: QueryFilter + 'static,
{
    type Item = D::Item<'w>;

    type IntoIter = ShieldtankComponentIter<'w, 's, S, E, D, F>;

    fn into_iter(self) -> Self::IntoIter {
        let iter = self.query.into_iter();
        ShieldtankComponentIter { iter }
    }
}

pub type LdtkProjectQuery<'w, 's, D, F = ()> =
    ShieldtankComponentQuery<'w, 's, LdtkProject, (), D, F>;
pub type LdtkWorldQuery<'w, 's, D, F = ()> = ShieldtankComponentQuery<'w, 's, LdtkWorld, (), D, F>;
