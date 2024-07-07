use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use shieldtank::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::WARN,
                    filter: "shieldtank=trace,example=trace".into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            // CoveyOfWorldsPlugin,
            ShieldTankPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut project_commands: LdtkProjectCommands,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            // good scale for a 1920x1080 canvas/window
            scale: Vec3::splat(0.3),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        asset_server.load::<LdtkProject>("ldtk/top_down.ldtk"),
        SpatialBundle::default(),
    ));
}

fn update() {}
