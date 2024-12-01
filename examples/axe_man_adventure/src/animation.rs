use bevy::prelude::*;
use shieldtank::item_iterator::LdtkItemRecurrentIdentifierIterator;
use shieldtank::query::LdtkQuery;
use shieldtank::{commands::LdtkCommands, item::LdtkItemTrait};

use crate::actor::{ActorAction, ActorState};

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
    mut commands: Commands,
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
                    commands.entity(layer.get_ecs_entity()).insert(visibility);
                });
        }
    }
}

pub(crate) fn animate_actor_attacking_dying(
    time: Res<Time>,
    mut actor_query: Query<(Entity, &mut ActorState)>,
    mut commands: Commands,
    ldtk_query: LdtkQuery,
    mut ldtk_commands: LdtkCommands,
) {
    for (entity, mut actor) in actor_query.iter_mut() {
        let facing = actor.facing;
        let action = &mut actor.action;

        match (action, facing) {
            (
                crate::actor::ActorAction::Attacking { timer, frame },
                crate::actor::ActorDirection::North,
            ) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    if *frame == 4 {
                        commands.entity(entity).insert(ActorState {
                            facing,
                            action: ActorAction::Idle,
                        });
                    } else {
                        let Ok(entity_item) = ldtk_query.get_entity(entity) else {
                            continue;
                        };

                        ldtk_commands
                            .entity(&entity_item)
                            .set_tile_to_field_instance_array_index("AttackNorth", *frame);
                    }

                    *frame += 1;
                }
            }
            (
                crate::actor::ActorAction::Attacking { timer, frame },
                crate::actor::ActorDirection::East,
            )
            | (
                crate::actor::ActorAction::Attacking { timer, frame },
                crate::actor::ActorDirection::West,
            ) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    if *frame == 4 {
                        commands.entity(entity).insert(ActorState {
                            facing,
                            action: ActorAction::Idle,
                        });
                    } else {
                        let Ok(entity_item) = ldtk_query.get_entity(entity) else {
                            continue;
                        };

                        ldtk_commands
                            .entity(&entity_item)
                            .set_tile_to_field_instance_array_index("AttackProfile", *frame);
                    }

                    *frame += 1;
                }
            }
            (
                crate::actor::ActorAction::Attacking { timer, frame },
                crate::actor::ActorDirection::South,
            ) => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    if *frame == 4 {
                        commands.entity(entity).insert(ActorState {
                            facing,
                            action: ActorAction::Idle,
                        });
                    } else {
                        let Ok(entity_item) = ldtk_query.get_entity(entity) else {
                            continue;
                        };

                        ldtk_commands
                            .entity(&entity_item)
                            .set_tile_to_field_instance_array_index("AttackSouth", *frame);
                    }

                    *frame += 1;
                }
            }
            (
                crate::actor::ActorAction::Dead { timer, frame },
                crate::actor::ActorDirection::North,
            ) => {
                timer.tick(time.delta());

                if timer.just_finished() && *frame < 4 {
                    let Ok(entity_item) = ldtk_query.get_entity(entity) else {
                        continue;
                    };

                    ldtk_commands
                        .entity(&entity_item)
                        .set_tile_to_field_instance_array_index("DeadNorth", *frame);

                    *frame += 1;
                };
            }
            (
                crate::actor::ActorAction::Dead { timer, frame },
                crate::actor::ActorDirection::East,
            )
            | (
                crate::actor::ActorAction::Dead { timer, frame },
                crate::actor::ActorDirection::West,
            ) => {
                timer.tick(time.delta());

                if timer.just_finished() && *frame < 4 {
                    let Ok(entity_item) = ldtk_query.get_entity(entity) else {
                        continue;
                    };

                    ldtk_commands
                        .entity(&entity_item)
                        .set_tile_to_field_instance_array_index("DeadProfile", *frame);

                    *frame += 1;
                };
            }
            (
                crate::actor::ActorAction::Dead { timer, frame },
                crate::actor::ActorDirection::South,
            ) => {
                timer.tick(time.delta());

                if timer.just_finished() && *frame < 4 {
                    let Ok(entity_item) = ldtk_query.get_entity(entity) else {
                        continue;
                    };

                    ldtk_commands
                        .entity(&entity_item)
                        .set_tile_to_field_instance_array_index("DeadSouth", *frame);

                    *frame += 1;
                };
            }
            (_, _) => {}
        }
    }
}

pub(crate) fn animate_actor_idle_moving(
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
            (_, _) => {
                continue;
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
}
