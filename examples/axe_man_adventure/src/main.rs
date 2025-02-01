use std::time::Duration;

use bevy::color::palettes::tailwind::GRAY_500;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use shieldtank::prelude::*;

const RESOLUTION: Vec2 = Vec2::new(1280.0, 960.0);

const GLOBAL_FRAME_TIME: f32 = 1.0 / 3.75;
const ATTACK_FRAME_TIME: f32 = 1.0 / 15.0;
const DEAD_FRAME_TIME: f32 = 1.0 / 7.5;

const PLAYER_MOVE_SPEED: f32 = 40.0;

const AXE_MAN_IID: Iid = iid!("a0170640-9b00-11ef-aa23-11f9c6be2b6e");
const LANCER_IID: Iid = iid!("85f22ca0-fec0-11ee-8cdd-41f7def1ae8a");

//
// Components
//

#[derive(Component, Reflect)]
struct MessageBoard;

#[derive(Debug, PartialEq, Eq, Component, Reflect)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Component, Reflect)]
struct PlayerMove {
    target: Vec2,
}

#[derive(PartialEq, Eq, Component, Reflect)]
enum LivingState {
    Alive,
    Dead,
}

#[derive(Component, Reflect)]
struct AnimationOverride {
    timer: Timer,
    frame: usize,
    animation_set: &'static str,
}

impl AnimationOverride {
    fn new_attack_animation() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(ATTACK_FRAME_TIME),
                TimerMode::Repeating,
            ),
            frame: 0,
            animation_set: "Attack",
        }
    }

    fn new_dying_animation() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(DEAD_FRAME_TIME),
                TimerMode::Repeating,
            ),
            frame: 0,
            animation_set: "Dead",
        }
    }
}

//
// Events
//

#[derive(Event)]
struct EntityAnimationEvent;

#[derive(Event)]
struct LayerAnimationEvent;

#[derive(Debug, Event)]
enum PlayerMoveEvent {
    Up,
    Right,
    Down,
    Left,
}

impl PlayerMoveEvent {
    fn as_vec2(&self) -> Vec2 {
        match self {
            PlayerMoveEvent::Up => (0.0, 1.0).into(),
            PlayerMoveEvent::Right => (1.0, 0.0).into(),
            PlayerMoveEvent::Down => (0.0, -1.0).into(),
            PlayerMoveEvent::Left => (-1.0, 0.0).into(),
        }
    }

    fn as_direction(&self) -> Direction {
        match self {
            PlayerMoveEvent::Up => Direction::North,
            PlayerMoveEvent::Right => Direction::East,
            PlayerMoveEvent::Down => Direction::South,
            PlayerMoveEvent::Left => Direction::West,
        }
    }
}

#[derive(Event)]
struct PlayerInteractEvent;

//
// Resources
//

#[derive(Debug, Resource, Reflect)]
pub(crate) struct GlobalAnimationTimer {
    pub(crate) timer: Timer,
    pub(crate) frame: usize,
}

impl GlobalAnimationTimer {
    fn new() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(GLOBAL_FRAME_TIME),
                TimerMode::Repeating,
            ),
            frame: 0,
        }
    }
}

//
// Main
//

fn main() {
    let log_plugin_settings = bevy::log::LogPlugin {
        level: bevy::log::Level::WARN,
        filter: "wgpu_hal=off,\
                 winit=off,\
                 bevy_winit=off,\
                 bevy_ldtk_asset=debug,\
                 shieldtank=debug,\
                 axe_man_adventure=debug"
            .into(),
        ..default()
    };

    let window_plugin_settings: WindowPlugin = WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::Windowed,
            resolution: RESOLUTION.into(),
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(log_plugin_settings)
            .set(window_plugin_settings)
            .set(ImagePlugin::default_nearest()),
        ShieldtankPlugins,
        WorldInspectorPlugin::default(),
    ));

    app.register_type::<AnimationOverride>()
        .register_type::<Direction>()
        .register_type::<PlayerMove>()
        .register_type::<LivingState>();

    app.insert_resource(GlobalAnimationTimer::new())
        .add_observer(animate_idle_entities)
        .add_observer(animate_water)
        .add_observer(player_interact_event)
        .add_observer(player_move_event);

    app.add_systems(Startup, startup).add_systems(
        Update,
        (
            animation_override,
            flip_sprites,
            initialize_entities,
            initialize_axe_man,
            lancer_brood,
            player_keyboard_commands,
            player_move,
            update_global_animation_timer,
        ),
    );

    app.run();
}

