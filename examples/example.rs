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
                    filter: "shieldtank=debug,example=trace".into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            ShieldTankPlugin,
        ))
        .init_state::<GameState>()
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, loading.run_if(in_state(GameState::Loading)))
        // .add_systems(OnEnter(GameState::Playing), on_enter_playing)
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

fn loading(projects_query: LdtkProjectsQuery, mut next_state: ResMut<NextState<GameState>>) {
    if projects_query.all_projects_loaded() {
        next_state.set(GameState::Playing);
    }
}

fn update(
    mut entity_commands: LdtkEntityCommands,
    entities_query: LdtkEntitiesQuery,
    levels_query: LdtkLevelsQuery,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(player) = entities_query.get_single_with_identifier("Axe_Man") else {
        return;
    };

    // let level = player.get_level().expect("an ldtk level");

    // let attempt_move = player.location().grid_move(IVec2::new(1, 0));
    //
    // let int_grid_value = level.get_int_grid_value(attempt_move);
    //
    // if int_grid_value.identifier != "water" {
    //     ldtk_commands.ldtk_entity(player).move(attempt_move);
    // }

    if keys.just_pressed(KeyCode::Space) {
        debug!("space pressed!");

        let swing_tile = player
            .field_instance("Swing")
            .expect("the swing field instance")
            .as_tile()
            .expect("a tile");
        //
        // commands.entity(player.entity()).insert(swing_tile.clone());
        entity_commands.set_tile(&player, swing_tile.clone());
    }
}

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct LdtkEntityCommands<'w, 's> {
    commands: Commands<'w, 's>,
}

impl LdtkEntityCommands<'_, '_> {
    pub fn set_tile(&mut self, entity: &LdtkEntity, tile: TilesetRectangle) {
        self.commands.entity(entity.ecs_entity()).insert(tile);
    }
}
