use bevy_app::PostUpdate;
use bevy_asset::prelude::AssetChanged;
use bevy_asset::{AsAssetId, Assets, Handle};
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::system::Commands;
use bevy_ecs::system::{Query, Res};
use bevy_ldtk_asset::prelude::LdtkAsset;
use bevy_log::debug;

use super::shieldtank_component::ShieldtankComponent;

#[allow(non_upper_case_globals)]
pub(crate) const ChildSystemSet: PostUpdate = PostUpdate;

pub(crate) trait SpawnChildren: ShieldtankComponent + Sized
where
    <Self as AsAssetId>::Asset: LdtkAsset,
    <<Self as SpawnChildren>::Child as AsAssetId>::Asset: LdtkAsset,
{
    type Child: ShieldtankComponent;

    /// Should only return children which pass its own filter, if any
    fn get_children(
        &self,
        asset: &<Self as AsAssetId>::Asset,
    ) -> impl Iterator<Item = Handle<<Self::Child as AsAssetId>::Asset>>;

    #[allow(clippy::type_complexity)]
    fn child_spawn_system(
        assets: Res<Assets<<Self as AsAssetId>::Asset>>,
        query: Query<(Entity, &Self), Or<(Changed<Self>, AssetChanged<Self>)>>,
        children_query: Query<&Self::Child>,
        mut commands: Commands,
    ) -> bevy_ecs::error::Result<()> {
        let query_inner = |(entity, component): (Entity, &Self)| -> bevy_ecs::error::Result<()> {
            debug!("{entity:?} asset loaded!");

            let Some(asset) = assets.get(component.as_asset_id()) else {
                debug!("asset not ready?");
                return Ok(());
            };

            // TODO: this really breaks rustfmt for some reason...
            let children_assets_inner = |child_handle: Handle<
                <Self::Child as AsAssetId>::Asset,
            >|
             -> bevy_ecs::error::Result<()> {
                if children_query
                    .iter()
                    .any(|child_component| child_component.as_asset_id() == child_handle.id())
                {
                    debug!("child already exists! {child_handle:?}");
                    return Ok(());
                }

                let child_component = Self::Child::new(child_handle);
                let child_id = commands.spawn(child_component).id();
                commands.entity(entity).add_child(child_id);
                debug!("Spawning new child: {child_id:?}");

                Ok(())
            };

            component
                .get_children(asset)
                .try_for_each(children_assets_inner)
        };

        query.iter().try_for_each(query_inner)
    }
}
