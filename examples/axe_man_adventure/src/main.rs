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
            player::keyboard_input,
            actor::actor_attempt_move,
            actor::actor_moving,
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

//
//#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
//enum GameState {
//    #[default]
//    WaitingOnPlayer,
//    Playing,
//    GameOver,
//}
//
//
//#[derive(Debug, Event)]
//struct PlayerMoveEvent {
//    actor_facing: ActorFacing,
//}
//
//#[derive(Debug, Event)]
//struct ActorAnimateEvent;
//
//#[derive(Clone, Copy, Debug)]
//enum ActorFacing {
//    North,
//    East,
//    South,
//    West,
//}
//
//impl ActorFacing {
//    const NORTH_I64VEC2: I64Vec2 = I64Vec2::new(0, -1);
//    const EAST_I64VEC2: I64Vec2 = I64Vec2::new(1, 0);
//    const SOUTH_I64VEC2: I64Vec2 = I64Vec2::new(0, 1);
//    const WEST_I64VEC2: I64Vec2 = I64Vec2::new(0, -1);
//
//    const LDTK_TO_BEVY: Vec2 = Vec2::new(1.0, -1.0);
//
//    fn as_global_move_offset(&self, grid_cell_size: i64) -> Vec2 {
//        Self::LDTK_TO_BEVY
//            * match self {
//                ActorFacing::North => Self::NORTH_I64VEC2 * grid_cell_size,
//                ActorFacing::East => Self::EAST_I64VEC2 * grid_cell_size,
//                ActorFacing::South => Self::SOUTH_I64VEC2 * grid_cell_size,
//                ActorFacing::West => Self::WEST_I64VEC2 * grid_cell_size,
//            }
//            .as_vec2()
//    }
//}
//
//#[derive(Debug)]
//enum ActorState {
//    Alive,
//    Dead,
//}
//
//#[derive(Debug, Component)]
//struct ActorAnimationState {
//    facing: ActorFacing,
//    state: ActorState,
//}
//
//macro_rules! post_to_billboard {
//    ($board:expr, $($message:tt)*) => {
//        $board.send(MessageBoardEvent(format!($($message)*)))
//    };
//}
//
//#[derive(Debug, PartialEq, Eq)]
//enum PlayerAction {
//    MoveNorth,
//    MoveEast,
//    MoveSouth,
//    MoveWest,
//    Interact,
//}
//
//impl PlayerAction {
//    fn from_keyboard_input(keyboard_input: &ButtonInput<KeyCode>) -> Option<Self> {
//        let move_north = keyboard_input.just_pressed(KeyCode::ArrowUp)
//            | keyboard_input.just_pressed(KeyCode::KeyW);
//        let move_east = keyboard_input.just_pressed(KeyCode::ArrowRight)
//            | keyboard_input.just_pressed(KeyCode::KeyD);
//        let move_south = keyboard_input.just_pressed(KeyCode::ArrowDown)
//            | keyboard_input.just_pressed(KeyCode::KeyS);
//        let move_west = keyboard_input.just_pressed(KeyCode::ArrowLeft)
//            | keyboard_input.just_pressed(KeyCode::KeyA);
//        let interact = keyboard_input.just_pressed(KeyCode::Space);
//
//        match (move_north, move_east, move_south, move_west, interact) {
//            (true, false, false, false, false) => Some(PlayerAction::MoveNorth),
//            (false, true, false, false, false) => Some(PlayerAction::MoveEast),
//            (false, false, true, false, false) => Some(PlayerAction::MoveSouth),
//            (false, false, false, true, false) => Some(PlayerAction::MoveWest),
//            (false, false, false, false, true) => Some(PlayerAction::Interact),
//            _ => None,
//        }
//    }
//
//    //impl PlayerAction {
//    //    fn to_move_attempt(&self) -> Option<(ActorFacing, I64Vec2)> {
//    //        match self {
//    //            PlayerAction::MoveNorth => Some((ActorFacing::North, I64Vec2::new(0, -1))),
//    //            PlayerAction::MoveEast => Some((ActorFacing::East, I64Vec2::new(1, 0))),
//    //            PlayerAction::MoveSouth => Some((ActorFacing::South, I64Vec2::new(0, 1))),
//    //            PlayerAction::MoveWest => Some((ActorFacing::West, I64Vec2::new(-1, 0))),
//    //            PlayerAction::Interact => None,
//    //        }
//    //    }
//}
//
//fn main() {
//    let mut app = App::new();
//
//    app.add_plugins((
//        DefaultPlugins
//            .set(bevy::log::LogPlugin {
//                level: bevy::log::Level::WARN,
//                filter: "wgpu_hal=off,\
//                    winit=off,\
//                    bevy_winit=off,\
//                    bevy_ldtk_asset=debug,\
//                    shieldtank=debug,\
//                    axe_man_adventure=trace"
//                    .into(),
//                ..default()
//            })
//            .set(ImagePlugin::default_nearest())
//            .set(WindowPlugin {
//                primary_window: Some(Window {
//                    mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
//                    resolution: WindowResolution::new(800.0, 600.0),
//                    ..Default::default()
//                }),
//                ..Default::default()
//            }),
//        ShieldtankPlugins,
//    ))
//    .insert_resource(AnimationTimer(Timer::from_seconds(
//        0.250,
//        TimerMode::Repeating,
//    )))
//    //.init_resource::<Player>()
//    .init_state::<GameState>()
//    .add_event::<MessageBoardEvent>()
//    .add_event::<PlayerMoveEvent>()
//    .add_systems(OnEnter(GameState::WaitingOnPlayer), startup)
//    .add_systems(
//        Update,
//        wait_on_player_spawn.run_if(in_state(GameState::WaitingOnPlayer)),
//    )
//    .add_systems(OnEnter(GameState::Playing), install_animations)
//    .add_systems(
//        Update,
//        (
//            player_animation,
//            player_movement,
//            player_action,
//            update_message_board,
//        )
//            .run_if(in_state(GameState::Playing)),
//    )
//    .add_systems(
//        Update,
//        (animate_entity, update_animation_timer, animate_water),
//    )
//    .add_systems(OnEnter(GameState::GameOver), you_died);
//
//    app.run();
//}
//
//fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
//    #[cfg(target_arch = "wasm32")]
//    let scale = 0.6;
//
//    #[cfg(not(target_arch = "wasm32"))]
//    let scale = 0.4;
//
//    commands.spawn((
//        Camera2d,
//        Transform::from_scale(Vec2::splat(scale).extend(1.0))
//            .with_translation((0.0, -128.0, 1000.0).into()),
//    ));
//
//    commands.spawn(shieldtank::world::WorldComponent {
//        handle: asset_server.load("ldtk/axe_man_adventure.ldtk#worlds:World"),
//        config: asset_server.load("config/example.project_config.ron"),
//    });
//
//    commands.spawn((
//        Text::new("The Axe Man begins his adventure!"),
//        TextFont {
//            font: asset_server.load("fonts/Primitive.ttf"),
//            font_size: 30.0,
//            ..Default::default()
//        },
//        TextColor(GRAY_500.into()),
//        TextLayout::new_with_justify(JustifyText::Center),
//        Node {
//            position_type: PositionType::Absolute,
//            bottom: Val::Px(20.0),
//            left: Val::Px(5.0),
//            right: Val::Px(5.0),
//            ..default()
//        },
//        MessageBoard,
//    ));
//}
//
//fn wait_on_player_spawn(
//    //mut player_res: ResMut<Player>,
//    mut next_state: ResMut<NextState<GameState>>,
//    ldtk_query: LdtkQuery,
//) {
//    //if let Some(player_item) = ldtk_query.entities().find_iid(AXE_MAN_IID) {
//    //    *player_res = Player(Some(player_item.get_ecs_entity()));
//    next_state.set(GameState::Playing);
//    //
//    //    info!("Axe man spawned!");
//    //}
//}
//
//fn install_animations(mut commands: Commands, ldtk_query: LdtkQuery) {
//    //ldtk_query.entities().for_each(|item| {
//    //    commands
//    //        .entity(item.get_ecs_entity())
//    //        .insert(ActorAnimation {
//    //            facing: ActorFacing::East,
//    //            state: ActorState::Alive,
//    //        });
//    //});
//}
//
//fn player_action(
//    keyboard_input: Res<ButtonInput<KeyCode>>,
//    mut player_move_events: EventWriter<PlayerMoveEvent>,
//) {
//    let Some(player_action) = PlayerAction::from_keyboard_input(&keyboard_input) else {
//        return;
//    };
//
//    debug!("The Axe Man has performed an action! {player_action:?}");
//
//    match player_action {
//        PlayerAction::MoveNorth => player_move_events.send(PlayerMoveEvent {
//            actor_facing: ActorFacing::North,
//        }),
//        PlayerAction::MoveEast => todo!(),
//        PlayerAction::MoveSouth => todo!(),
//        PlayerAction::MoveWest => todo!(),
//        PlayerAction::Interact => todo!(),
//    };
//
//    //if let Some((actor_facing, move_direction)) = player_action.to_move_attempt() {
//    //    //if let Some(move_direction) = attempted_move {
//    //    player_move_events.send(PlayerMoveEvent {
//    //        actor_facing,
//    //        move_direction,
//    //    });
//    //}
//}
//
//fn player_animation(
//    mut player_move_events: EventReader<PlayerMoveEvent>,
//    //player: Res<Player>,
//    mut player_query: Query<&mut ActorAnimationState>,
//) {
//    //for event in player_move_events.read() {
//    //    let PlayerMoveEvent { actor_facing, .. } = event;
//    //
//    //    let mut axe_man_actor_animation = player_query.get_mut(player.0.unwrap()).unwrap();
//    //
//    //    axe_man_actor_animation.facing = *actor_facing;
//    //}
//}
//
//fn player_movement(
//    mut player_move_events: EventReader<PlayerMoveEvent>,
//    mut message_board_writer: EventWriter<MessageBoardEvent>,
//    //player: Res<Player>,
//    ldtk_query: LdtkQuery,
//    mut ldtk_commands: LdtkCommands,
//    mut next_state: ResMut<NextState<GameState>>,
//) {
//    for PlayerMoveEvent { actor_facing } in player_move_events.read() {
//        let Some(axe_man) = ldtk_query.entities().find_iid(AXE_MAN_IID) else {
//            return;
//        };
//
//        let Some((layer, level, world)) = axe_man.get_layer_level_world() else {
//            error!("could not get_layer_level_world()!");
//            return;
//        };
//
//        let Some(global_location) = axe_man.get_global_location() else {
//            return;
//        };
//
//        let offset = actor_facing.as_global_move_offset(layer.get_grid_cell_size());
//
//        let move_location = global_location + offset;
//
//        if let Some(entity_at_move_location) = ldtk_query
//            .entities()
//            .filter_global_location(move_location)
//            .next()
//        {
//            // Who did we bump into?
//            if entity_at_move_location.has_tag("Enemy") {
//                debug!(
//                    "The Axe Man has bumped into an enemy {}!",
//                    entity_at_move_location.get_identifier()
//                );
//            }
//
//            //    let offset = (layer.get_grid_cell_size() * move_direction * I64Vec2::new(1, -1)).as_vec2();
//            //
//            //    let attempted_move_location = global_location + offset;
//            //
//            //    if let Some(entity_at_move_location) = ldtk_query
//            //        .entities()
//            //        .filter_global_location(attempted_move_location)
//            //        .next()
//            //    {
//            //        if entity_at_move_location.has_tag("Enemy") {
//            //            debug!(
//            //                "The Axe Man has bumped into an enemy {}!",
//            //                entity_at_move_location.get_identifier()
//            //            );
//            //
//            //            //ldtk_commands
//            //            //    .entity(&axe_man)
//            //            //    .set_tile_to_field_instance("Dead");
//            //            //
//            //            //ldtk_commands
//            //            //    .entity(&entity_at_move_location)
//            //            //    .set_tile_to_field_instance("Stab");
//            //
//            //            next_state.set(GameState::GameOver);
//            //
//            //            post_to_billboard!(
//            //                message_board_writer,
//            //                "Our hero, The Axe Man, was slain by the vile Green Lancer!"
//            //            );
//            //        } else {
//            //            // NPC Bump!
//            //        };
//            //
//            //        return;
//        }
//
//        let Some(int_grid_value) = world.int_grid_value_at_global_location(move_location) else {
//            return;
//        };
//
//        let Some(int_grid_identifier) = int_grid_value.identifier else {
//            return;
//        };
//
//        let terrain_is_movable = match int_grid_identifier.as_str() {
//            "bridge" => {
//                post_to_billboard!(
//                    message_board_writer,
//                    "The Axe Man is crossing the Bridge of Woe!"
//                );
//                true
//            }
//            "grass" | "dirt" => {
//                let level_name = level.get_field_string("Name").unwrap_or("unknown land");
//
//                post_to_billboard!(
//                    message_board_writer,
//                    "The Axe Man is walking on {} on the {}!",
//                    int_grid_identifier,
//                    level_name
//                );
//                true
//            }
//            "water" => {
//                post_to_billboard!(
//                    message_board_writer,
//                    "The Axe Man, though virtuous, is just a man and cannot walk on water!"
//                );
//                false
//            }
//            _ => {
//                post_to_billboard!(
//                    message_board_writer,
//                    "The Axe Man is refusing to walk on some dubious unknown terrain! {}",
//                    int_grid_identifier
//                );
//                false
//            }
//        };
//
//        if terrain_is_movable {
//            let Some(axe_man_transform) = axe_man.get_transform() else {
//                return;
//            };
//
//            //let offset = (move_direction * layer.get_asset().grid_cell_size * I64Vec2::new(1, -1))
//            //    .as_vec2()
//            //    .extend(0.0);
//
//            //let new_translation = axe_man_transform.translation + offset;
//            let Some(layer_location) = axe_man.get_layer_local_location() else {
//                return;
//            };
//
//            ldtk_commands
//                .entity(&axe_man)
//                .set_translation((layer_location + offset).extend(0.0));
//        }
//    }
//}
//
//fn update_animation_timer(time: Res<Time>, mut animation_timer: ResMut<AnimationTimer>) {
//    animation_timer.0.tick(time.delta());
//}
//
//fn animate_water(
//    animation_timer: ResMut<AnimationTimer>,
//    mut animation_frame: Local<usize>,
//    ldtk_query: LdtkQuery,
//    mut ldtk_commands: LdtkCommands,
//) {
//    if animation_timer.0.just_finished() {
//        *animation_frame += 1;
//        *animation_frame %= 4;
//
//        for level in ldtk_query.levels() {
//            ["Water1", "Water2", "Water3", "Water4"]
//                .into_iter()
//                .filter_map(|identifier| level.get_layers().filter_identifier(identifier).next())
//                .enumerate()
//                .map(|(index, layer)| {
//                    let visibility = if *animation_frame == index {
//                        Visibility::Visible
//                    } else {
//                        Visibility::Hidden
//                    };
//                    (layer, visibility)
//                })
//                .for_each(|(layer, visibility)| {
//                    ldtk_commands.layer(&layer).set_visibility(visibility);
//                });
//        }
//    };
//}
//
//fn animate_entity(
//    mut ldtk_commands: LdtkCommands,
//    ldtk_query: LdtkQuery,
//    animation_query: Query<&ActorAnimationState>,
//    animation_timer: ResMut<AnimationTimer>,
//    mut animation_frame: Local<usize>,
//) {
//    //if animation_timer.0.just_finished() {
//    //    *animation_frame += 1;
//    //    *animation_frame %= 4;
//    //
//    //    ldtk_query.entities().for_each(|entity_item| {
//    //        let Ok(animation) = animation_query.get(entity_item.get_ecs_entity()) else {
//    //            return;
//    //        };
//    //
//    //        debug!(
//    //            "entity: {} animation: {:?}",
//    //            entity_item.get_identifier(),
//    //            animation
//    //        );
//    //
//    //        let (identifier, flip_x) = match animation.facing {
//    //            ActorFacing::North => ("IdleNorth", false),
//    //            ActorFacing::East => ("IdleProfile", false),
//    //            ActorFacing::South => ("IdleSouth", false),
//    //            ActorFacing::West => ("IdleProfile", true),
//    //        };
//    //
//    //        ldtk_commands
//    //            .entity(&entity_item)
//    //            .set_tile_to_field_instance_array_index(identifier, *animation_frame)
//    //            .set_sprite_flip_x(flip_x);
//    //    })
//    //}
//}
//
//fn update_message_board(
//    mut message_board_posts: EventReader<MessageBoardEvent>,
//    mut message_board_query: Query<&mut Text, With<MessageBoard>>,
//) {
//    message_board_posts
//        .read()
//        .for_each(|MessageBoardEvent(post)| {
//            message_board_query.single_mut().0 = post.clone();
//        });
//}
//
//fn you_died(mut commands: Commands, asset_server: Res<AssetServer>) {
//    commands.spawn((
//        Text::new("You Died!"),
//        TextFont {
//            font: asset_server.load("fonts/Primitive.ttf"),
//            font_size: 90.0,
//            ..Default::default()
//        },
//        TextColor(RED_900.into()),
//        TextLayout::new_with_justify(JustifyText::Center),
//        Node {
//            position_type: PositionType::Absolute,
//            top: Val::Px(350.0),
//            left: Val::Px(5.0),
//            right: Val::Px(5.0),
//            ..default()
//        },
//        MessageBoard,
//    ));
//}
