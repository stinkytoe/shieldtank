use bevy::prelude::*;
use shieldtank::commands::LdtkCommands;
use shieldtank::item_iterator::LdtkItemRecurrentIdentifierIterator;
use shieldtank::query::LdtkQuery;

use crate::actor::ActorState;

#[derive(Debug, Resource)]
pub(crate) struct GlobalAnimationTimer {
    pub(crate) timer: Timer,
    pub(crate) frame: usize,
}

#[derive(Debug, Event)]
pub(crate) struct GlobalAnimationEvent;

#[derive(Debug, Event)]
pub(crate) struct ActorAnimationEvent(pub Entity);

pub(crate) fn update_global_animation_timer(
    time: Res<Time>,
    mut global_events: EventWriter<GlobalAnimationEvent>,
    mut actor_events: EventWriter<ActorAnimationEvent>,
    mut animation_timer: ResMut<GlobalAnimationTimer>,
    actors_query: Query<Entity, With<ActorState>>,
) {
    animation_timer.timer.tick(time.delta());

    if animation_timer.timer.just_finished() {
        animation_timer.frame += 1;
        animation_timer.frame %= 4;

        global_events.send(GlobalAnimationEvent);

        for actor_entity in actors_query.iter() {
            actor_events.send(ActorAnimationEvent(actor_entity));
        }
    }
}

pub(crate) fn animate_water(
    global_animation_timer: ResMut<GlobalAnimationTimer>,
    mut events: EventReader<GlobalAnimationEvent>,
    ldtk_query: LdtkQuery,
    mut ldtk_commands: LdtkCommands,
) {
    for _ in events.read() {
        for level in ldtk_query.levels() {
            ["Water1", "Water2", "Water3", "Water4"]
                .into_iter()
                .filter_map(|identifier| level.get_layers().filter_identifier(identifier).next())
                .enumerate()
                .map(|(index, layer)| {
                    let visibility = if global_animation_timer.frame == index {
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
    }
}

pub(crate) fn animate_actor(
    animation_timer: ResMut<GlobalAnimationTimer>,
    actor_query: Query<&ActorState>,
    ldtk_query: LdtkQuery,
    mut ldtk_commands: LdtkCommands,
    mut actor_events: EventReader<ActorAnimationEvent>,
) {
    for ActorAnimationEvent(actor_entity) in actor_events.read() {
        let Ok(ActorState { facing, action, .. }) = actor_query.get(*actor_entity) else {
            return;
        };

        let (identifier, flip_x) = match (action, facing) {
            (crate::actor::ActorAction::Idle, crate::actor::ActorDirection::North) => {
                ("IdleNorth", false)
            }
            (crate::actor::ActorAction::Idle, crate::actor::ActorDirection::East) => {
                ("IdleProfile", false)
            }
            (crate::actor::ActorAction::Idle, crate::actor::ActorDirection::South) => {
                ("IdleSouth", false)
            }
            (crate::actor::ActorAction::Idle, crate::actor::ActorDirection::West) => {
                ("IdleProfile", true)
            }
            (crate::actor::ActorAction::Moving(_), crate::actor::ActorDirection::North) => {
                ("WalkNorth", false)
            }
            (crate::actor::ActorAction::Moving(_), crate::actor::ActorDirection::East) => {
                ("WalkProfile", false)
            }
            (crate::actor::ActorAction::Moving(_), crate::actor::ActorDirection::South) => {
                ("WalkSouth", false)
            }
            (crate::actor::ActorAction::Moving(_), crate::actor::ActorDirection::West) => {
                ("WalkProfile", true)
            }
            (crate::actor::ActorAction::Attacking, crate::actor::ActorDirection::North) => {
                todo!()
            }
            (crate::actor::ActorAction::Attacking, crate::actor::ActorDirection::East) => {
                todo!()
            }
            (crate::actor::ActorAction::Attacking, crate::actor::ActorDirection::South) => {
                todo!()
            }
            (crate::actor::ActorAction::Attacking, crate::actor::ActorDirection::West) => {
                todo!()
            }
            (crate::actor::ActorAction::Dead, crate::actor::ActorDirection::North) => {
                todo!()
            }
            (crate::actor::ActorAction::Dead, crate::actor::ActorDirection::East) => {
                todo!()
            }
            (crate::actor::ActorAction::Dead, crate::actor::ActorDirection::South) => {
                todo!()
            }
            (crate::actor::ActorAction::Dead, crate::actor::ActorDirection::West) => {
                todo!()
            }
        };

        let Ok(entity_item) = ldtk_query.get_entity(*actor_entity) else {
            return;
        };

        ldtk_commands
            .entity(&entity_item)
            .set_tile_to_field_instance_array_index(identifier, animation_timer.frame)
            .set_sprite_flip_x(flip_x);
    }
    //    }
}
