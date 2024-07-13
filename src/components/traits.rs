use bevy::prelude::*;

use crate::{
    assets::{event::LdtkAssetEvent, traits::LdtkAsset},
    iid::Iid,
};

pub(crate) trait LdtkComponent<A>
where
    A: LdtkAsset + Sized,
    Self: Component + Sized,
{
    fn iid(&self) -> Iid;

    fn ldtk_asset_event_system(
        events: EventReader<LdtkAssetEvent<A>>,
        query: Query<(Entity, &Self)>,
        assets: Res<Assets<A>>,
    );
}
