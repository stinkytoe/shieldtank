use bevy_app::Plugin;
use bevy_asset::{AsAssetId, Handle};
use bevy_camera::visibility::Visibility;
use bevy_ecs::component::Component;
use bevy_ldtk_asset::project::Project;
use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_reflect::Reflect;
use bevy_transform::components::{GlobalTransform, Transform};

use super::shieldtank_component::{ShieldtankComponent, ShieldtankComponentSystemSet};
use super::spawn_children::SpawnChildren;
use super::world::ShieldtankWorld;

#[derive(Debug, Default, Component, Reflect)]
#[require(GlobalTransform, Transform, Visibility)]
pub struct LdtkProject {
    pub handle: Handle<Project>,
}

impl AsAssetId for LdtkProject {
    type Asset = Project;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

impl ShieldtankComponent for LdtkProject {
    fn new(handle: Handle<<Self as AsAssetId>::Asset>) -> Self {
        Self { handle }
    }
}

#[derive(Clone, Debug, Default, Component, Reflect)]
pub struct ShieldtankProjectFilter;

impl SpawnChildren for LdtkProject {
    type Child = ShieldtankWorld;

    fn get_children(&self, asset: &Project) -> impl Iterator<Item = Handle<WorldAsset>> {
        asset.worlds.values().cloned()
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
