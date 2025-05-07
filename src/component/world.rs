use bevy_app::Plugin;
use bevy_asset::{AsAssetId, Handle};
use bevy_ecs::component::Component;
use bevy_ldtk_asset::world::World;
use bevy_reflect::Reflect;
use bevy_render::view::Visibility;
use bevy_transform::components::{GlobalTransform, Transform};

use super::{
    level::LdtkLevel,
    shieldtank_component::{ShieldtankComponent, ShieldtankComponentSystemSet},
    spawn_children::SpawnChildren,
};

#[derive(Debug, Default, Reflect)]
pub enum LevelsToSpawn {
    #[default]
    All,
    None,
}

impl LevelsToSpawn {
    fn handle_matches(&self) -> bool {
        match self {
            LevelsToSpawn::All => true,
            LevelsToSpawn::None => false,
        }
    }
}

#[derive(Debug, Default, Component, Reflect)]
#[require(GlobalTransform, Transform, Visibility)]
pub struct LdtkWorld {
    pub handle: Handle<World>,
    pub levels_to_spawn: LevelsToSpawn,
}

impl AsAssetId for LdtkWorld {
    type Asset = World;

    fn as_asset_id(&self) -> bevy_asset::AssetId<Self::Asset> {
        self.handle.id()
    }
}

impl ShieldtankComponent for LdtkWorld {
    fn new(handle: Handle<<Self as AsAssetId>::Asset>) -> Self {
        Self {
            handle,
            ..Default::default()
        }
    }
}

impl SpawnChildren for LdtkWorld {
    type Child = LdtkLevel;

    fn get_children(
        &self,
        asset: &<Self as AsAssetId>::Asset,
    ) -> impl Iterator<Item = Handle<<Self::Child as AsAssetId>::Asset>> {
        asset
            .levels
            .values()
            .filter(|_| self.levels_to_spawn.handle_matches())
            .cloned()
    }
}

pub struct LdtkWorldPlugin;
impl Plugin for LdtkWorldPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.register_type::<LdtkWorld>();
        app.add_systems(
            ShieldtankComponentSystemSet,
            <LdtkWorld as ShieldtankComponent>::add_basic_components_system,
        );
    }
}
