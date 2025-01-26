use std::time::Duration;

use bevy::color::palettes::tailwind::GRAY_500;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
    .add_systems(Startup, startup)
    .add_systems(
        Update,
        (update_global_animation_timer, initialize_animate_tag),
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

            let visibility = if frame == index {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };

            shieldtank_commands.layer(&layer).insert(visibility);
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
                "Entity with animate tag spawned: {} iid: {}",
                item.get_identifier(),
                item.get_iid()
            );

            shieldtank_commands.entity(&item).insert(Direction::East);
        });
}
