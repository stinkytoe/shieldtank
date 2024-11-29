use bevy::prelude::*;
use shieldtank::entity::EntityItemIteratorExt;
use shieldtank::field_instances::LdtkItemFieldInstancesExt;
use shieldtank::item::LdtkItemTrait;
use shieldtank::item_iterator::LdtkItemIterator;
use shieldtank::query::LdtkQuery;

use crate::actor::{ActorAction, ActorAttemptMoveEvent, ActorDirection, ActorState};
use crate::animation::ActorAnimationEvent;
use crate::message_board::MessageBoardEvent;
use crate::{post, GameState, AXE_MAN_IID};

#[derive(Debug)]
pub(crate) enum PlayerAction {
    MoveNorth,
    MoveEast,
    MoveSouth,
    MoveWest,
    Interact,
}

#[derive(Debug)]
pub(crate) enum PlayerInteractionEventKind {
    Bump(Entity),
    Interact,
}

#[derive(Debug, Event)]
pub(crate) struct PlayerInteractEvent {
    pub(crate) entity: Entity,
    pub(crate) kind: PlayerInteractionEventKind,
}

impl PlayerAction {
    fn from_keyboard_input(keyboard_input: &ButtonInput<KeyCode>) -> Option<Self> {
        let move_north =
            keyboard_input.pressed(KeyCode::ArrowUp) | keyboard_input.pressed(KeyCode::KeyW);
        let move_east =
            keyboard_input.pressed(KeyCode::ArrowRight) | keyboard_input.pressed(KeyCode::KeyD);
        let move_south =
            keyboard_input.pressed(KeyCode::ArrowDown) | keyboard_input.pressed(KeyCode::KeyS);
        let move_west =
            keyboard_input.pressed(KeyCode::ArrowLeft) | keyboard_input.pressed(KeyCode::KeyA);
        let interact =
            keyboard_input.pressed(KeyCode::Space) | keyboard_input.pressed(KeyCode::KeyF);

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

pub(crate) fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut actor_query: Query<&mut ActorState>,
    ldtk_query: LdtkQuery,
    mut actor_animation_event: EventWriter<ActorAnimationEvent>,
    mut actor_attempt_move_event: EventWriter<ActorAttemptMoveEvent>,
    mut player_interaction_event: EventWriter<PlayerInteractEvent>,
) {
    let Some(player_action) = PlayerAction::from_keyboard_input(&keyboard_input) else {
        return;
    };

    debug!("The player has performed an action! {player_action:?}");

    let Some(axe_man) = ldtk_query.entities().find_iid(AXE_MAN_IID) else {
        return;
    };

    let Ok(mut axe_man_actor_state) = actor_query.get_mut(axe_man.get_ecs_entity()) else {
        return;
    };

    // Only do something if we're currently Idle
    if matches!(axe_man_actor_state.action, ActorAction::Idle) {
        match player_action {
            PlayerAction::MoveNorth => {
                axe_man_actor_state.facing = ActorDirection::North;
                actor_attempt_move_event.send(ActorAttemptMoveEvent(axe_man.get_ecs_entity()));
            }
            PlayerAction::MoveEast => {
                axe_man_actor_state.facing = ActorDirection::East;
                actor_attempt_move_event.send(ActorAttemptMoveEvent(axe_man.get_ecs_entity()));
            }
            PlayerAction::MoveSouth => {
                axe_man_actor_state.facing = ActorDirection::South;
                actor_attempt_move_event.send(ActorAttemptMoveEvent(axe_man.get_ecs_entity()));
            }
            PlayerAction::MoveWest => {
                axe_man_actor_state.facing = ActorDirection::West;
                actor_attempt_move_event.send(ActorAttemptMoveEvent(axe_man.get_ecs_entity()));
            }
            PlayerAction::Interact => {
                player_interaction_event.send(PlayerInteractEvent {
                    entity: axe_man.get_ecs_entity(),
                    kind: PlayerInteractionEventKind::Interact,
                });
            }
        };
        actor_animation_event.send(ActorAnimationEvent(axe_man.get_ecs_entity()));
    }
}

pub(crate) fn player_interaction(
    mut player_interaction_events: EventReader<PlayerInteractEvent>,
    mut message_board_events: EventWriter<MessageBoardEvent>,
    mut commands: Commands,
    ldtk_query: LdtkQuery,
    actor_query: Query<&ActorState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for PlayerInteractEvent { entity, kind } in player_interaction_events.read() {
        let Ok(axe_man) = ldtk_query.get_entity(*entity) else {
            continue;
        };

        let name = axe_man.get_field_string("Name").unwrap_or("who?");

        match kind {
            PlayerInteractionEventKind::Bump(bumped_entity) => {
                let Ok(bumped_entity) = ldtk_query.get_entity(*bumped_entity) else {
                    continue;
                };

                let bumped_entity_name = bumped_entity.get_field_string("Name").unwrap_or("who?");

                post!(
                    message_board_events,
                    "{name} has bumped in to {bumped_entity_name}!"
                );
            }

            PlayerInteractionEventKind::Interact => {
                let Some(axe_man_global_location) = axe_man.get_global_location() else {
                    continue;
                };

                let Some(layer) = axe_man.get_layer() else {
                    continue;
                };

                let Ok(ActorState { facing, .. }) = actor_query.get(*entity) else {
                    continue;
                };

                let offset = facing.as_vec2(layer.get_grid_cell_size());

                if let Some(entity_at_location) = ldtk_query
                    .entities()
                    .filter_global_location(axe_man_global_location + offset)
                    .next()
                {
                    let entity_name = entity_at_location
                        .get_field_string("Name")
                        .unwrap_or("who?");

                    post!(
                        message_board_events,
                        "{name} has interacted with {entity_name}!"
                    );

                    if entity_at_location.has_tag("Enemy") {
                        commands.entity(*entity).insert(ActorState {
                            facing: *facing,
                            action: ActorAction::new_attacking(),
                        });

                        commands
                            .entity(entity_at_location.get_ecs_entity())
                            .insert(ActorState {
                                facing: facing.as_opposite(),
                                action: ActorAction::new_dead(),
                            });

                        next_state.set(GameState::GameOver);
                    }
                } else {
                    post!(message_board_events, "{name} has interacted with nothing!");
                    commands.entity(*entity).insert(ActorState {
                        facing: *facing,
                        action: ActorAction::new_attacking(),
                    });
                }
            }
        };
    }
}