//
// Startup
//

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("Spawning project...");
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, -128.0, 0.0).with_scale(Vec2::splat(0.4).extend(1.0)),
    ));

    commands.spawn(ProjectComponent {
        handle: asset_server.load("ldtk/axe_man_adventure.ldtk"),
        config: asset_server.add(ProjectConfig::default()),
    });

    commands.spawn((
        Name::new("MessageBoard"),
        Text::new("The Axe Man begins his adventure!"),
        TextFont {
            font: asset_server.load("fonts/Primitive.ttf"),
            font_size: 50.0,
            ..Default::default()
        },
        TextColor(GRAY_500.into()),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(40.0),
            left: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        MessageBoard,
    ));
}

//
// Observers
//

#[allow(clippy::type_complexity)]
fn animate_idle_entities(
    trigger: Trigger<EntityAnimationEvent>,
    global_animation_timer: Res<GlobalAnimationTimer>,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
    player_query: Query<(
        &Direction,
        Option<&AnimationOverride>,
        Option<&LivingState>,
        Option<&PlayerMove>,
    )>,
) {
    let ecs_entity = trigger.entity();

    let Some((direction, animation_override, player_state, player_move)) =
        player_query.get(ecs_entity).ok()
    else {
        return;
    };

    let (frame, animation_set) = match (animation_override, player_state, player_move) {
        (Some(_), _, _) => return,
        (_, Some(LivingState::Dead), _) => (3, "Dead"),
        (_, _, Some(_)) => (global_animation_timer.frame, "Walk"),
        (_, _, None) => (global_animation_timer.frame, "Idle"),
    };

    let Some(shieldtank_entity) = shieldtank_query.get_entity(ecs_entity).ok() else {
        return;
    };

    let animation_direction = match direction {
        Direction::North => "North",
        Direction::South => "South",
        Direction::East | Direction::West => "Profile",
    };

    let field_array_name = format!("{animation_set}{animation_direction}");

    let Some(tile_array) = shieldtank_entity.get_field_array_tiles(&field_array_name) else {
        error!("Could not find field array tile: {field_array_name}");
        return;
    };

    let Some(tile) = tile_array.get(frame) else {
        error!("field array {field_array_name} index out of range: {frame}");
        return;
    };

    shieldtank_commands
        .entity(&shieldtank_entity)
        .insert(tile.clone());
}

fn animate_water(
    _trigger: Trigger<LayerAnimationEvent>,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
    global_animation_timer: Res<GlobalAnimationTimer>,
) {
    const LAYERS_TO_ANIMATE: &[&str] = &["Water1", "Water2", "Water3", "Water4"];

    let frame = global_animation_timer.frame;

    for level in shieldtank_query.iter_levels() {
        for (index, layer_identifier) in LAYERS_TO_ANIMATE.iter().enumerate() {
            let Some(layer) = level
                .iter_layers()
                .filter_identifier(layer_identifier)
                .next()
            else {
                return;
            };

            if frame == index {
                shieldtank_commands
                    .layer(&layer)
                    .insert(Visibility::Visible);
            } else {
                shieldtank_commands.layer(&layer).insert(Visibility::Hidden);
            }
        }
    }
}

fn player_interact_event(
    trigger: Trigger<PlayerInteractEvent>,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    let Some(axe_man) = shieldtank_query.get_entity(trigger.entity()).ok() else {
        return;
    };

    info!("interaction event! {}", axe_man.get_identifier());

    shieldtank_commands
        .entity(&axe_man)
        .insert(AnimationOverride::new_attack_animation());

    let Some(lancer) = shieldtank_query.entity_by_iid(LANCER_IID) else {
        return;
    };

    shieldtank_commands
        .entity(&lancer)
        .insert(AnimationOverride::new_dying_animation())
        .insert(LivingState::Dead);
}

