use bevy::log::Level;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use shieldtank::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

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
            ShieldTankPlugin,
        ))
        .init_state::<GameState>()
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, loading.run_if(in_state(GameState::Loading)))
        .add_systems(Update, update.run_if(in_state(GameState::Playing)))
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn loading(project_commands: LdtkProjectQuery, mut next_state: ResMut<NextState<GameState>>) {
    if project_commands.all_projects_loaded() {
        next_state.set(GameState::Playing);
    }
}

fn update(
    mut ldtk_commands: LdtkCommands,
    ldtk_entity_query: LdtkEntityQuery,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let player: LdtkEntity = ldtk_entity_query.single_with_identifier("Axe_Man");

    if keys.just_pressed(KeyCode::Space) {
        debug!("space pressed!");

        let swing_tile = player
            .get_field_instance("Swing")
            .expect("the swing field instance")
            .as_tile()
            .expect("a tile");

        let grid = player.grid();
        debug!("{grid}");

        ldtk_commands
            .ldtk_entity(&player)
            .set_tile(swing_tile.clone());
    }
}
