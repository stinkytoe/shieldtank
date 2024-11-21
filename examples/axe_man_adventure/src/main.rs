use bevy::color::palettes::tailwind::GRAY_500;
use bevy::math::I64Vec2;
use bevy::prelude::*;
use bevy::utils::error;
use shieldtank::bevy_ldtk_asset::iid::{iid, Iid};
use shieldtank::commands::LdtkCommands;
use shieldtank::entity::{EntityItem, EntityItemIteratorExt};
use shieldtank::field_instances::LdtkItemFieldInstancesExt;
use shieldtank::item::LdtkItemTrait;
use shieldtank::item_iterator::{LdtkItemIterator, LdtkItemRecurrentIdentifierIterator};
use shieldtank::level::LevelItemIteratorExt;
use shieldtank::plugin::ShieldtankPlugins;
use shieldtank::query::LdtkQuery;

const AXE_MAN_IID: Iid = iid!("a0170640-9b00-11ef-aa23-11f9c6be2b6e");

#[derive(Resource)]
struct AnimationTimer(Timer);

#[derive(Debug, Default, Resource)]
struct Player(Option<Entity>);

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
enum GameState {
    #[default]
    WaitingOnPlayer,
    Playing,
}

#[derive(Component)]
struct MessageBoard;

#[derive(Event)]
struct MessageBoardEvent(String);

#[derive(Event)]
struct PlayerMoveEvent(I64Vec2);

macro_rules! post_to_billboard {
    ($board:expr, $($message:tt)*) => {
        $board.send(MessageBoardEvent(format!($($message)*)))
    };
}

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
    .insert_resource(AnimationTimer(Timer::from_seconds(
        0.250,
        TimerMode::Repeating,
    )))
    .init_resource::<Player>()
    .init_state::<GameState>()
    .add_event::<MessageBoardEvent>()
    .add_event::<PlayerMoveEvent>()
    .add_systems(OnEnter(GameState::WaitingOnPlayer), startup)
    .add_systems(
        Update,
        wait_on_player.run_if(in_state(GameState::WaitingOnPlayer)),
    )
    .add_systems(
        Update,
        (
            player_movement.map(error),
            player_action.map(error),
            animate_water,
            animate_axe_man,
            update_message_board,
        )
            .run_if(in_state(GameState::Playing)),
    );

    app.run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec2::splat(0.4).extend(1.0))
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
        MessageBoard,
    ));
}

fn wait_on_player(
    mut player_res: ResMut<Player>,
    mut next_state: ResMut<NextState<GameState>>,
    ldtk_query: LdtkQuery,
) {
    if let Some(player_item) = ldtk_query.entities().find_iid(AXE_MAN_IID) {
        *player_res = Player(Some(player_item.get_ecs_entity()));
        next_state.set(GameState::Playing);

        info!("Axe man initialized!");
    }
}

fn player_action(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_move_events: EventWriter<PlayerMoveEvent>,
) -> anyhow::Result<()> {
    let Some(player_action) = PlayerAction::from_keyboard_input(&keyboard_input) else {
        return Ok(());
    };

    debug!("The Axe Man has performed an action! {player_action:?}");

    let attempted_move = player_action.to_move_attempt();

    if let Some(move_direction) = attempted_move {
        player_move_events.send(PlayerMoveEvent(move_direction));
    }

    Ok(())
}

fn player_movement(
    mut player_move_events: EventReader<PlayerMoveEvent>,
    mut message_board_writer: EventWriter<MessageBoardEvent>,
    player: Res<Player>,
    ldtk_query: LdtkQuery,
    mut ldtk_commands: LdtkCommands,
) -> anyhow::Result<()> {
    for event in player_move_events.read() {
        let PlayerMoveEvent(move_direction) = event;

        let axe_man = ldtk_query.get_entity(player.0.unwrap())?;

        let Some(attempted_move_location) =
            get_global_location_for_grid_move(&axe_man, *move_direction)
        else {
            return Ok(());
        };

        if let Some(entity_at_move_location) = ldtk_query
            .entities()
            .filter_global_location(attempted_move_location)
            .next()
        {
            if entity_at_move_location.has_tag("Enemy") {
                debug!(
                    "The Axe Man has bumped into an enemy! {}",
                    entity_at_move_location.get_identifier()
                );

                ldtk_commands
                    .entity(&axe_man)
                    .set_tile_to_field_instance("Dead");

                ldtk_commands
                    .entity(&entity_at_move_location)
                    .set_tile_to_field_instance("Stab");

                post_to_billboard!(
                    message_board_writer,
                    "Our hero, The Axe Man, was slain by the vile Green Lancer!"
                );
            };

            return Ok(());
        }

        let Some(int_grid) = ldtk_query.int_grid_value_at_global_location(attempted_move_location)
        else {
            return Ok(());
        };

        let Some(int_grid_value_at_attempted_move_location) = int_grid.identifier else {
            return Ok(());
        };

        let terrain_is_movable = match int_grid_value_at_attempted_move_location.as_str() {
            "bridge" => {
                post_to_billboard!(
                    message_board_writer,
                    "The Axe Man is crossing the Bridge of Woe!"
                );
                true
            }
            "grass" | "dirt" => {
                let Some(level) = ldtk_query
                    .levels()
                    .filter_global_location(attempted_move_location)
                    .next()
                else {
                    return Ok(());
                };

                let Some(level_name) = level.get_field_string("Name") else {
                    return Ok(());
                };

                post_to_billboard!(
                    message_board_writer,
                    "The Axe Man is walking on {} on the {}!",
                    int_grid_value_at_attempted_move_location,
                    level_name
                );
                true
            }
            "water" => {
                post_to_billboard!(
                    message_board_writer,
                    "The Axe Man, though virtuous, is just a man and cannot walk on water!"
                );
                false
            }
            _ => {
                post_to_billboard!(
                    message_board_writer,
                    "The Axe Man is refusing to walk on some dubious unknown terrain! {}",
                    int_grid_value_at_attempted_move_location
                );
                false
            }
        };

        if terrain_is_movable {
            let (Some(axe_man_layer), Some(axe_man_transform)) =
                (axe_man.get_layer(), axe_man.get_transform())
            else {
                return Ok(());
            };

            let offset =
                (move_direction * axe_man_layer.get_asset().grid_cell_size * I64Vec2::new(1, -1))
                    .as_vec2()
                    .extend(0.0);

            let new_translation = axe_man_transform.translation + offset;

            ldtk_commands
                .entity(&axe_man)
                .set_translation(new_translation);
        }
    }

    Ok(())
}

