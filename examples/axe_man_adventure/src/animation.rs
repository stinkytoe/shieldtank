use std::time::Duration;

use bevy::prelude::*;

pub const GLOBAL_FRAME_TIME: f32 = 1.0 / 3.75;
pub const ATTACKING_FRAME_TIME: f32 = 1.0 / 15.0;
pub const DEAD_FRAME_TIME: f32 = 1.0 / 7.5;

#[derive(Debug, Resource, Reflect)]
pub(crate) struct AnimationTimer {
    pub(crate) timer: Timer,
    pub(crate) frame: usize,
}

impl AnimationTimer {
    pub fn new(frame_time: f32) -> Self {
        Self {
            timer: Timer::new(Duration::from_secs_f32(frame_time), TimerMode::Repeating),
            frame: 0,
        }
    }
}

// Idle and Moving will animate according to the global animation timer.
//
// Attacking will animate according to its own timer, and switch to Idle when complete.
//
// Dead will animate according to its own timer, and remain on its last frame.
#[derive(Debug, Component, Reflect)]
pub(crate) enum AnimationState {
    Idle,
    Moving,
    Attacking { timer: AnimationTimer },
    Dying { timer: AnimationTimer },
    Dead,
}

impl AnimationState {
    pub(crate) fn idle() -> Self {
        AnimationState::Idle
    }

    pub(crate) fn moving() -> Self {
        AnimationState::Moving
    }

    pub(crate) fn attacking() -> Self {
        AnimationState::Attacking {
            timer: AnimationTimer::new(ATTACKING_FRAME_TIME),
        }
    }

    pub(crate) fn dying() -> Self {
        AnimationState::Dying {
            timer: AnimationTimer::new(DEAD_FRAME_TIME),
        }
    }
    pub(crate) fn dead() -> Self {
        AnimationState::Dead
    }
}

#[derive(Debug, Event)]
pub(crate) struct GlobalAnimationEvent;

#[derive(Debug, Event)]
pub(crate) struct EntityAnimationEvent;

