pub mod by_iid;
pub mod getters;
pub mod iter;

use bevy_asset::{AssetId, AssetServer, Assets};
use bevy_ecs::system::{Query, Res, SystemParam};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::layer_definition::LayerDefinition as LdtkLayerDefinition;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_ldtk_asset::tileset_definition::TilesetDefinition as LdtkTilesetDefinition;
use bevy_ldtk_asset::world::World as WorldAsset;

use crate::component::entity::EntityComponentQueryData;
use crate::component::layer::LayerComponentQueryData;
use crate::component::level::LevelComponentQueryData;
use crate::component::project::ProjectComponentQueryData;
use crate::component::world::WorldComponentQueryData;
use crate::component::ShieldtankQueryData;
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

    config_assets: Res<'w, Assets<ProjectConfig>>,

    layer_definitions: Res<'w, Assets<LdtkLayerDefinition>>,
    tileset_definitions: Res<'w, Assets<LdtkTilesetDefinition>>,

    asset_server: Res<'w, AssetServer>,
}

impl ShieldtankQuery<'_, '_> {
    pub fn get_config_assets(&self) -> &Assets<ProjectConfig> {
        &self.config_assets
    }

    pub fn get_layer_definition(
        &self,
        id: AssetId<LdtkLayerDefinition>,
    ) -> Option<&LdtkLayerDefinition> {
        self.layer_definitions.get(id)
    }

    pub fn get_tileset_definition(
        &self,
        id: AssetId<LdtkTilesetDefinition>,
    ) -> Option<&LdtkTilesetDefinition> {
        self.tileset_definitions.get(id)
    }

    pub fn get_asset_server(&self) -> &AssetServer {
        &self.asset_server
    }
}
