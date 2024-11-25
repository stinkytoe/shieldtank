mod actor;
mod animation;
mod message_board;
mod player;

use bevy::color::palettes::tailwind::GRAY_500;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use shieldtank::bevy_ldtk_asset::iid::{iid, Iid};
use shieldtank::item_iterator::LdtkItemIterator;
use shieldtank::plugin::ShieldtankPlugins;
use shieldtank::query::LdtkQuery;

const AXE_MAN_IID: Iid = iid!("a0170640-9b00-11ef-aa23-11f9c6be2b6e");

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
                    resolution: WindowResolution::new(800.0, 600.0),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        ShieldtankPlugins,
    ))
    .insert_resource(animation::GlobalAnimationTimer {
        timer: Timer::from_seconds(0.250, TimerMode::Repeating),
        frame: 0,
    })
    .add_event::<animation::GlobalAnimationEvent>()
    .add_event::<animation::ActorAnimationEvent>()
    .add_event::<actor::ActorAttemptMoveEvent>()
    .add_event::<message_board::MessageBoardEvent>()
    .add_event::<player::PlayerInteractEvent>()
    .init_state::<GameState>()
    // Always
    .add_systems(Startup, startup)
    .add_systems(
        Update,
        (
            animation::animate_actor,
            animation::animate_water,
            animation::update_global_animation_timer,
            message_board::update_message_board,
        ),
    )
    // WaitingOnPlayer
    .add_systems(
        Update,
        wait_on_player_spawn.run_if(in_state(GameState::WaitingOnPlayer)),
    )
    // Playing
    .add_systems(OnEnter(GameState::Playing), actor::install_actor_components)
    .add_systems(
        Update,
        (
            actor::actor_attempt_move,
            actor::actor_moving,
            player::keyboard_input,
            player::player_interaction,
        )
            .run_if(in_state(GameState::Playing)),
    );

    app.run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[cfg(target_arch = "wasm32")]
    let scale = 0.6;

    #[cfg(not(target_arch = "wasm32"))]
    let scale = 0.4;

    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec2::splat(scale).extend(1.0))
            .with_translation((0.0, -128.0, 1000.0).into()),
    ));

    commands.spawn(shieldtank::world::WorldComponent {
        handle: asset_server.load("ldtk/axe_man_adventure.ldtk#worlds:World"),
        config: asset_server.load("config/example.project_config.ron"),
    });

    commands.spawn((
        Text::new("The Axe Man begins his adventure!"),
        TextFont {
            font: asset_server.load("fonts/Primitive.ttf"),
            font_size: 30.0,
            ..Default::default()
        },
        TextColor(GRAY_500.into()),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        message_board::MessageBoard,
    ));
}

fn wait_on_player_spawn(
    //mut player_res: ResMut<Player>,
    mut next_state: ResMut<NextState<GameState>>,
    ldtk_query: LdtkQuery,
) {
    // TODO: Should we actually do asset load detection here?
    if let Some(_player_item) = ldtk_query.entities().find_iid(AXE_MAN_IID) {
        next_state.set(GameState::Playing);
        info!("Axe man spawned!");
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
enum GameState {
    #[default]
    WaitingOnPlayer,
    Playing,
    GameOver,
}
