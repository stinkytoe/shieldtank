use bevy::prelude::*;
use thiserror::Error;

use crate::iid::Iid;

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
    fn project(&self) -> Entity;

    // fn spawn(&self, commands: &Commands);

    fn asset_response_system(
        mut commands: Commands,
        mut asset_events: EventReader<AssetEvent<Self>>,
        mut assets: ResMut<Assets<Self>>,
        spawned_entities: Query<(Entity, &Handle<Self>)>,
    ) -> Result<(), LdtkAssetError<Self>> {
        for event in asset_events.read() {
            match event {
                // AssetEvent::LoadedWithDependencies { id } => {
                // let handle = assets
                //     .get_strong_handle(*id)
                //     .ok_or(LdtkAssetError::GetStrongHandleFailed(*id))?;
                // let asset = assets.get(&handle).ok_or(LdtkAssetError::BadAssetId(*id))?;
                // let project = projects
                //     .get(asset.project_iid())
                //     .ok_or(LdtkAssetError::BadProjectIid(asset.project_iid()))?;
                //
                // commands.spawn((handle, SpatialBundle::default()));
                // }
                AssetEvent::Added { id } => {}
                AssetEvent::Modified { id: _ } => {}
                AssetEvent::Removed { id } => {}
                // AssetEvent::Unused { id } => {}
                _ => {}
            }
        }

        Ok(())
    }
}
