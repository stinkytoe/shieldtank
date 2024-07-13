use bevy::{
    ecs::query::{QueryData, QueryEntityError, WorldQuery},
    prelude::*,
};
use thiserror::Error;

use crate::{assets::traits::LdtkAsset, iid::Iid, prelude::LdtkProject};

#[derive(Debug, Error)]
pub(crate) enum LdtkComponentError<A: LdtkAsset> {
    #[error(transparent)]
    QueryEntityError(#[from] QueryEntityError),
    #[error("bad project handle? {0:?}")]
    BadProjectHandle(Handle<LdtkProject>),
    #[error("bad project handle? {0:?}")]
    BadAssetHandle(Handle<A>),
    #[error("handle not found in project: {0}")]
    HandleNotFound(Iid),
}

pub(crate) trait LdtkComponent<A, S>
where
    A: LdtkAsset + Sized,
    S: QueryData,
    Self: Component + Sized,
{
    fn iid(&self) -> Iid;
    fn project_entity(&self) -> Entity;

    fn asset_handle_from_project(project: &LdtkProject, iid: Iid) -> Option<Handle<A>>;

    fn on_spawn(
        commands: &mut Commands,
        entity: Entity,
        project: &LdtkProject,
        asset: &A,
        component: &Self,
        component_set_query: &Query<S>,
    ) -> Result<(), LdtkComponentError<A>>;

    fn ldtk_asset_event_system(
        mut commands: Commands,
        //events: EventReader<LdtkAssetEvent<A>>,
        added_query: Query<(Entity, &Self), Added<Self>>,
        component_set_query: Query<S>,
        projects_query: Query<&Handle<LdtkProject>>,
        project_assets: Res<Assets<LdtkProject>>,
        assets: Res<Assets<A>>,
    ) -> Result<(), LdtkComponentError<A>> {
        for (entity, component) in added_query.iter() {
            let project_handle = projects_query.get(component.project_entity())?;

            let project_asset = project_assets
                .get(project_handle)
                .ok_or(LdtkComponentError::BadProjectHandle(project_handle.clone()))?;

            let asset_handle = Self::asset_handle_from_project(project_asset, component.iid())
                .ok_or(LdtkComponentError::HandleNotFound(component.iid()))?;

            let asset = assets
                .get(&asset_handle)
                .ok_or(LdtkComponentError::BadAssetHandle(asset_handle.clone()))?;

            trace!("Calling LdtkComponent::on_spawn(...) for {entity}...");

            Self::on_spawn(
                &mut commands,
                entity,
                project_asset,
                asset,
                component,
                &component_set_query,
            )?;
        }

        Ok(())
    }
}
