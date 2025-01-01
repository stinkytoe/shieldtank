use bevy::color::palettes::tailwind::GRAY_500;
use bevy::log;
use bevy::prelude::*;
use bevy::state::commands;
use shieldtank::{
    commands::LdtkCommands,
    entity::EntityItemIteratorExt,
    item::LdtkItemTrait,
    item_iterator::{LdtkItemIterator, LdtkItemRecurrentIdentifierIterator},
    query::LdtkQuery,
    world::WorldComponent,
};

use crate::actor::ActorMovement;
use crate::actor::{ActorAttemptMoveEvent, ActorDirection};
use crate::animation::{
    AnimationState, AnimationTimer, EntityAnimationEvent, GlobalAnimationEvent,
};
use crate::message_board::MessageBoard;
use crate::player::PlayerAction;
use crate::ACTOR_SPEED;
use crate::{GameState, LdtkProject, AXE_MAN_IID};

pub(crate) fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scale = 0.4;
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec2::splat(scale).extend(1.0))
            .with_translation((0.0, -128.0, 1000.0).into()),
    ));

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

    commands.insert_resource(LdtkProject {
        project: asset_server.load("ldtk/axe_man_adventure.ldtk"),
    });
}

pub(crate) fn wait_project_loading(
    ldtk_project: Res<LdtkProject>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if asset_server.is_loaded_with_dependencies(ldtk_project.project.id()) {
        next_state.set(GameState::Playing)
    }
}

pub(crate) fn on_enter_playing(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(WorldComponent {
        handle: asset_server.load("ldtk/axe_man_adventure.ldtk#worlds:World"),
        config: asset_server.load("config/example.project_config.ron"),
    });
}

pub(crate) fn update_global_animation_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut animation_timer: ResMut<AnimationTimer>,
    query: Query<(Entity, &AnimationState)>,
) {
    animation_timer.timer.tick(time.delta());

    if animation_timer.timer.just_finished() {
        animation_timer.frame += 1;
        animation_timer.frame %= 4;

        log::trace!("tick! {}", animation_timer.frame);

        commands.trigger(GlobalAnimationEvent);

        query
            .iter()
            .for_each(|(entity, animation_state)| match animation_state {
                AnimationState::Idle | AnimationState::Moving | AnimationState::Dead => {
                    commands.entity(entity).trigger(EntityAnimationEvent);
                }
                _ => {}
            });
    }
}

pub(crate) fn update_entity_animation_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimationState)>,
) {
    query.iter_mut().for_each(
        |(entity, mut animation_state)| match animation_state.as_mut() {
            AnimationState::Attacking { timer } => {
                timer.timer.tick(time.delta());
                if timer.timer.just_finished() {
                    timer.frame += 1;

                    if timer.frame == 4 {
                        *animation_state = AnimationState::idle();
                    }

                    commands.entity(entity).trigger(EntityAnimationEvent);
                }
            }
            AnimationState::Dying { timer } => {
                timer.timer.tick(time.delta());
                if timer.timer.just_finished() {
                    timer.frame += 1;

                    if timer.frame == 4 {
                        *animation_state = AnimationState::dead();
                    }
                }
            }
            _ => {}
        },
    );
}

