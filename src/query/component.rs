use bevy_asset::AsAssetId;
use bevy_ecs::entity::Entity;
use bevy_ecs::name::Name;
use bevy_ecs::query::{QueryData, QueryFilter};
use bevy_ecs::system::{Query, SystemParam};
use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::prelude::LdtkAsset;

use crate::component::entity::LdtkEntity;
use crate::component::iid::LdtkIid;
use crate::component::layer::LdtkLayer;
use crate::component::level::LdtkLevel;
use crate::component::project::LdtkProject;
use crate::component::shieldtank_component::ShieldtankComponent;
use crate::component::world::LdtkWorld;

#[derive(QueryData)]
struct ShieldtankComponentData<S>
where
    S: ShieldtankComponent + AsAssetId,
    <S as AsAssetId>::Asset: LdtkAsset,
{
    entity: Entity,
    component: &'static S,
    iid: &'static LdtkIid,
    name: &'static Name,
}

#[allow(clippy::type_complexity)]
#[allow(private_bounds)]
#[derive(SystemParam)]
pub struct ShieldtankComponentQuery<'w, 's, S, D, F = ()>
where
    S: ShieldtankComponent + AsAssetId,
    <S as AsAssetId>::Asset: LdtkAsset,
    D: QueryData + 'static,
    F: QueryFilter + 'static,
{
    query: Query<'w, 's, (ShieldtankComponentData<S>, D), F>,
}

#[allow(clippy::type_complexity)]
#[allow(private_bounds)]
impl<S, D, F> ShieldtankComponentQuery<'_, '_, S, D, F>
where
    S: ShieldtankComponent + AsAssetId,
    <S as AsAssetId>::Asset: LdtkAsset,
    D: QueryData<ReadOnly = D> + 'static,
    F: QueryFilter + 'static,
{
    pub fn get(&self, entity: Entity) -> Option<<D as QueryData>::Item<'_>> {
        self.query
            .as_readonly()
            .into_iter()
            .find(|(data, ..)| data.entity == entity)
            .map(|(_, data)| data)
    }

    pub fn get_iid(&self, iid: Iid) -> Option<<D as QueryData>::Item<'_>> {
        self.query
            .as_readonly()
            .into_iter()
            .find(|(data, ..)| **data.iid == iid)
            .map(|(_, data)| data)
    }

    pub fn with_name<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = <D as QueryData>::Item<'a>> {
        self.query
            .as_readonly()
            .into_iter()
            .filter(move |(data, ..)| data.name.as_str() == name)
            .map(|(_, data)| data)
    }

    pub fn iter(&self) -> impl Iterator<Item = <D as QueryData>::Item<'_>> {
        self.query.as_readonly().into_iter().map(|(_, data)| data)
    }
}

#[allow(clippy::type_complexity)]
#[allow(private_bounds)]
impl<S, D, F> ShieldtankComponentQuery<'_, '_, S, D, F>
where
    S: ShieldtankComponent + AsAssetId,
    <S as AsAssetId>::Asset: LdtkAsset,
    D: QueryData,
    F: QueryFilter,
{
    pub fn get_mut(&mut self, entity: Entity) -> Option<<D as QueryData>::Item<'_>> {
        self.query
            .reborrow()
            .into_iter()
            .find(|(data, ..)| data.entity == entity)
            .map(|(_, data)| data)
    }

    pub fn get_iid_mut(&mut self, iid: Iid) -> Option<<D as QueryData>::Item<'_>> {
        self.query
            .reborrow()
            .into_iter()
            .find(|(data, ..)| **data.iid == iid)
            .map(|(_, data)| data)
    }

    pub fn with_name_mut<'a>(
        &'a mut self,
        name: &'a str,
    ) -> impl Iterator<Item = <D as QueryData>::Item<'a>> {
        self.query
            .reborrow()
            .into_iter()
            .filter(move |(data, ..)| data.name.as_str() == name)
            .map(|(_, data)| data)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = <D as QueryData>::Item<'_>> {
        self.query.reborrow().into_iter().map(|(_, data)| data)
    }
}

pub type LdtkProjectQuery<'w, 's, D, F = ()> = ShieldtankComponentQuery<'w, 's, LdtkProject, D, F>;
pub type LdtkWorldQuery<'w, 's, D, F = ()> = ShieldtankComponentQuery<'w, 's, LdtkWorld, D, F>;
pub type LdtkLevelQuery<'w, 's, D, F = ()> = ShieldtankComponentQuery<'w, 's, LdtkLevel, D, F>;
pub type LdtkLayerQuery<'w, 's, D, F = ()> = ShieldtankComponentQuery<'w, 's, LdtkLayer, D, F>;
pub type LdtkEntityQuery<'w, 's, D, F = ()> = ShieldtankComponentQuery<'w, 's, LdtkEntity, D, F>;
