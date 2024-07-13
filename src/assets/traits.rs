use bevy::prelude::*;
use thiserror::Error;

use crate::iid::{Iid, IidSet};

#[derive(Debug, Error)]
pub enum LdtkAssetError<A>
where
    A: Asset,
{
    #[error("Bad asset id? {0}")]
    BadAssetId(AssetId<A>),
    #[error("Couldn't get strong handle? {0}")]
    GetStrongHandleFailed(AssetId<A>),
    #[error("Bad handle? {0}")]
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

    fn asset_event_system(
        mut commands: Commands,
        mut asset_events: EventReader<AssetEvent<Self>>,
        mut assets: ResMut<Assets<Self>>,
        spawned_entities: Query<(Entity, &Handle<Self>)>,
    ) -> Result<(), LdtkAssetError<Self>> {
        for event in asset_events.read() {
            match event {
                AssetEvent::Added { id } => {
                    trace!("LdtkAsset Added: {id:?}")
                }
                AssetEvent::Modified { id } => {
                    trace!("LdtkAsset Modified: {id:?}")
                }
                AssetEvent::Removed { id } => {
                    trace!("LdtkAsset Removed: {id:?}")
                }
                AssetEvent::Unused { id } => {
                    trace!("LdtkAsset Unused: {id:?}")
                }
                AssetEvent::LoadedWithDependencies { id } => {
                    trace!("LdtkAsset LoadedWithDependencies: {id:?}")
                }
            }
        }

        Ok(())
    }
}
