use bevy_app::{Plugin, PostUpdate};
use bevy_asset::prelude::AssetChanged;
use bevy_asset::{AsAssetId, Assets, Handle};
use bevy_ecs::entity::Entity;
use bevy_ecs::hierarchy::Children;
use bevy_ecs::query::{Changed, Or};
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_ecs::system::Commands;
use bevy_ecs::system::{Query, Res};
use bevy_ldtk_asset::prelude::LdtkAsset;
use bevy_log::debug;

use super::layer::ShieldtankLayer;
use super::level::ShieldtankLevel;
use super::project::LdtkProject;
use super::shieldtank_component::ShieldtankComponent;
use super::world::ShieldtankWorld;

#[allow(non_upper_case_globals)]
pub(crate) const ChildSystemSet: PostUpdate = PostUpdate;

pub(crate) trait SpawnChildren: ShieldtankComponent + Sized + std::fmt::Debug
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
        query: Query<(Entity, &Self, Option<&Children>), Or<(Changed<Self>, AssetChanged<Self>)>>,
        children_query: Query<&Self::Child>,
        mut commands: Commands,
    ) {
        query.iter().for_each(|(entity, component, children)| {
            let spawned_children = match children {
                Some(children) => children
                    .into_iter()
                    .copied()
                    .filter_map(|entity| children_query.get(entity).ok())
                    .map(|child| child.as_asset_id())
                    .collect(),
                None => vec![],
            };

            let Some(asset) = assets.get(component.as_asset_id()) else {
                debug!("asset not ready?");
                return;
            };

            component.get_children(asset).for_each(|child_handle| {
                if !spawned_children.contains(&child_handle.id()) {
                    let child_component = Self::Child::new(child_handle.clone());
                    let child_id = commands.spawn(child_component).id();
                    commands.entity(entity).add_child(child_id);
                    debug!("Spawning new child: {child_handle:?}");
                }
            });
        });
    }
}

pub struct SpawnChildrenPlugin;
impl Plugin for SpawnChildrenPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            ChildSystemSet,
            (
                <LdtkProject as SpawnChildren>::child_spawn_system,
                <ShieldtankWorld as SpawnChildren>::child_spawn_system,
                <ShieldtankLevel as SpawnChildren>::child_spawn_system,
                <ShieldtankLayer as SpawnChildren>::child_spawn_system,
            )
                .chain(),
        );
    }
}
