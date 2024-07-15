use bevy::prelude::*;

use crate::assets::traits::LdtkAsset;

#[derive(Debug, Event)]
pub(crate) enum LdkAssetEvent<A: LdtkAsset> {
    Modified { entity: Entity, handle: Handle<A> },
}
