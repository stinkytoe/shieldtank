use bevy::prelude::*;
use shieldtank::item::LdtkItemTrait;
use shieldtank::item_iterator::LdtkItemRecurrentIdentifierIterator;
use shieldtank::query::LdtkQuery;

use crate::actor::{ActorAction, ActorAttemptMoveEvent, ActorDirection, ActorState};
use crate::animation::ActorAnimationEvent;

#[derive(Debug)]
pub(crate) enum PlayerAction {
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

pub(crate) fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut actor_query: Query<&mut ActorState>,
    ldtk_query: LdtkQuery,
    mut actor_animation_event: EventWriter<ActorAnimationEvent>,
    mut actor_attempt_move_event: EventWriter<ActorAttemptMoveEvent>,
) {
    let Some(player_action) = PlayerAction::from_keyboard_input(&keyboard_input) else {
        return;
    };

    debug!("The player has performed an action! {player_action:?}");

    let Some(axe_man) = ldtk_query.entities().filter_identifier("Axe_Man").next() else {
        return;
    };

    let Ok(mut axe_man_actor_state) = actor_query.get_mut(axe_man.get_ecs_entity()) else {
        return;
    };

    // Only do something if we're currently Idle
    if axe_man_actor_state.action == ActorAction::Idle {
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
            PlayerAction::Interact => todo!(),
        };
    }

    actor_animation_event.send(ActorAnimationEvent(axe_man.get_ecs_entity()));
}
