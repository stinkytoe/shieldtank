use bevy::prelude::*;

use super::traits::LdtkAsset;

#[derive(Event, Debug)]
pub(crate) enum LdtkAssetEvent<A>
where
    A: LdtkAsset + Sized,
{
    Spawned { id: AssetId<A> },
}
