//#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::str::FromStr;

use bevy::math::I64Vec2;
use bevy::prelude::*;
use shieldtank::bevy_ldtk_asset::field_instance::FieldInstanceType;
use shieldtank::bevy_ldtk_asset::iid::Iid;
use shieldtank::entity::{EntityItem, EntityItemIteratorExt};
use shieldtank::item::LdtkItemTrait;
use shieldtank::item_iterator::LdtkItemIterator;
use shieldtank::plugin::ShieldtankPlugins;
use shieldtank::project_config::ProjectConfig;
use shieldtank::query::LdtkQuery;
use shieldtank::tileset_rectangle::TilesetRectangle;

const AXE_MAN_IID: &str = "a0170640-9b00-11ef-aa23-11f9c6be2b6e";

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
    .add_systems(Startup, startup)
    .add_systems(Update, (player_action, animate_axe_man));

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
}

fn player_action(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ldtk_query: LdtkQuery,
    mut player_facing: ResMut<PlayerFacing>,
) {
    // We wrap in a closure so we can early exit when one of many Option<..> calls returns None.
    // This way we can easily skip this loop if, for example, the Axe Man hasn't been loaded from
    // the LDtk project yet, or it's Transform/GlobalTransform/Sprite/etc components haven't been
    // finalized yet.
    //
    // A more robust game might consider using a game state to check when these
    // things to finish, and storing the appropriate references in a Resource.
    //
    // Another option is to have this system return Option, and create a special system to pipe our
    // return into, such as in the `system_piping` Bevy demo:
    // https://github.com/bevyengine/bevy/blob/main/examples/ecs/system_piping.rs
    //
    // For this demo, though, I'll stick with this method just to keep the code simple and easy to read.
    let mut player_action_inner = || {
        let player_action = PlayerAction::from_keyboard_input(&keyboard_input)?;

        let axe_man = ldtk_query
            .entities()
            .find_iid(Iid::from_str(AXE_MAN_IID).unwrap())?;

        debug!("The Axe Man has performed an action! {player_action:?}");

        let attempted_move = player_action.to_move_attempt();

        if let Some(move_direction) = attempted_move {
            let attempted_move_location =
                get_global_location_for_grid_move(&axe_man, move_direction)?;

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
                        .get_asset()
                        .field_instances
                        .iter()
                        .find(|tile| &tile.identifier == "Dead")
                        .as_ref()
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
                        .get_asset()
                        .field_instances
                        .iter()
                        .find(|tile| &tile.identifier == "Stab")
                        .as_ref()
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
                }
            } else {
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
                        debug!(
                            "The Axe Man, though virtuous, is just a man and cannot walk on water!"
                        );
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
    };

    player_action_inner();
}

fn animate_axe_man(
    mut commands: Commands,
    player_facing: ResMut<PlayerFacing>,
    ldtk_query: LdtkQuery,
) {
    let mut animate_axe_man_inner = || {
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
    };

    animate_axe_man_inner();
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