fn player_move_event(
    trigger: Trigger<PlayerMoveEvent>,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
    mut message_board_query: Query<&mut Text, With<MessageBoard>>,
) {
    let Some(axe_man) = shieldtank_query.get_entity(trigger.entity()).ok() else {
        return;
    };

    let mut message_board = message_board_query.single_mut();

    let player_move_event = trigger.event();

    shieldtank_commands
        .entity(&axe_man)
        .insert(player_move_event.as_direction())
        .trigger(EntityAnimationEvent);

    info!(
        "PlayerMoveEvent: {} -> {:?}",
        axe_man.get_identifier(),
        player_move_event
    );

    let Some(layer) = axe_man.get_layer() else {
        error!("couldn't find layer?");
        return;
    };

    let Some(world) = axe_man.get_world() else {
        error!("couldn't find world?");
        return;
    };

    let grid_cell_size = layer.get_asset().grid_cell_size as f32;

    let axe_man_world_location = axe_man.world_location();

    let world_attempted_move =
        axe_man_world_location + grid_cell_size * player_move_event.as_vec2();

    if let Some(entity_at) = world
        .iter_entities()
        .filter_world_location_in_region(world_attempted_move)
        .next()
    {
        info!("Entity {} occupies space...", entity_at.get_identifier());
        // TODO: Add bump event
        return;
    };

    if let Some(int_grid_at) = world.int_grid_at(world_attempted_move) {
        // TODO: Change to an attempt_move event of some kind
        let movable_terrain = match int_grid_at.identifier.as_deref() {
            Some("grass") => {
                info!("Walking on grass!");
                message_board.0 = "The Axe Man is walking on the grass.".to_string();
                true
            }
            Some("dirt") => {
                info!("Walking on dirt!");
                message_board.0 = "The Axe Man is walking on dirt.".to_string();
                true
            }
            Some("tree") => {
                info!("Walking under a tree!");
                message_board.0 = "The Axe Man is shading under a tree.".to_string();
                true
            }
            Some("bridge") => {
                info!("Walking on a bridge!");
                message_board.0 = "The Axe Man is crossing The Bridge of Woe!".to_string();
                true
            }
            Some("water") => {
                info!("Walking on water!");
                message_board.0 = "The Axe Man cannot walk on water!".to_string();
                false
            }
            Some(unknown) => {
                info!("Walking on unknown terrain: {unknown}");
                false
            }
            None => {
                info!("no identifier...");
                false
            }
        };

        if movable_terrain {
            shieldtank_commands.entity(&axe_man).insert(PlayerMove {
                target: world_attempted_move,
            });
        }
    }
}

//
// Systems
//

fn animation_override(
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimationOverride, &Direction)>,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    for (ecs_entity, mut animation_override, direction) in query.iter_mut() {
        animation_override.timer.tick(time.delta());

        if animation_override.timer.just_finished() {
            let Some(shieldtank_entity) = shieldtank_query.get_entity(ecs_entity).ok() else {
                return;
            };

            let frame = animation_override.frame;
            let animation_set = animation_override.animation_set;

            if frame < 3 {
                animation_override.frame += 1;
                let animation_direction = match direction {
                    Direction::North => "North",
                    Direction::South => "South",
                    Direction::East | Direction::West => "Profile",
                };

                let field_array_name = format!("{animation_set}{animation_direction}");

                let Some(tile_array) = shieldtank_entity.get_field_array_tiles(&field_array_name)
                else {
                    error!("aah!");
                    return;
                };

                let Some(tile) = tile_array.get(frame).cloned() else {
                    error!("aah!");
                    return;
                };

                shieldtank_commands.entity(&shieldtank_entity).insert(tile);
            } else {
                shieldtank_commands
                    .entity(&shieldtank_entity)
                    .remove::<AnimationOverride>()
                    .trigger(EntityAnimationEvent);
            }
        }
    }
}

fn flip_sprites(
    direction_changed_query: Query<(Entity, &Direction), Changed<Direction>>,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    for (ecs_entity, direction) in direction_changed_query.iter() {
        let Some(shieldtank_entity) = shieldtank_query.get_entity(ecs_entity).ok() else {
            return;
        };

        if *direction == Direction::West {
            shieldtank_commands.entity(&shieldtank_entity).flip_x(true);
        } else {
            shieldtank_commands.entity(&shieldtank_entity).flip_x(false);
        }
    }
}

fn initialize_entities(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    shieldtank_query
        .iter_entities()
        .filter_just_finalized()
        .filter_tag("animate")
        .for_each(|item| {
            info!(
                "Entity with animate tag spawned: {}\tiid: {}",
                item.get_identifier(),
                item.get_iid()
            );

            shieldtank_commands.entity(&item).insert(Direction::East);
        });
}

