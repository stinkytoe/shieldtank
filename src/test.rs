#[test]
fn do_test() {}

use bevy_app::{App, Startup, TaskPoolPlugin};
use bevy_asset::io::embedded::GetAssetServer as _;
use bevy_asset::{AssetPlugin, AssetServer, LoadState};
use bevy_ecs::system::{Commands, Res};
use bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin;
use bevy_ldtk_asset::world::World as WorldAsset;

use crate::component::ShieldtankComponent;
use crate::component::world::ShieldtankWorld;

fn spawn_a_world(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle = asset_server.load::<WorldAsset>("ldtk/empty_single_world.ldtk#world:World");

    commands.spawn(ShieldtankWorld::new(handle));
}

#[test]
fn test_world() {
    let mut bevy_app = App::new();

    bevy_app.add_plugins(TaskPoolPlugin::default());
    bevy_app.add_plugins(AssetPlugin::default());
    bevy_app.add_plugins(BevyLdtkAssetPlugin);
    bevy_app.add_systems(Startup, spawn_a_world);
    bevy_app.update();

    let bevy_asset_server = bevy_app.get_asset_server().clone();

    let handle = bevy_asset_server.load::<WorldAsset>("ldtk/empty_single_world.ldtk#world:World");

    while let Some(LoadState::Loading) = bevy_asset_server.get_load_state(handle.id()) {
        bevy_app.update();
    }

    assert!(matches!(
        bevy_asset_server.get_load_state(handle.id()),
        Some(LoadState::Loaded)
    ));
}
