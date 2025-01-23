use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shieldtank::component::project::ProjectComponent;
use shieldtank::plugin::ShieldtankPlugins;
use shieldtank::project_config::ProjectConfig;
use shieldtank::query::ShieldtankQuery;

fn main() {
    let log_plugin_settings = bevy::log::LogPlugin {
        level: bevy::log::Level::WARN,
        filter: "wgpu_hal=off,\
                 winit=off,\
                 bevy_winit=off,\
                 bevy_ldtk_asset=debug,\
                 shieldtank=debug,\
                 playground=debug"
            .into(),
        ..default()
    };

    const RESOLUTION: Vec2 = Vec2::new(1280.0, 720.0);

    let window_plugin_settings = WindowPlugin {
        primary_window: Some(Window {
            // mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            mode: WindowMode::Windowed,
            resolution: RESOLUTION.into(),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(log_plugin_settings)
            .set(window_plugin_settings)
            .set(ImagePlugin::default_nearest()),
        ShieldtankPlugins,
        WorldInspectorPlugin::default(),
    ))
    .add_systems(Startup, startup)
    .add_systems(Update, update);

    app.run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("Spawning project...");
    commands.spawn(ProjectComponent {
        handle: asset_server.load("ldtk/axe_man_adventure.ldtk"),
        config: asset_server.add(ProjectConfig::default()),
    });
}

fn update(_shieldtank_query: ShieldtankQuery) {}
