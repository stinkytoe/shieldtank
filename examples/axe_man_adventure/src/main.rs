use std::time::Duration;

use bevy::color::palettes::tailwind::GRAY_500;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shieldtank::bevy_ldtk_asset::iid::{iid, Iid};
use shieldtank::commands::ShieldtankCommands;
use shieldtank::component::project::ProjectComponent;
use shieldtank::item::entity::iter::HasTagIteratorExt;
use shieldtank::item::iter::recurrent_identifier::ItemRecurrentIdentifierIteratorExt;
use shieldtank::item::iter::ItemIteratorExt;
use shieldtank::plugin::ShieldtankPlugins;
use shieldtank::project_config::ProjectConfig;
use shieldtank::query::ShieldtankQuery;

const RESOLUTION: Vec2 = Vec2::new(1280.0, 960.0);
const GLOBAL_FRAME_TIME: f32 = 1.0 / 3.75;
const AXE_MAN_IID: Iid = iid!("a0170640-9b00-11ef-aa23-11f9c6be2b6e");

#[derive(Component)]
struct MessageBoard;

#[derive(Component, Reflect)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Event)]
struct GlobalAnimationEvent {
    frame: usize,
}

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

#[derive(Debug, Event)]
enum PlayerMoveEvent {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Event)]
struct PlayerInteractEvent;

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

    let window_plugin_settings = WindowPlugin {
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
    ))
    .register_type::<Direction>()
    .add_observer(animate_water)
    .add_observer(player_move_event)
    .add_observer(player_interact_event)
    .add_systems(Startup, startup)
    .add_systems(
        Update,
        (
            update_global_animation_timer,
            initialize_animate_tag,
            player_keyboard_commands,
        ),
    )
    .insert_resource(AnimationTimer::new(GLOBAL_FRAME_TIME));

    app.run();
}

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

fn update_global_animation_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut animation_timer: ResMut<AnimationTimer>,
) {
    animation_timer.timer.tick(time.delta());

    if animation_timer.timer.just_finished() {
        animation_timer.frame += 1;
        animation_timer.frame %= 4;

        commands.trigger(GlobalAnimationEvent {
            frame: animation_timer.frame,
        });
    }
}

fn animate_water(
    trigger: Trigger<GlobalAnimationEvent>,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    const LAYERS_TO_ANIMATE: &[&str] = &["Water1", "Water2", "Water3", "Water4"];

    let GlobalAnimationEvent { frame } = *trigger.event();

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

fn initialize_animate_tag(
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

fn player_keyboard_commands(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    let Some(axe_man) = shieldtank_query.entity_by_iid(AXE_MAN_IID) else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
        info!("Player move up!");
        shieldtank_commands
            .entity(&axe_man)
            .trigger(PlayerMoveEvent::Up);
    }

    if keyboard_input.just_pressed(KeyCode::ArrowRight)
        || keyboard_input.just_pressed(KeyCode::KeyD)
    {
        info!("Player move right!");
        shieldtank_commands
            .entity(&axe_man)
            .trigger(PlayerMoveEvent::Right);
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS)
    {
        info!("Player move down!");
        shieldtank_commands
            .entity(&axe_man)
            .trigger(PlayerMoveEvent::Down);
    }

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) || keyboard_input.just_pressed(KeyCode::KeyA)
    {
        info!("Player move left!");
        shieldtank_commands
            .entity(&axe_man)
            .trigger(PlayerMoveEvent::Left);
    }

    if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::KeyF) {
        info!("Player pressed interact key!");
        shieldtank_commands
            .entity(&axe_man)
            .trigger(PlayerInteractEvent);
    }
}

fn player_move_event(
    trigger: Trigger<PlayerMoveEvent>,
    mut _shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    let Some(axe_man) = shieldtank_query.get_entity(trigger.entity()).ok() else {
        return;
    };

    info!(
        "PlayerMoveEvent: {} -> {:?}",
        axe_man.get_identifier(),
        trigger.event()
    );
}

fn player_interact_event(
    trigger: Trigger<PlayerInteractEvent>,
    mut _shieldtank_commands: ShieldtankCommands,
    shieldtank_query: ShieldtankQuery,
) {
    let Some(axe_man) = shieldtank_query.get_entity(trigger.entity()).ok() else {
        warn!("axe_man not found. not loaded yet?");
        return;
    };

    let Some(world) = axe_man.get_world() else {
        error!("couldn't find world?");
        return;
    };

    let Some(int_grid_at) = world.int_grid_at(axe_man.world_location()) else {
        info!("no int grid at location!");
        return;
    };

    match int_grid_at.identifier.as_deref() {
        Some("grass") => info!("Walking on grass!"),
        Some("dirt") => info!("Walking on dirt!"),
        Some("tree") => info!("Walking under a tree!"),
        Some("bridge") => info!("Walking on a bridge!"),
        Some("water") => info!("Walking on water!"),
        Some(unknown) => info!("Walking on unknown terrain: {unknown}"),
        None => info!("no identifier..."),
    }
}