pub(crate) fn animate_water(
    trigger: Trigger<GlobalAnimationEvent>,
    animation_timer: Res<AnimationTimer>,
    mut commands: Commands,
    ldtk_query: LdtkQuery,
) {
    let event = trigger.event();
    log::trace!("animate_water frame: {event:?}");

    for level in ldtk_query.levels() {
        ["Water1", "Water2", "Water3", "Water4"]
            .into_iter()
            .filter_map(|identifier| level.get_layers().filter_identifier(identifier).next())
            .enumerate()
            .map(|(index, layer)| {
                let visibility = if animation_timer.frame == index {
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

pub(crate) fn register_entity_animations(mut commands: Commands, ldtk_query: LdtkQuery) {
    ldtk_query
        .entities()
        .filter_added()
        .filter_tag("animate")
        .for_each(|item| {
            log::info!(
                "LDtk entity with animate tag spawned: {}",
                item.get_identifier()
            );

            commands
                .entity(item.get_ecs_entity())
                .insert((AnimationState::idle(), ActorDirection::East))
                .observe(animate_entity)
                .observe(actor_attempt_move);
        });
}

pub(crate) fn animate_entity(
    trigger: Trigger<EntityAnimationEvent>,
    animation_timer: Res<AnimationTimer>,
    animate_entity_query: Query<(&AnimationState, &ActorDirection)>,
    mut ldtk_commands: LdtkCommands,
    ldtk_query: LdtkQuery,
) {
    let entity = trigger.entity();
    log::trace!("animate_entity_global_animation frame: {entity:?}");

    let Ok((animation_state, actor_direction)) = animate_entity_query.get(entity) else {
        return;
    };

    let (action_str, frame) = match animation_state {
        AnimationState::Idle => ("Idle", animation_timer.frame),
        AnimationState::Moving => ("Walk", animation_timer.frame),
        AnimationState::Attacking { timer } => ("Attack", timer.frame),
        AnimationState::Dying { timer } => ("Dead", timer.frame),
        AnimationState::Dead => ("Dead", 3),
    };

    let flip_x = matches!(actor_direction, ActorDirection::West);

    let Ok(entity_item) = ldtk_query.get_entity(entity) else {
        log::error!("Entity recieved EntityAnimationEvent without proper components! {entity:?}");
        return;
    };

    let identifier = &format!("{}{}", action_str, actor_direction);

    log::trace!(
        "animating {} with identifier: {} index: {}",
        entity_item.get_identifier(),
        identifier,
        animation_timer.frame
    );

    ldtk_commands
        .entity(&entity_item)
        .set_tile_to_field_instance_array_index(identifier, frame)
        .set_sprite_flip_x(flip_x);
}

pub(crate) fn keyboard_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // mut actor_query: Query<&mut ActorDirection>,
    mut actor_query: Query<(Entity, &AnimationState, &mut ActorDirection)>,
    ldtk_query: LdtkQuery,
    // mut actor_animation_event: EventWriter<ActorAnimationEvent>,
    // mut actor_attempt_move_event: EventWriter<ActorAttemptMoveEvent>,
    // mut player_interaction_event: EventWriter<PlayerInteractEvent>,
) {
    let Some(player_action) = PlayerAction::from_keyboard_input(&keyboard_input) else {
        return;
    };

    log::debug!("The player has performed an action! {player_action:?}");

    let Some(axe_man) = ldtk_query.entities().find_iid(AXE_MAN_IID) else {
        return;
    };

    let Ok((entity, animation_state, mut actor_direction)) =
        actor_query.get_mut(axe_man.get_ecs_entity())
    else {
        return;
    };

    if !matches!(animation_state, AnimationState::Idle) {
        return;
    };

    match player_action {
        PlayerAction::MoveNorth => {
            *actor_direction = ActorDirection::North;
            commands.entity(entity).trigger(ActorAttemptMoveEvent);
        }
        PlayerAction::MoveEast => {
            *actor_direction = ActorDirection::East;
            commands.entity(entity).trigger(ActorAttemptMoveEvent);
        }
        PlayerAction::MoveSouth => {
            *actor_direction = ActorDirection::South;
            commands.entity(entity).trigger(ActorAttemptMoveEvent);
        }
        PlayerAction::MoveWest => {
            *actor_direction = ActorDirection::West;
            commands.entity(entity).trigger(ActorAttemptMoveEvent);
        }
        PlayerAction::Interact => todo!(),
    }

    commands.entity(entity).trigger(EntityAnimationEvent);

    // actor_animation_event.send(ActorAnimationEvent(axe_man.get_ecs_entity()));
    //     // Only do something if we're currently Idle
    //     if matches!(axe_man_actor_state.action, ActorAction::Idle) {
    //         match player_action {
    //             PlayerAction::MoveNorth => {
    //                 axe_man_actor_state.facing = ActorDirection::North;
    //                 actor_attempt_move_event.send(ActorAttemptMoveEvent(axe_man.get_ecs_entity()));
    //             }
    //             PlayerAction::MoveEast => {
    //                 axe_man_actor_state.facing = ActorDirection::East;
    //                 actor_attempt_move_event.send(ActorAttemptMoveEvent(axe_man.get_ecs_entity()));
    //             }
    //             PlayerAction::MoveSouth => {
    //                 axe_man_actor_state.facing = ActorDirection::South;
    //                 actor_attempt_move_event.send(ActorAttemptMoveEvent(axe_man.get_ecs_entity()));
    //             }
    //             PlayerAction::MoveWest => {
    //                 axe_man_actor_state.facing = ActorDirection::West;
    //                 actor_attempt_move_event.send(ActorAttemptMoveEvent(axe_man.get_ecs_entity()));
    //             }
    //             PlayerAction::Interact => {
    //                 player_interaction_event.send(PlayerInteractEvent {
    //                     entity: axe_man.get_ecs_entity(),
    //                     kind: PlayerInteractionEventKind::Interact,
    //                 });
    //             }
    //         };
    //         actor_animation_event.send(ActorAnimationEvent(axe_man.get_ecs_entity()));
    //     }
}

pub(crate) fn actor_attempt_move(
    trigger: Trigger<ActorAttemptMoveEvent>,
    mut commands: Commands,
    ldtk_query: LdtkQuery,
    mut actor_direction_query: Query<(&ActorDirection, &mut AnimationState)>,
) {
    let ecs_entity = trigger.entity();
    log::debug!("Attempt move! event: {ecs_entity:?}");

    let Ok(entity) = ldtk_query.get_entity(ecs_entity) else {
        return;
    };

    let Some((layer, _level, world)) = entity.get_layer_level_world() else {
        return;
    };

    let Some(world_location) = entity.get_world_local_location() else {
        return;
    };

    let Ok((actor_direction, mut animation_state)) = actor_direction_query.get_mut(ecs_entity)
    else {
        return;
    };

    let grid_cell_size = layer.get_grid_cell_size();

    let actor_direction_as_vec2 = actor_direction.as_vec2(grid_cell_size);

    debug!("actor_direction_as_vec2: {actor_direction_as_vec2}");

    let attempted_move_world_location = world_location + actor_direction_as_vec2;

    let Some(int_grid_value) =
        world.int_grid_value_at_world_location(attempted_move_world_location)
    else {
        return;
    };

    let Some(int_grid_identifier) = int_grid_value.identifier.as_deref() else {
        return;
    };

    log::debug!("destination cell type: {int_grid_identifier}");

    match int_grid_identifier {
        "dirt" | "grass" | "bridge" => {
            // *animation_state = AnimationState::moving();
            commands.entity(ecs_entity).insert(ActorMovement {
                destination: attempted_move_world_location,
                speed: ACTOR_SPEED,
            });
        }
        "water" => {}
        _ => {}
    };
}

pub(crate) fn actor_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        // &GlobalTransform,
        &ActorMovement,
        &mut AnimationState,
    )>,
    ldtk_query: LdtkQuery,
    mut ldtk_commands: LdtkCommands,
) {
    query
        .iter_mut()
        .for_each(|(ecs_entity, actor_movement, mut animation_state)| {
            let Ok(entity) = ldtk_query.get_entity(ecs_entity) else {
                return;
            };

            let Some((layer, level, world)) = entity.get_layer_level_world() else {
                return;
            };

            // let Some(layer) = entity.get_layer() else {
            //     return;
            // };
            //
            // let Some(layer_location) = entity.get_layer_local_location() else {
            //     return;
            // };
            //
            // let Some(world_location) = entity.get_world_local_location() else {
            //     return;
            // };
            //
            // let offset = actor_movement.destination - world_location;
            //
            // // let direction = (actor_movement.destination - world_location).normalize()
            // //     * actor_movement.speed
            // //     * time.delta_secs();
            //
            // // if direction.length() < 0.1 {
            // ldtk_commands
            //     .entity(&entity)
            //     .set_layer_location(&layer, layer_location + offset);
            // *animation_state = AnimationState::idle();
            // commands.entity(ecs_entity).remove::<ActorMovement>();
            // // } else {
            // //     ldtk_commands
            // //         .entity(&entity)
            // //         .set_layer_location(&layer, layer_location + direction);
            // // };

            // log::debug!("{direction:?}");
        });
}
