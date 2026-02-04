use bevy_app::Plugin;
use bevy_asset::{AsAssetId, Handle};
use bevy_camera::visibility::Visibility;
use bevy_ecs::component::Component;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_reflect::Reflect;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::component::filter::ShieldtankComponentFilter;
use crate::component::level::ShieldtankLevel;
use crate::component::shieldtank_component::{ShieldtankComponent, ShieldtankComponentSystemSet};
use crate::component::spawn_children::SpawnChildren;

#[derive(Debug, Default, Component, Reflect)]
#[require(GlobalTransform, Transform, Visibility)]
pub struct ShieldtankWorld {
    pub handle: Handle<WorldAsset>,
}

impl AsAssetId for ShieldtankWorld {
    type Asset = WorldAsset;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

impl ShieldtankComponent for ShieldtankWorld {
    fn new(handle: Handle<<Self as AsAssetId>::Asset>) -> Self {
        Self { handle }
    }
}

#[derive(Clone, Debug, Default, Component, Reflect)]
pub struct ShieldtankWorldFilter;

impl ShieldtankComponentFilter for ShieldtankWorldFilter {}

impl SpawnChildren for ShieldtankWorld {
    type Child = ShieldtankLevel;
    type Filter = ShieldtankWorldFilter;

    fn get_children(
        &self,
        asset: &WorldAsset,
        _filter: ShieldtankWorldFilter,
    ) -> impl Iterator<Item = Handle<LevelAsset>> {
        asset.levels.values().cloned()
    }
}

pub struct ShieldtankWorldPlugin;
impl Plugin for ShieldtankWorldPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<ShieldtankWorld>();
        app.add_systems(
            ShieldtankComponentSystemSet,
            <ShieldtankWorld as ShieldtankComponent>::add_basic_components_system,
        );
    }
}
