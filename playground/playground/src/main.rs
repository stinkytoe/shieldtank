use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shieldtank::commands::ShieldtankCommands;
use shieldtank::component::project::ProjectComponent;
use shieldtank::plugin::ShieldtankPlugins;
use shieldtank::project_config::ProjectConfig;
use shieldtank::query::ShieldtankQuery;

fn main() {
    println!("Hello, world!");

    let log_plugin_settings = bevy::log::LogPlugin {
        level: bevy::log::Level::WARN,
        filter: "wgpu_hal=off,\
                 winit=off,\
                 bevy_winit=off,\
                 bevy_ldtk_asset=debug,\
                 shieldtank2=debug,\
                 playground=debug"
            .into(),
        ..default()
    };

    let window_plugin_settings = WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            resolution: WindowResolution::new(800.0, 600.0),
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
    .add_systems(Startup, test_system);

    app.add_systems(
        Update,
        (new_shieldtank_component, new_shieldtank_component2),
    );

    app.run();
}

fn test_system(
    mut commands: Commands,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(ProjectComponent {
        handle: asset_server.load("ldtk/axe_man_adventure.ldtk"),
        config: asset_server.add(ProjectConfig::default()),
    });

    shieldtank_query.iter_entities().for_each(|item| {
        let transform = item.get_transform();

        shieldtank_commands.entity(&item).test(2).test(3).test(4);
    });
}

pub fn new_shieldtank_component(shieldtank_query: ShieldtankQuery) {
    shieldtank_query.iter_projects().for_each(|item| {
        let component = item.get_component();

        let tuple = (component.is_added(), component.is_changed());

        if tuple.0 || tuple.1 {
            println!("shieldtank_query: {tuple:?}");
        }
    });
}

pub fn new_shieldtank_component2(
    query: Query<Ref<ProjectComponent>>,
    asset_server: Res<AssetServer>,
) {
    query.iter().for_each(|component| {
        let tuple = (component.is_added(), component.is_changed());
        let load_state = asset_server.get_load_state(component.handle.id());

        if tuple.0 || tuple.1 {
            println!("bevy query: {tuple:?}");
            println!("load_state: {load_state:?}");
        }
    });
}
