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
                    shieldtank=trace,\
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
        Transform::from_scale(Vec2::splat(0.3).extend(1.0)),
    ));

    commands.spawn(shieldtank::project::Project {
        handle: asset_server.load("ldtk/top_down.ldtk"),
        config: asset_server.load("project_config/top_down.project_config.ron"),
    });
}

//fn update() {}
//
//fn handle_added_ldtk_entity(
//    //mut commands: Commands,
//    asset_server: Res<AssetServer>,
//    assets: Res<Assets<ldtk_asset::Entity>>,
//    query: Query<(Entity, &shieldtank::entity::Entity), Added<shieldtank::entity::Entity>>,
//) -> shieldtank::Result<()> {
//    query
//        .iter()
//        .try_for_each(|(_entity, component)| -> shieldtank::Result<()> {
//            block_on(async { asset_server.wait_for_asset(&component.handle).await })?;
//
//            let asset = assets
//                .get(component.handle.id())
//                .ok_or(shieldtank::Error::BadHandle)?;
//
//            info!("LDtk Entity added! {}@{}", asset.identifier, asset.iid);
//
//            Ok(())
//        })?;
//    Ok(())
//}
//
//fn handle_added_ldtk_level(
//    mut commands: Commands,
//    asset_server: Res<AssetServer>,
//    assets: Res<Assets<ldtk_asset::Level>>,
//    query: Query<(Entity, &shieldtank::level::Level), Added<shieldtank::level::Level>>,
//) -> shieldtank::Result<()> {
//    query
//        .iter()
//        .try_for_each(|(entity, component)| -> shieldtank::Result<()> {
//            block_on(async { asset_server.wait_for_asset(&component.handle).await })?;
//
//            let asset = assets
//                .get(component.handle.id())
//                .ok_or(shieldtank::Error::BadHandle)?;
//
//            info!("LDtk Level added! {}", asset.identifier);
//
//            commands
//                .entity(entity)
//                .insert(Name::new(asset.identifier().to_string()))
//                .with_children(|parent| {
//                    asset
//                        .children()
//                        .filter(|&handle| component.load_pattern.handle_matches_pattern(handle))
//                        .for_each(|handle| {
//                            parent.spawn(component.new_child(handle.clone()));
//                            //parent.spawn(shieldtank_component::Layer {
//                            //    handle: handle.clone(),
//                            //    load_pattern: component.load_pattern.clone(),
//                            //});
//                        });
//                });
//
//            Ok(())
//        })?;
//
//    Ok(())
//}
//
//fn handle_added_ldtk_layer(
//    mut commands: Commands,
//    asset_server: Res<AssetServer>,
//    assets: Res<Assets<ldtk_asset::Layer>>,
//    query: Query<(Entity, &shieldtank::layer::Layer), Added<shieldtank::layer::Layer>>,
//) -> shieldtank::Result<()> {
//    query
//        .iter()
//        .try_for_each(|(entity, component)| -> shieldtank::Result<()> {
//            block_on(async { asset_server.wait_for_asset(&component.handle).await })?;
//
//            let asset = assets
//                .get(component.handle.id())
//                .ok_or(shieldtank::Error::BadHandle)?;
//
//            info!("LDtk Layer added! {}", asset.identifier);
//
//            commands
//                .entity(entity)
//                .insert(Name::new(asset.identifier().to_string()))
//                .with_children(|parent| {
//                    asset
//                        .children()
//                        .filter(|&handle| component.load_pattern.handle_matches_pattern(handle))
//                        .for_each(|handle| {
//                            parent.spawn(shieldtank::entity::Entity {
//                                handle: handle.clone(),
//                            });
//                        });
//                });
//
//            Ok(())
//        })?;
//
//    Ok(())
//}