fn animate_water(
    time: Res<Time>,
    mut animation_timer: ResMut<AnimationTimer>,
    mut animation_frame: Local<usize>,
    ldtk_query: LdtkQuery,
    mut ldtk_commands: LdtkCommands,
) {
    animation_timer.0.tick(time.delta());

    if animation_timer.0.just_finished() {
        *animation_frame += 1;
        *animation_frame %= 4;

        for level in ldtk_query.levels() {
            ["Water1", "Water2", "Water3", "Water4"]
                .into_iter()
                .filter_map(|identifier| level.layers().filter_identifier(identifier).next())
                .enumerate()
                .map(|(index, layer)| {
                    let visibility = if *animation_frame == index {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                    (layer, visibility)
                })
                .for_each(|(layer, visibility)| {
                    ldtk_commands.layer(&layer).set_visibility(visibility);
                });
        }
    };
}

fn animate_axe_man() {}

fn update_message_board(
    mut message_board_posts: EventReader<MessageBoardEvent>,
    mut message_board_query: Query<&mut Text, With<MessageBoard>>,
) {
    message_board_posts
        .read()
        .for_each(|MessageBoardEvent(post)| {
            message_board_query.single_mut().0 = post.clone();
        });
}

// TODO: Add this to the interface somehow...
fn get_global_location_for_grid_move(entity_item: &EntityItem, grid_move: I64Vec2) -> Option<Vec2> {
    let global_location = entity_item.get_global_location()?;
    let layer = entity_item.get_layer()?;
    let grid_cell_size = layer.get_asset().grid_cell_size;
    let offset = grid_move * grid_cell_size * I64Vec2::new(1, -1);
    let attempted_move_location = global_location + offset.as_vec2();

    Some(attempted_move_location)
}

#[derive(Debug, PartialEq, Eq)]
enum PlayerAction {
    MoveNorth,
    MoveEast,
    MoveSouth,
    MoveWest,
    Interact,
}

impl PlayerAction {
    fn from_keyboard_input(keyboard_input: &ButtonInput<KeyCode>) -> Option<Self> {
        let move_north = keyboard_input.just_pressed(KeyCode::ArrowUp)
            | keyboard_input.just_pressed(KeyCode::KeyW);
        let move_east = keyboard_input.just_pressed(KeyCode::ArrowRight)
            | keyboard_input.just_pressed(KeyCode::KeyD);
        let move_south = keyboard_input.just_pressed(KeyCode::ArrowDown)
            | keyboard_input.just_pressed(KeyCode::KeyS);
        let move_west = keyboard_input.just_pressed(KeyCode::ArrowLeft)
            | keyboard_input.just_pressed(KeyCode::KeyA);
        let interact = keyboard_input.just_pressed(KeyCode::Space);

        match (move_north, move_east, move_south, move_west, interact) {
            (true, false, false, false, false) => Some(PlayerAction::MoveNorth),
            (false, true, false, false, false) => Some(PlayerAction::MoveEast),
            (false, false, true, false, false) => Some(PlayerAction::MoveSouth),
            (false, false, false, true, false) => Some(PlayerAction::MoveWest),
            (false, false, false, false, true) => Some(PlayerAction::Interact),
            _ => None,
        }
    }
}

impl PlayerAction {
    fn to_move_attempt(&self) -> Option<I64Vec2> {
        match self {
            PlayerAction::MoveNorth => Some(I64Vec2::new(0, -1)),
            PlayerAction::MoveEast => Some(I64Vec2::new(1, 0)),
            PlayerAction::MoveSouth => Some(I64Vec2::new(0, 1)),
            PlayerAction::MoveWest => Some(I64Vec2::new(-1, 0)),
            PlayerAction::Interact => None,
        }
    }
}