// pub(crate) fn animate_actor_attacking_dying(
//     time: Res<Time>,
//     mut actor_query: Query<(Entity, &mut ActorState)>,
//     mut commands: Commands,
//     ldtk_query: LdtkQuery,
//     mut ldtk_commands: LdtkCommands,
// ) {
//     for (entity, mut actor) in actor_query.iter_mut() {
//         let facing = actor.facing;
//         let action = &mut actor.action;
//
//         match (action, facing) {
//             (
//                 crate::actor::ActorAction::Attacking { timer, frame },
//                 crate::actor::ActorDirection::North,
//             ) => {
//                 timer.tick(time.delta());
//                 if timer.just_finished() {
//                     if *frame == 4 {
//                         commands.entity(entity).insert(ActorState {
//                             facing,
//                             action: ActorAction::Idle,
//                         });
//                     } else {
//                         let Ok(entity_item) = ldtk_query.get_entity(entity) else {
//                             continue;
//                         };
//
//                         ldtk_commands
//                             .entity(&entity_item)
//                             .set_tile_to_field_instance_array_index("AttackNorth", *frame);
//                     }
//
//                     *frame += 1;
//                 }
//             }
//             (
//                 crate::actor::ActorAction::Attacking { timer, frame },
//                 crate::actor::ActorDirection::East,
//             )
//             | (
//                 crate::actor::ActorAction::Attacking { timer, frame },
//                 crate::actor::ActorDirection::West,
//             ) => {
//                 timer.tick(time.delta());
//                 if timer.just_finished() {
//                     if *frame == 4 {
//                         commands.entity(entity).insert(ActorState {
//                             facing,
//                             action: ActorAction::Idle,
//                         });
//                     } else {
//                         let Ok(entity_item) = ldtk_query.get_entity(entity) else {
//                             continue;
//                         };
//
//                         ldtk_commands
//                             .entity(&entity_item)
//                             .set_tile_to_field_instance_array_index("AttackProfile", *frame);
//                     }
//
//                     *frame += 1;
//                 }
//             }
//             (
//                 crate::actor::ActorAction::Attacking { timer, frame },
//                 crate::actor::ActorDirection::South,
//             ) => {
//                 timer.tick(time.delta());
//                 if timer.just_finished() {
//                     if *frame == 4 {
//                         commands.entity(entity).insert(ActorState {
//                             facing,
//                             action: ActorAction::Idle,
//                         });
//                     } else {
//                         let Ok(entity_item) = ldtk_query.get_entity(entity) else {
//                             continue;
//                         };
//
//                         ldtk_commands
//                             .entity(&entity_item)
//                             .set_tile_to_field_instance_array_index("AttackSouth", *frame);
//                     }
//
//                     *frame += 1;
//                 }
//             }
//             (
//                 crate::actor::ActorAction::Dead { timer, frame },
//                 crate::actor::ActorDirection::North,
//             ) => {
//                 timer.tick(time.delta());
//
//                 if timer.just_finished() && *frame < 4 {
//                     let Ok(entity_item) = ldtk_query.get_entity(entity) else {
//                         continue;
//                     };
//
//                     ldtk_commands
//                         .entity(&entity_item)
//                         .set_tile_to_field_instance_array_index("DeadNorth", *frame);
//
//                     *frame += 1;
//                 };
//             }
//             (
//                 crate::actor::ActorAction::Dead { timer, frame },
//                 crate::actor::ActorDirection::East,
//             )
//             | (
//                 crate::actor::ActorAction::Dead { timer, frame },
//                 crate::actor::ActorDirection::West,
//             ) => {
//                 timer.tick(time.delta());
//
//                 if timer.just_finished() && *frame < 4 {
//                     let Ok(entity_item) = ldtk_query.get_entity(entity) else {
//                         continue;
//                     };
//
//                     ldtk_commands
//                         .entity(&entity_item)
//                         .set_tile_to_field_instance_array_index("DeadProfile", *frame);
//
//                     *frame += 1;
//                 };
//             }
//             (
//                 crate::actor::ActorAction::Dead { timer, frame },
//                 crate::actor::ActorDirection::South,
//             ) => {
//                 timer.tick(time.delta());
//
//                 if timer.just_finished() && *frame < 4 {
//                     let Ok(entity_item) = ldtk_query.get_entity(entity) else {
//                         continue;
//                     };
//
//                     ldtk_commands
//                         .entity(&entity_item)
//                         .set_tile_to_field_instance_array_index("DeadSouth", *frame);
//
//                     *frame += 1;
//                 };
//             }
//             (_, _) => {}
//         }
//     }
// }
//
// pub(crate) fn animate_actor_idle_moving(
//     animation_timer: ResMut<GlobalAnimationTimer>,
//     actor_query: Query<&ActorState>,
//     ldtk_query: LdtkQuery,
//     mut ldtk_commands: LdtkCommands,
//     mut actor_events: EventReader<ActorAnimationEvent>,
// ) {
//     for ActorAnimationEvent(actor_entity) in actor_events.read() {
//         let Ok(ActorState { facing, action, .. }) = actor_query.get(*actor_entity) else {
//             return;
//         };
//
//         let (identifier, flip_x) = match (action, facing) {
//             (crate::actor::ActorAction::Idle, crate::actor::ActorDirection::North) => {
//                 ("IdleNorth", false)
//             }
//             (crate::actor::ActorAction::Idle, crate::actor::ActorDirection::East) => {
//                 ("IdleProfile", false)
//             }
//             (crate::actor::ActorAction::Idle, crate::actor::ActorDirection::South) => {
//                 ("IdleSouth", false)
//             }
//             (crate::actor::ActorAction::Idle, crate::actor::ActorDirection::West) => {
//                 ("IdleProfile", true)
//             }
//             (crate::actor::ActorAction::Moving(_), crate::actor::ActorDirection::North) => {
//                 ("WalkNorth", false)
//             }
//             (crate::actor::ActorAction::Moving(_), crate::actor::ActorDirection::East) => {
//                 ("WalkProfile", false)
//             }
//             (crate::actor::ActorAction::Moving(_), crate::actor::ActorDirection::South) => {
//                 ("WalkSouth", false)
//             }
//             (crate::actor::ActorAction::Moving(_), crate::actor::ActorDirection::West) => {
//                 ("WalkProfile", true)
//             }
//             (_, _) => {
//                 continue;
//             }
//         };
//
//         let Ok(entity_item) = ldtk_query.get_entity(*actor_entity) else {
//             return;
//         };
//
//         ldtk_commands
//             .entity(&entity_item)
//             .set_tile_to_field_instance_array_index(identifier, animation_timer.frame)
//             .set_sprite_flip_x(flip_x);
//     }
// }
