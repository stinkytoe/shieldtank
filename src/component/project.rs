use bevy_app::Plugin;
use bevy_asset::{AsAssetId, Handle};
use bevy_ecs::component::Component;
use bevy_ldtk_asset::project::Project;
use bevy_ldtk_asset::world::World;
use bevy_reflect::Reflect;
use bevy_render::view::Visibility;
use bevy_transform::components::{GlobalTransform, Transform};

use super::shieldtank_component::{ShieldtankComponent, ShieldtankComponentSystemSet};
use super::spawn_children::SpawnChildren;
use super::world::LdtkWorld;

#[derive(Debug, Default, Reflect)]
pub enum WorldsToSpawn {
    #[default]
    All,
    None,
}

impl WorldsToSpawn {
    pub(crate) fn handle_matches(&self, _handle: Handle<World>) -> bool {
        match self {
            WorldsToSpawn::All => true,
            WorldsToSpawn::None => false,
        }
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[require(GlobalTransform, Transform, Visibility)]
pub struct LdtkProject {
    pub handle: Handle<Project>,
    pub worlds_to_spawn: WorldsToSpawn,
}

impl AsAssetId for LdtkProject {
    type Asset = Project;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

impl ShieldtankComponent for LdtkProject {
    fn new(handle: Handle<<Self as AsAssetId>::Asset>) -> Self {
        Self {
            handle,
            ..Default::default()
        }
    }
}

impl SpawnChildren for LdtkProject {
    type Child = LdtkWorld;

    fn get_children(
        &self,
        asset: &Project,
    ) -> impl Iterator<Item = Handle<<Self::Child as AsAssetId>::Asset>> {
        asset
            .worlds
            .values()
            .filter(|handle| self.worlds_to_spawn.handle_matches(handle.clone_weak()))
            .cloned()
    }
}

pub struct LdtkProjectPlugin;
impl Plugin for LdtkProjectPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LdtkProject>();
        app.add_systems(
            ShieldtankComponentSystemSet,
            <LdtkProject as ShieldtankComponent>::add_basic_components_system,
        );
    }
}
