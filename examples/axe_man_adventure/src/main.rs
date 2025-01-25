mod actor;
mod animation;
mod message_board;
mod player;
mod systems;

use actor::{ActorDirection, ActorMovement};
use animation::AnimationState;
use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::window::WindowResolution;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shieldtank::bevy_ldtk_asset::iid::{iid, Iid};
use shieldtank::plugin::ShieldtankPlugins;
use systems::{
    actor_movement, animate_entity, animate_water, keyboard_input, on_enter_playing,
    register_entity_animations, startup, update_entity_animation_timer,
    update_global_animation_timer, wait_project_loading,
};

const ACTOR_SPEED: f32 = 30.0;
const AXE_MAN_IID: Iid = iid!("a0170640-9b00-11ef-aa23-11f9c6be2b6e");

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(bevy::log::LogPlugin {
                level: bevy::log::Level::WARN,
                filter: "wgpu_hal=off,\
                    winit=off,\
                    bevy_winit=off,\
                    bevy_ldtk_asset=debug,\
                    shieldtank=debug,\
                    axe_man_adventure=debug"
                    .into(),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    resolution: WindowResolution::new(800.0, 600.0),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        // WorldInspectorPlugin::new(),
        ShieldtankPlugins,
    ))
    .register_type::<AnimationState>()
    .register_type::<ActorDirection>()
    .register_type::<ActorMovement>()
    .add_event::<message_board::MessageBoardEvent>()
    .add_event::<animation::GlobalAnimationEvent>()
    .insert_resource(animation::AnimationTimer::new(animation::GLOBAL_FRAME_TIME))
    .init_state::<GameState>()
    .add_observer(animate_water)
    .add_observer(animate_entity)
    .add_systems(OnEnter(GameState::Loading), startup)
    .add_systems(
        Update,
        (
            update_global_animation_timer,
            update_entity_animation_timer,
            register_entity_animations,
        ),
    )
    .add_systems(
        Update,
        wait_project_loading.run_if(in_state(GameState::Loading)),
    )
    .add_systems(OnEnter(GameState::Playing), on_enter_playing)
    .add_systems(
        Update,
        (keyboard_input, actor_movement).run_if(in_state(GameState::Playing)),
    );

    app.run();
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
    // GameOver,
}
