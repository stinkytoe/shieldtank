use bevy::prelude::*;
use thiserror::Error;

use crate::assets::project::LdtkProject;
use crate::iid::Iid;
use crate::iid::IidSet;

#[derive(Debug, Error)]
pub enum LdtkAssetError<A>
where
    A: Asset,
{
    #[error("Bad handle? {0:?}")]
    BadHandle(Handle<A>),
    #[error("Bad project iid? {0}")]
    BadProjectIid(Iid),
}

pub trait LdtkAsset
where
    Self: Asset + Sized,
{
    fn iid(&self) -> Iid;
    fn children(&self) -> &IidSet;
    fn asset_handle_from_project(project: &LdtkProject, iid: Iid) -> Option<Handle<Self>>;

    // #[allow(clippy::type_complexity)]
    fn collect_entities(
        commands: &mut Commands,
        project_asset: &LdtkProject,
        iids: &IidSet,
        assets: &Assets<Self>,
        entities_query: &Query<(Entity, &Handle<Self>)>,
    ) -> Result<Vec<(Entity, Handle<Self>)>, LdtkAssetError<Self>> {
        let loaded_entities = iids
            .iter()
            .map(|iid| {
                let asset_handle = Self::asset_handle_from_project(project_asset, *iid)
                    .ok_or(LdtkAssetError::<Self>::BadProjectIid(*iid))?;

                let entity = if let Some((entity, _)) = entities_query
                    .iter()
                    .find(|(_, inner_handle)| asset_handle.id() == inner_handle.id())
                {
                    trace!("keeping already spawned entity: {entity:?}");
                    entity
                } else {
                    let entity = commands
                        .spawn((asset_handle.clone(), SpatialBundle::default()))
                        .id();
                    trace!("spawning new entity: {entity:?}");
                    entity
                };

                Ok((entity, asset_handle.clone()))
            })
            .collect::<Result<Vec<_>, LdtkAssetError<Self>>>()?;

        Ok(loaded_entities)
    }

    fn despawn_orphaned_entities<'a>(
        commands: &mut Commands,
        assets: &Assets<Self>,
        iids: IidSet,
        mut asset_entities: impl Iterator<Item = (Entity, &'a Handle<Self>)>,
    ) -> Result<(), LdtkAssetError<Self>> {
        asset_entities.try_for_each(|(entity, handle)| -> Result<(), LdtkAssetError<Self>> {
            let asset = assets
                .get(handle.id())
                .ok_or(LdtkAssetError::<Self>::BadHandle(handle.clone()))?;

            let iid = asset.iid();

            if !iids.contains(&iid) {
                trace!("despawning orphaned entity: {entity:?}");
                commands.entity(entity).remove_parent();
                commands.entity(entity).despawn();
            }

            Ok(())
        })
    }
}
