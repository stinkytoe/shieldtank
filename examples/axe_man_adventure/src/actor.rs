use bevy::math::I64Vec2;
use bevy::prelude::*;
use shieldtank::commands::LdtkCommands;
use shieldtank::entity::EntityItemIteratorExt;
use shieldtank::field_instances::LdtkItemFieldInstancesExt;
use shieldtank::item::LdtkItemTrait;
use shieldtank::query::LdtkQuery;

use crate::message_board::MessageBoardEvent;
use crate::post_to_message_board;

#[derive(Debug)]
pub(crate) enum ActorDirection {
    North,
    East,
    South,
    West,
}

impl ActorDirection {
    pub fn as_i64vec2(&self) -> I64Vec2 {
        match self {
            ActorDirection::North => I64Vec2::new(0, -1),
            ActorDirection::East => I64Vec2::new(1, 0),
            ActorDirection::South => I64Vec2::new(0, 1),
            ActorDirection::West => I64Vec2::new(-1, 0),
        }
    }

    pub fn as_vec2(&self, grid_cell_size: i64) -> Vec2 {
        let offset = grid_cell_size * I64Vec2::new(1, -1) * self.as_i64vec2();

        offset.as_vec2()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum ActorAction {
    Idle,
    Moving(ActorMovement),
    Attacking,
    Dead,
}

#[derive(Debug, PartialEq)]
pub(crate) struct ActorMovement {
    pub(crate) destination: Vec2,
    pub(crate) speed: f32,
}

#[derive(Debug, Component)]
pub(crate) struct ActorState {
    pub(crate) facing: ActorDirection,
    pub(crate) action: ActorAction,
}

#[derive(Debug, Event)]
pub(crate) struct ActorAttemptMoveEvent(pub Entity);

pub(crate) fn install_actor_components(mut commands: Commands, ldtk_query: LdtkQuery) {
    ldtk_query
        .entities()
        .for_each(|entity_item| match entity_item.get_identifier() {
            "Axe_Man" | "Thief" => {
                commands
                    .entity(entity_item.get_ecs_entity())
                    .insert(ActorState {
                        facing: ActorDirection::East,
                        action: ActorAction::Idle,
                    });
            }
            "Lancer" => {
                commands
                    .entity(entity_item.get_ecs_entity())
                    .insert(ActorState {
                        facing: ActorDirection::West,
                        action: ActorAction::Idle,
                    });
            }
            unknown => {
                error!("Unknown entity identifier! {unknown}");
            }
        });
}

pub(crate) fn actor_attempt_move(
    mut events: EventReader<ActorAttemptMoveEvent>,
    ldtk_query: LdtkQuery,
    mut actor_query: Query<&mut ActorState>,
    mut message_board_events: EventWriter<MessageBoardEvent>,
) {
    for ActorAttemptMoveEvent(entity) in events.read() {
        let Ok(entity_item) = ldtk_query.get_entity(*entity) else {
            continue;
        };

        let Some((layer, level, world)) = entity_item.get_layer_level_world() else {
            continue;
        };

        let Ok(mut actor_state) = actor_query.get_mut(*entity) else {
            continue;
        };

        let entity_name = entity_item.get_field_string("Name").unwrap_or("who?");

        let Some(global_location_of_entity) = entity_item.get_global_location() else {
            continue;
        };

        let offset = actor_state.facing.as_vec2(layer.get_grid_cell_size());

        let global_location_of_move = global_location_of_entity + offset;

        if let Some(_colliding_entity) = ldtk_query
            .entities()
            .filter_global_location(global_location_of_move)
            .next()
        {
            //if colliding_entity.has_tag("Enemy")
        } else {
            let Some(int_grid) = world.int_grid_value_at_global_location(global_location_of_move)
            else {
                continue;
            };

            let Some(int_grid_identifier) = int_grid.identifier else {
                error!("int grid with no identifier! {int_grid:?}");
                continue;
            };

            let level_name = level.get_field_string("Name").unwrap_or("unknown land");

            match int_grid_identifier.as_str() {
                "dirt" | "grass" => {
                    debug!(
                        "{entity_name} is walking on {int_grid_identifier} on the {level_name}!"
                    );
                }
                "bridge" => {
                    debug!("{entity_name} is crossing the Bridge of Woe!");
                }
                "tree" => {
                    debug!("{entity_name} is shading under the trees!");
                }
                "water" => {
                    debug!("{entity_name} is just a man and cannot walk on water!");
                    post_to_message_board!(
                        message_board_events,
                        "{entity_name} is just a man and cannot walk on water!"
                    );
                    continue;
                }
                unknown => {
                    debug!(
                        "{entity_name} is refusing to walk on dubious unknown terrain! {unknown}"
                    );
                    continue;
                }
            }

            actor_state.action = ActorAction::Moving(ActorMovement {
                destination: global_location_of_move,
                speed: 0.75,
            });
        }
    }
}

pub(crate) fn actor_moving(
    ldtk_query: LdtkQuery,
    mut ldtk_commands: LdtkCommands,
    mut actor_query: Query<(Entity, &mut ActorState)>,
) {
    for (entity, mut actor_state) in actor_query.iter_mut() {
        let Ok(entity_item) = ldtk_query.get_entity(entity) else {
            continue;
        };

        let ActorAction::Moving(ActorMovement { destination, speed }) = actor_state.action else {
            continue;
        };

        let Some(entity_location) = entity_item.get_global_location() else {
            continue;
        };

        let travel = destination - entity_location;

        if travel.length() < speed {
            actor_state.action = ActorAction::Idle;

            ldtk_commands
                .entity(&entity_item)
                .set_global_location(destination);
        } else {
            let target = entity_location + (travel.normalize() * speed);

            ldtk_commands
                .entity(&entity_item)
                .set_global_location(target);
        }
    }
}
