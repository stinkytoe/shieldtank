//#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use bevy::prelude::*;
use bevy_ldtk_asset::plugin::BevyLdtkAssetPlugin;
use shieldtank::{plugin::ShieldtankPlugin, project_config::ProjectConfig};

fn main() {
    let mut app = App::new();

    //{
    //    let default_project_config = ProjectConfig::default();
    //    let default_ron_string =
    //        ron::ser::to_string_pretty(&default_project_config, ron::ser::PrettyConfig::default())
    //            .unwrap();
    //    println!("{default_ron_string}");
    //}

    app.add_plugins((
        DefaultPlugins
            .set(bevy::log::LogPlugin {
                level: bevy::log::Level::WARN,
                filter: "wgpu_hal=off,\
                    winit=off,\
                    bevy_ldtk_asset=debug,\
                    shieldtank=debug,\
                    example=trace"
                    .into(),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        //WorldInspectorPlugin::default(),
        BevyLdtkAssetPlugin,
        ShieldtankPlugin,
    ))
    .add_systems(Startup, startup);

    app.run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut _project_configs: ResMut<Assets<ProjectConfig>>,
) {
    commands.spawn((
        Camera2d,
        // good scale for a 1920x1080 canvas/window
        Transform::from_scale(Vec2::splat(0.7).extend(1.0)),
    ));

    //commands.spawn(shieldtank::project::Project {
    //    handle: asset_server.load("ldtk/top_down.ldtk"),
    //    config: asset_server.load("config/example.project_config.ron"),
    //});

    commands.spawn(shieldtank::world::World {
        handle: asset_server.load("ldtk/top_down.ldtk#World"),
        config: asset_server.load("config/example.project_config.ron"),
    });

    //commands.spawn(shieldtank::level::Level {
    //    handle: asset_server.load("ldtk/top_down.ldtk#World/Island_of_Thieves"),
    //    config: asset_server.load("config/example.project_config.ron"),
    //});
}
