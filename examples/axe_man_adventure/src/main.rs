//#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::str::FromStr;

use bevy::input::keyboard::KeyboardInput;
use bevy::math::I64Vec2;
use bevy::prelude::*;
use shieldtank::bevy_ldtk_asset::iid::Iid;
use shieldtank::entity::EntityItem;
use shieldtank::item::LdtkItemTrait;
use shieldtank::item_iterator::LdtkItemIterator;
use shieldtank::plugin::ShieldtankPlugins;
use shieldtank::project_config::ProjectConfig;
use shieldtank::query::LdtkQuery;

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
    .add_systems(Startup, startup)
    .add_systems(Update, update.pipe(option_handler_system));

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
        handle: asset_server.load("ldtk/axe_man_adventure.ldtk#World"),
        config: asset_server.load("config/example.project_config.ron"),
    });
}

fn option_handler_system(In(_result): In<Option<()>>) {}

fn update(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ldtk_query: LdtkQuery,
) -> Option<()> {
    let player_action = get_player_action(&keyboard_input)?;

    let axe_man_iid = Iid::from_str("a0170640-9b00-11ef-aa23-11f9c6be2b6e").unwrap();
    let axe_man = ldtk_query.entities().find_iid(axe_man_iid)?;

    debug!("The Axe Man has performed an action! {player_action:?}");

    let attempted_move = match player_action {
        PlayerAction::MoveNorth => Some(I64Vec2::new(0, -1)),
        PlayerAction::MoveEast => Some(I64Vec2::new(1, 0)),
        PlayerAction::MoveSouth => Some(I64Vec2::new(0, 1)),
        PlayerAction::MoveWest => Some(I64Vec2::new(-1, 0)),
        PlayerAction::Interact => None,
    };

    if let Some(move_direction) = attempted_move {
        let attempted_move_location = get_global_location_for_grid_move(&axe_man, move_direction)?;

        let int_grid_value_at_attempted_move_location = ldtk_query
            .int_grid_value_at_global_location(attempted_move_location)?
            .identifier?;

        let terrain_is_movable = match int_grid_value_at_attempted_move_location.as_str() {
            "bridge" => {
                debug!("The Axe Man is walking on a bridge!");
                true
            }
            "grass" | "dirt" => {
                debug!(
                    "The Axe Man is walking on some {}!",
                    int_grid_value_at_attempted_move_location
                );
                true
            }
            "water" => {
                debug!("The Axe Man, though virtuous, is just a man and cannot walk on water!");
                false
            }
            _ => {
                debug!("The Axe Man is refusing to walk on some dubious unknown terrain! {int_grid_value_at_attempted_move_location}");
                false
            }
        };

        if terrain_is_movable {
            let axe_man_transform = axe_man.get_transform()?;
            let axe_man_layer = axe_man.get_layer()?;
            let offset =
                (move_direction * axe_man_layer.get_asset().grid_cell_size * I64Vec2::new(1, -1))
                    .as_vec2()
                    .extend(0.0);

            let new_transform =
                axe_man_transform.with_translation(axe_man_transform.translation + offset);

            commands
                .entity(axe_man.get_ecs_entity())
                .insert(new_transform);
        }
    }

    Some(())
}

fn get_global_location_for_grid_move(entity_item: &EntityItem, grid_move: I64Vec2) -> Option<Vec2> {
    let global_location = entity_item.get_global_location()?;
    let layer = entity_item.get_layer()?;
    let grid_cell_size = layer.get_asset().grid_cell_size;
    let offset = grid_move * grid_cell_size * I64Vec2::new(1, -1);
    let attempted_move_location = global_location + offset.as_vec2();

    Some(attempted_move_location)
}

#[derive(Debug)]
enum PlayerAction {
    MoveNorth,
    MoveEast,
    MoveSouth,
    MoveWest,
    Interact,
}

fn get_player_action(keyboard_input: &ButtonInput<KeyCode>) -> Option<PlayerAction> {
    let move_north = keyboard_input.just_pressed(KeyCode::ArrowUp);
    let move_east = keyboard_input.just_pressed(KeyCode::ArrowRight);
    let move_south = keyboard_input.just_pressed(KeyCode::ArrowDown);
    let move_west = keyboard_input.just_pressed(KeyCode::ArrowLeft);
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
