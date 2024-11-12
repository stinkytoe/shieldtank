//#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::str::FromStr;

use bevy::color::palettes::tailwind::GRAY_500;
use bevy::math::I64Vec2;
use bevy::prelude::*;
use shieldtank::bevy_ldtk_asset::field_instance::FieldInstanceType;
use shieldtank::bevy_ldtk_asset::iid::Iid;
use shieldtank::entity::{EntityItem, EntityItemIteratorExt};
use shieldtank::field_instances::LdtkItemFieldInstancesExt;
use shieldtank::item::LdtkItemTrait;
use shieldtank::item_iterator::{LdtkItemIterator, LdtkItemRecurrentIdentifierIterator};
use shieldtank::level::LevelItemIteratorExt;
use shieldtank::plugin::ShieldtankPlugins;
use shieldtank::project_config::ProjectConfig;
use shieldtank::query::LdtkQuery;
use shieldtank::tileset_rectangle::TilesetRectangle;

const AXE_MAN_IID: &str = "a0170640-9b00-11ef-aa23-11f9c6be2b6e";

#[derive(Resource)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct MessageBoard;

#[derive(Event)]
struct MessageBoardEvent(String);

macro_rules! post_to_billboard {
    ($($message:tt)*) => {
        MessageBoardEvent(format!($($message)*))
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
    .insert_resource(PlayerFacing::Right)
    .insert_resource(AnimationTimer(Timer::from_seconds(
        0.250,
        TimerMode::Repeating,
    )))
    .add_event::<MessageBoardEvent>()
    .add_systems(Startup, startup)
    .add_systems(
        Update,
        (
            player_action.pipe(option_handler_system),
            animate_water.pipe(option_handler_system),
            animate_axe_man.pipe(option_handler_system),
            update_message_board,
        ),
    );

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
        handle: asset_server.load("ldtk/axe_man_adventure.ldtk#worlds:World"),
        config: asset_server.load("config/example.project_config.ron"),
    });

    commands.spawn((
        Text::new("The Axe Man begins his adventure!"),
        TextFont {
            font: asset_server.load("fonts/Shadowed Germanica.ttf"),
            font_size: 40.0,
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

fn player_action(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut message_board_writer: EventWriter<MessageBoardEvent>,
    ldtk_query: LdtkQuery,
    mut player_facing: ResMut<PlayerFacing>,
) -> Option<()> {
    let player_action = PlayerAction::from_keyboard_input(&keyboard_input)?;

    let axe_man = ldtk_query
        .entities()
        .find_iid(Iid::from_str(AXE_MAN_IID).unwrap())?;

    debug!("The Axe Man has performed an action! {player_action:?}");

    let attempted_move = player_action.to_move_attempt();

    if let Some(move_direction) = attempted_move {
        let attempted_move_location = get_global_location_for_grid_move(&axe_man, move_direction)?;

        let entity_at_move_location = ldtk_query
            .entities()
            .filter_global_location(attempted_move_location)
            .next();

        if let Some(entity_at_move_location) = entity_at_move_location {
            // TODO: it should be easier to check if an EntityItem has a tag
            if entity_at_move_location
                .get_asset()
                .tags
                .contains(&"Enemy".to_string())
            {
                debug!(
                    "The Axe Man has bumped into an enemy! {}",
                    entity_at_move_location.get_identifier()
                );

                // TODO: This needs to be WAY! simpler!
                let axe_man_dead_tile = axe_man
                    .get_field_instance("Dead")
                    .and_then(|field_instance| {
                        let FieldInstanceType::Tile(tile) = &field_instance.field_instance_type
                        else {
                            return None;
                        };
                        tile.as_ref()
                    })
                    .map(|tile| TilesetRectangle {
                        anchor: axe_man.get_asset().anchor,
                        tile: tile.clone(),
                    })?;

                commands
                    .entity(axe_man.get_ecs_entity())
                    .insert(axe_man_dead_tile);

                let enemy_stab_tile = entity_at_move_location
                    .get_field_instance("Stab")
                    .and_then(|field_instance| {
                        let FieldInstanceType::Tile(tile) = &field_instance.field_instance_type
                        else {
                            return None;
                        };
                        tile.as_ref()
                    })
                    .map(|tile| TilesetRectangle {
                        anchor: axe_man.get_asset().anchor,
                        tile: tile.clone(),
                    })?;

                commands
                    .entity(entity_at_move_location.get_ecs_entity())
                    .insert(enemy_stab_tile);

                message_board_writer.send(post_to_billboard!(
                    "Our hero, The Axe Man, was slain by the vile Green Lancer!"
                ));
            }
        } else {
            let int_grid_value_at_attempted_move_location = ldtk_query
                .int_grid_value_at_global_location(attempted_move_location)?
                .identifier?;

            let terrain_is_movable = match int_grid_value_at_attempted_move_location.as_str() {
                "bridge" => {
                    message_board_writer.send(post_to_billboard!(
                        "The Axe Man is crossing the Bridge of Woe!"
                    ));
                    true
                }
                "grass" | "dirt" => {
                    let level = ldtk_query
                        .levels()
                        .filter_global_location(attempted_move_location)
                        .next()?;

                    let FieldInstanceType::String(level_name) =
                        &level.get_field_instance("Name")?.field_instance_type
                    else {
                        return None;
                    };

                    message_board_writer.send(post_to_billboard!(
                        "The Axe Man is walking on some {} on the {}!",
                        int_grid_value_at_attempted_move_location,
                        level_name.as_ref().unwrap()
                    ));
                    true
                }
                "water" => {
                    message_board_writer.send(post_to_billboard!(
                        "The Axe Man, though virtuous, is just a man and cannot walk on water!"
                    ));
                    false
                }
                _ => {
                    message_board_writer.send(post_to_billboard!(
                        "The Axe Man is refusing to walk on some dubious unknown terrain! {}",
                        int_grid_value_at_attempted_move_location
                    ));
                    false
                }
            };

            if terrain_is_movable {
                let axe_man_transform = axe_man.get_transform()?;
                let axe_man_layer = axe_man.get_layer()?;
                let offset = (move_direction
                    * axe_man_layer.get_asset().grid_cell_size
                    * I64Vec2::new(1, -1))
                .as_vec2()
                .extend(0.0);

                if player_action == PlayerAction::MoveWest {
                    *player_facing = PlayerFacing::Left;
                }

                if player_action == PlayerAction::MoveEast {
                    *player_facing = PlayerFacing::Right;
                }

                let new_transform =
                    axe_man_transform.with_translation(axe_man_transform.translation + offset);

                commands
                    .entity(axe_man.get_ecs_entity())
                    .insert(new_transform);
            }
        }
    }

    Some(())
}

fn animate_water(
    mut commands: Commands,
    time: Res<Time>,
    mut animation_timer: ResMut<AnimationTimer>,
    mut animation_frame: Local<usize>,
    ldtk_query: LdtkQuery,
) -> Option<()> {
    animation_timer.0.tick(time.delta());
    if animation_timer.0.just_finished() {
        *animation_frame = (*animation_frame + 1) % 4;

        for level in ldtk_query.levels() {
            let water_1 = level.layers().filter_identifier("Water1").next()?;
            let water_2 = level.layers().filter_identifier("Water2").next()?;
            let water_3 = level.layers().filter_identifier("Water3").next()?;
            let water_4 = level.layers().filter_identifier("Water4").next()?;

            match *animation_frame {
                0 => {
                    commands
                        .entity(water_1.get_ecs_entity())
                        .insert(Visibility::Visible);
                    commands
                        .entity(water_2.get_ecs_entity())
                        .insert(Visibility::Hidden);
                    commands
                        .entity(water_3.get_ecs_entity())
                        .insert(Visibility::Hidden);
                    commands
                        .entity(water_4.get_ecs_entity())
                        .insert(Visibility::Hidden);
                }
                1 => {
                    commands
                        .entity(water_1.get_ecs_entity())
                        .insert(Visibility::Hidden);
                    commands
                        .entity(water_2.get_ecs_entity())
                        .insert(Visibility::Visible);
                    commands
                        .entity(water_3.get_ecs_entity())
                        .insert(Visibility::Hidden);
                    commands
                        .entity(water_4.get_ecs_entity())
                        .insert(Visibility::Hidden);
                }
                2 => {
                    commands
                        .entity(water_1.get_ecs_entity())
                        .insert(Visibility::Hidden);
                    commands
                        .entity(water_2.get_ecs_entity())
                        .insert(Visibility::Hidden);
                    commands
                        .entity(water_3.get_ecs_entity())
                        .insert(Visibility::Visible);
                    commands
                        .entity(water_4.get_ecs_entity())
                        .insert(Visibility::Hidden);
                }
                3 => {
                    commands
                        .entity(water_1.get_ecs_entity())
                        .insert(Visibility::Hidden);
                    commands
                        .entity(water_2.get_ecs_entity())
                        .insert(Visibility::Hidden);
                    commands
                        .entity(water_3.get_ecs_entity())
                        .insert(Visibility::Hidden);
                    commands
                        .entity(water_4.get_ecs_entity())
                        .insert(Visibility::Visible);
                }
                _ => unreachable!(),
            }
        }
    };

    Some(())
}

fn animate_axe_man(
    mut commands: Commands,
    player_facing: ResMut<PlayerFacing>,
    ldtk_query: LdtkQuery,
) -> Option<()> {
    if player_facing.is_changed() {
        let axe_man = ldtk_query
            .entities()
            .find_iid(Iid::from_str(AXE_MAN_IID).unwrap())?;

        let flip_x = match *player_facing {
            PlayerFacing::Left => true,
            PlayerFacing::Right => false,
        };

        let mut axe_man_sprite = axe_man.get_sprite()?.clone();

        axe_man_sprite.flip_x = flip_x;

        commands
            .entity(axe_man.get_ecs_entity())
            .insert(axe_man_sprite);
    }

    Some(())
}

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

fn option_handler_system(In(_result): In<Option<()>>) {}

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

#[derive(Debug, Resource)]
enum PlayerFacing {
    Left,
    Right,
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