fn initialize_axe_man(
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    let Some(axe_man) = shieldtank_query
        .iter_entities()
        .filter_just_finalized()
        .find_iid(AXE_MAN_IID)
    else {
        return;
    };

    info!("Setting components for The Axe Man!");

    shieldtank_commands
        .entity(&axe_man)
        .insert(LivingState::Alive);
}

fn lancer_brood(mut shieldtank_commands: ShieldtankCommands, shieldtank_query: ShieldtankQuery) {
    let Some(axe_man) = shieldtank_query.entity_by_iid(AXE_MAN_IID) else {
        return;
    };

    let Some(lancer) = shieldtank_query.entity_by_iid(LANCER_IID) else {
        return;
    };

    let dir_vec = axe_man.world_location() - lancer.world_location();

    let direction = match (dir_vec.x < dir_vec.y, -dir_vec.x < dir_vec.y) {
        (true, true) => Direction::North,
        (true, false) => Direction::West,
        (false, true) => Direction::East,
        (false, false) => Direction::South,
    };

    shieldtank_commands
        .entity(&lancer)
        .insert(direction)
        .trigger(EntityAnimationEvent);
}

fn player_keyboard_commands(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
    player_move_query: Query<Entity, With<PlayerMove>>,
) {
    let Some(axe_man) = shieldtank_query.entity_by_iid(AXE_MAN_IID) else {
        return;
    };

    if player_move_query.contains(axe_man.get_ecs_entity()) {
        return;
    };

    let up_pressed =
        keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW);

    let right_pressed =
        keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD);

    let down_pressed =
        keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS);

    let left_pressed =
        keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA);

    match (up_pressed, right_pressed, down_pressed, left_pressed) {
        (true, false, false, false) => {
            info!("Player move up!");
            shieldtank_commands
                .entity(&axe_man)
                .trigger(PlayerMoveEvent::Up);
        }
        (false, true, false, false) => {
            info!("Player move right!");
            shieldtank_commands
                .entity(&axe_man)
                .trigger(PlayerMoveEvent::Right);
        }
        (false, false, true, false) => {
            info!("Player move down!");
            shieldtank_commands
                .entity(&axe_man)
                .trigger(PlayerMoveEvent::Down);
        }
        (false, false, false, true) => {
            info!("Player move left!");
            shieldtank_commands
                .entity(&axe_man)
                .trigger(PlayerMoveEvent::Left);
        }
        _ => (),
    };

    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::KeyF) {
        info!("Player pressed interact key!");
        shieldtank_commands
            .entity(&axe_man)
            .trigger(PlayerInteractEvent);
    }
}

fn player_move(
    time: Res<Time>,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
    player_move_query: Query<(Entity, &PlayerMove), With<PlayerMove>>,
) {
    let Some((axe_man_ecs_entity, PlayerMove { target })) = player_move_query.get_single().ok()
    else {
        return;
    };

    let Some(axe_man) = shieldtank_query.get_entity(axe_man_ecs_entity).ok() else {
        warn!("some other entity besides axe man with a PlayerMove component?");
        return;
    };

    let axe_man_world_location = axe_man.world_location();

    let to_target = target - axe_man_world_location;

    if to_target.length_squared() < 0.01 {
        shieldtank_commands
            .entity(&axe_man)
            .remove::<PlayerMove>()
            .set_world_location(*target);
    } else {
        let new_location = axe_man_world_location
            + time.delta_secs() * PLAYER_MOVE_SPEED * to_target.normalize_or_zero();

        shieldtank_commands
            .entity(&axe_man)
            .set_world_location(new_location);
    }
}

fn update_global_animation_timer(
    time: Res<Time>,
    mut global_animation_timer: ResMut<GlobalAnimationTimer>,
    mut commands: Commands,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    global_animation_timer.timer.tick(time.delta());

    if global_animation_timer.timer.just_finished() {
        global_animation_timer.frame += 1;
        global_animation_timer.frame %= 4;

        commands.trigger(LayerAnimationEvent);

        shieldtank_query
            .iter_entities()
            .filter_tag("animate")
            .for_each(|shieldtank_entity| {
                shieldtank_commands
                    .entity(&shieldtank_entity)
                    .trigger(EntityAnimationEvent);
            });
    }
}
