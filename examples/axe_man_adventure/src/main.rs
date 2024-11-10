//#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::str::FromStr;

use bevy::prelude::*;
use shieldtank::bevy_ldtk_asset::iid::Iid;
use shieldtank::item_iterator::{LdtkItemIterator, LdtkItemUniqueIdentifierIterator};
use shieldtank::plugin::ShieldtankPlugins;
use shieldtank::project_config::ProjectConfig;
use shieldtank::query::LdtkQuery;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(bevy::log::LogPlugin {
                level: bevy::log::Level::WARN,
                filter: "wgpu_hal=off,\
                    winit=off,\
                    bevy_winit=off,\
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
        ShieldtankPlugins,
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
    let axe_man_iid = Iid::from_str("a0170640-9b00-11ef-aa23-11f9c6be2b6e").unwrap();
    let Some(axe_man) = ldtk_query.entities().find_iid(axe_man_iid) else {
        warn!("Failed to get axe_man! Maybe not loaded yet?");
        return;
    };

    let Some(layer) = axe_man.get_layer() else {
        error!("Failed to get axe_man's layer!");
        return;
    };

    let Some(int_grid) = layer.get_int_grid() else {
        error!("Failed to get axe_man's int_grid!");
        return;
    };

    let Some(grid) = axe_man.get_grid_coordinates() else {
        error!("Failed to get axe_man's grid!");
        return;
    };

    let Some(int_grid_value) = int_grid.get(grid) else {
        error!("Failed to get axe_man's int grid value!");
        return;
    };

    debug!("{int_grid_value:?}");
}
