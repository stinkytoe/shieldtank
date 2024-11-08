//#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use bevy::prelude::*;
use itertools::Itertools;
use shieldtank::item::LdtkItemIteratorExt;
use shieldtank::level::LevelItemIteratorExt;
use shieldtank::query::LdtkQuery;
use shieldtank::{plugin::ShieldtankPlugin, project_config::ProjectConfig};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(bevy::log::LogPlugin {
                level: bevy::log::Level::WARN,
                filter: "wgpu_hal=off,\
                    winit=off,\
                    bevy_ldtk_asset=debug,\
                    shieldtank=debug,\
                    axe_man_adventure=trace"
                    .into(),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        //WorldInspectorPlugin::default(),
        //BevyLdtkAssetPlugin,
        ShieldtankPlugin,
    ))
    .add_systems(Startup, startup)
    .add_systems(Update, update);

    app.run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut _project_configs: ResMut<Assets<ProjectConfig>>,
) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec2::splat(0.4).extend(1.0)),
    ));

    commands.spawn(shieldtank::world::World {
        handle: asset_server.load("ldtk/axe_man_adventure.ldtk#World"),
        config: asset_server.load("config/example.project_config.ron"),
    });
}

fn update(ldtk_query: LdtkQuery) {
    let Ok(Some(_axe_man)) = ldtk_query
        .entities()
        .filter_identifier("Axe_Man")
        .at_most_one()
    else {
        return;
    };

    //let Ok(Some(level)) = ldtk_query
    //    .levels()
    //    .contains_point(axe_man.get_global_location())
    //    .at_most_one()
    //else {
    //    return;
    //};

    //let axe_man_location = axe_man.get_grid();
}
