use bevy::ecs::query::QueryIter;
use bevy::ecs::system::SystemParam;
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

fn loading(ldtk_query: LdtkQuery, mut next_state: ResMut<NextState<GameState>>) {
    if ldtk_query.all_projects_loaded() {
        next_state.set(GameState::Playing);
    }
}

// fn on_enter_playing(project_query: LdtkProjectQuery) {
// ldtk_entity_query
//     .iter_added()
//     .for_each(|ldtk_entity: LdtkEntity| {
//         debug!("ldtk_entity added: {ldtk_entity:?}");
//     });
//
// ldtk_entity_query
//     .iter()
//     .filter_tag("player")
//     .for_each(|ldtk_entity| {
//         debug!("ldtk_entity with \"player\" tag: {ldtk_entity:?}");
//     });
// }

fn update(
    // words!!!!
    // mut ldtk_commands: LdtkCommands,
    mut commands: Commands,
    ldtk_query: LdtkQuery,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let player = ldtk_query.entities().single_with_identifier("Axe_Man");

    // let level = player.get_level().expect("an ldtk level");
    //
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
            .get_field_instance("Swing")
            .expect("the swing field instance")
            .as_tile()
            .expect("a tile");

        commands.entity(player.entity()).insert(swing_tile.clone());

        // let x = ldtk_commands.ldtk_entity(&player);

        // ldtk_commands
        //     .ldtk_entity(&player)
        //     .set_tile(swing_tile.clone());
    }
}

// #[derive(SystemParam)]
// pub struct LdtkCommands<'w, 's> {
//     _commands: Commands<'w, 's>,
// }

// impl<'a> LdtkCommands<'a, 'a> {
//     pub fn ldtk_entity<'b>(&'a mut self, item: &'b LdtkEntity<'b>) -> LdtkEntityCommands<'b, 'b>
//     where
//         'b: 'a,
//     {
//         LdtkEntityCommands {
//             commands: self,
//             item,
//         }
//     }
// }
//
// pub struct LdtkEntityCommands<'a, 'b> {
//     commands: &'a mut LdtkCommands<'a, 'b>,
//     item: &'a LdtkEntity<'a>,
// }
//
// impl LdtkEntityCommands<'_, '_> {
//     pub fn set_tile(&mut self, _tile: TilesetRectangle) {
//         // self.commands
//         //     ._commands
//         //     .entity(self.item.entity)
//         //     .insert(tile);
//         todo!()
//     }
// }
