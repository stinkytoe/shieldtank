use bevy_app::PostUpdate;
use bevy_asset::prelude::AssetChanged;
use bevy_asset::{AsAssetId, Assets, Handle};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::name::Name;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::{Commands, Query, Res};
use bevy_ldtk_asset::prelude::LdtkAsset;

use super::iid::ShieldtankIid;

#[allow(non_upper_case_globals)]
pub(crate) const ShieldtankComponentSystemSet: PostUpdate = PostUpdate;

pub(crate) trait ShieldtankComponent: Component + AsAssetId
where
    Self: Sized,
    <Self as AsAssetId>::Asset: LdtkAsset,
{
    fn new(handle: Handle<<Self as AsAssetId>::Asset>) -> Self;

    fn name(&self, asset: &<Self as AsAssetId>::Asset) -> Name {
        Name::new(asset.get_identifier().to_string())
    }

    fn iid(&self, asset: &<Self as AsAssetId>::Asset) -> ShieldtankIid {
        ShieldtankIid::new(asset.get_iid())
    }

    #[allow(clippy::type_complexity)]
    fn add_basic_components_system(
        query: Query<(Entity, &Self), Or<(Changed<Self>, AssetChanged<Self>)>>,
        assets: Res<Assets<<Self as AsAssetId>::Asset>>,
        mut commands: Commands,
    ) {
        query
            .iter()
            .filter_map(|(entity, component)| {
                Some((entity, component, assets.get(component.as_asset_id())?))
            })
            .for_each(|(entity, component, asset)| {
                let name = component.name(asset);
                let iid = component.iid(asset);
                commands.entity(entity).insert((name, iid));
            });
    }
}
