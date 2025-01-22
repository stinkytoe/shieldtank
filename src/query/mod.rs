use bevy_asset::{AssetServer, Assets};
use bevy_ecs::entity::Entity;
use bevy_ecs::system::{Query, Res, SystemParam};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_ldtk_asset::world::World as WorldAsset;

use crate::component::entity::EntityComponentQueryData;
use crate::component::layer::LayerComponentQueryData;
use crate::component::level::LevelComponentQueryData;
use crate::component::project::ProjectComponentQueryData;
use crate::component::world::WorldComponentQueryData;
use crate::component::ShieldtankQueryData;
use crate::error::Result;
use crate::item::entity::EntityItem;
use crate::item::layer::LayerItem;
use crate::item::level::LevelItem;
use crate::item::project::ProjectItem;
use crate::item::world::WorldItem;
use crate::item::Item;
use crate::project_config::ProjectConfig;
use crate::shieldtank_error;

#[derive(SystemParam)]
pub struct ShieldtankQuery<'w, 's> {
    project_assets: Res<'w, Assets<ProjectAsset>>,
    project_query: Query<
        'w,
        's,
        (
            ShieldtankQueryData<'static, ProjectAsset>,
            ProjectComponentQueryData<'static>,
        ),
    >,

    world_assets: Res<'w, Assets<WorldAsset>>,
    world_query: Query<
        'w,
        's,
        (
            ShieldtankQueryData<'static, WorldAsset>,
            WorldComponentQueryData<'static>,
        ),
    >,

    level_assets: Res<'w, Assets<LevelAsset>>,
    level_query: Query<
        'w,
        's,
        (
            ShieldtankQueryData<'static, LevelAsset>,
            LevelComponentQueryData<'static>,
        ),
    >,

    layer_assets: Res<'w, Assets<LayerAsset>>,
    layer_query: Query<
        'w,
        's,
        (
            ShieldtankQueryData<'static, LayerAsset>,
            LayerComponentQueryData<'static>,
        ),
    >,

    entity_assets: Res<'w, Assets<EntityAsset>>,
    entity_query: Query<
        'w,
        's,
        (
            ShieldtankQueryData<'static, EntityAsset>,
            EntityComponentQueryData<'static>,
        ),
    >,

    pub(crate) config_assets: Res<'w, Assets<ProjectConfig>>,

    pub(crate) asset_server: Res<'w, AssetServer>,
}

macro_rules! make_iter {
    ($name:tt, $iter_item:ident, $query_field:tt, $assets_field:tt) => {
        pub fn $name(&self) -> impl Iterator<Item = $iter_item> {
            self.$query_field.iter().filter_map(
                |(shieldtank_query_data, component_query_data)| -> Option<_> {
                    let asset_id = shieldtank_query_data.1.handle.id();
                    let config_id = shieldtank_query_data.1.config.id();

                    let asset = self.$assets_field.get(asset_id)?;
                    let config = self.config_assets.get(config_id)?;

                    Some(Item {
                        asset,
                        config,
                        shieldtank_query_data,
                        component_query_data,
                        query: self,
                    })
                },
            )
        }
    };
}

impl ShieldtankQuery<'_, '_> {
    make_iter!(iter_projects, ProjectItem, project_query, project_assets);
    make_iter!(iter_worlds, WorldItem, world_query, world_assets);
    make_iter!(iter_levels, LevelItem, level_query, level_assets);
    make_iter!(iter_layers, LayerItem, layer_query, layer_assets);
    make_iter!(iter_entities, EntityItem, entity_query, entity_assets);
}

macro_rules! make_getter {
    ($name:tt, $result_item:ident, $query_field:tt, $assets_field:tt) => {
        pub fn $name(&self, ecs_entity: Entity) -> Result<$result_item> {
            self.$query_field
                .get(ecs_entity)
                .map(|(shieldtank_query_data, component_query_data)| {
                    let asset_id = shieldtank_query_data.1.handle.id();
                    let config_id = shieldtank_query_data.1.config.id();

                    let asset = self
                        .$assets_field
                        .get(asset_id)
                        .ok_or(shieldtank_error!("Bad asset handle! {asset_id:?}",))?;

                    let config = self
                        .config_assets
                        .get(config_id)
                        .ok_or(shieldtank_error!("Bad config handle! {asset_id:?}",))?;

                    Ok(Item {
                        asset,
                        config,
                        shieldtank_query_data,
                        component_query_data,
                        query: self,
                    })
                })
                .map_err(|e| shieldtank_error!("Bad query! {e} {ecs_entity:?}",))?
        }
    };
}

impl ShieldtankQuery<'_, '_> {
    make_getter!(get_project, ProjectItem, project_query, project_assets);
    make_getter!(get_world, WorldItem, world_query, world_assets);
    make_getter!(get_level, LevelItem, level_query, level_assets);
    make_getter!(get_layer, LayerItem, layer_query, layer_assets);
    make_getter!(get_entity, EntityItem, entity_query, entity_assets);
}
