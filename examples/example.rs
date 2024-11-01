//#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use bevy::prelude::*;
use bevy_ldtk_asset::prelude::*;
use shieldtank::plugin::ShieldtankPlugin;

fn main() {
    //use ron::ser::PrettyConfig;
    //use shieldtank::project_config::ProjectConfig;
    //let sample_project_config = ProjectConfig::default();
    //let ser = ron::ser::to_string_pretty(&sample_project_config, PrettyConfig::default()).unwrap();
    //println!("{ser}");

    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(bevy::log::LogPlugin {
                level: bevy::log::Level::WARN,
                filter: "wgpu_hal=off,\
                    winit=off,\
                    bevy_ldtk_asset=debug,\
                    shieldtank=debug,\
                    example=trace"
                    .into(),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        //WorldInspectorPlugin::default(),
        BevyLdtkAssetPlugin,
        ShieldtankPlugin,
    ))
    .add_systems(Startup, startup);

    //app //
    //    .register_type::<shieldtank::level::Level>()
    //    .register_type::<shieldtank::layer::Layer>()
    //    .add_systems(Startup, startup)
    //    .add_systems(Update, update)
    //    .add_systems(
    //        Update,
    //        (
    //            handle_added_ldtk_level.map(dbg),
    //            handle_added_ldtk_layer.map(dbg),
    //            handle_added_ldtk_entity.map(dbg),
    //        ),
    //    );

    app.run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        // good scale for a 1920x1080 canvas/window
        Transform::from_scale(Vec2::splat(0.7).extend(1.0)),
    ));

    commands.spawn(shieldtank::project::Project {
        handle: asset_server.load("ldtk/top_down.ldtk"),
        config: asset_server.load("project_config/top_down.project_config.ron"),
    });
}
