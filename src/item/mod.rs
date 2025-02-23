pub mod entity;
pub mod iter;
pub mod layer;
pub mod level;
mod macros;
pub mod project;
pub mod world;

use bevy_asset::Handle;
use bevy_ecs::change_detection::Ref;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::QueryData;
use bevy_hierarchy::{Children, Parent};
use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::prelude::LdtkAsset;
use bevy_math::Vec2;
use bevy_render::view::Visibility;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::component::{ShieldtankComponent, ShieldtankComponentFinalized, ShieldtankQueryData};
use crate::project_config::ProjectConfig;
use crate::query::ShieldtankQuery;

#[derive()]
pub struct Item<'w, 's, A: LdtkAsset + 'w, D: QueryData + 'w> {
    asset: &'w A,
    config: &'w ProjectConfig,
    shieldtank_query_data: ShieldtankQueryData<'w, A>,
    component_query_data: D,
    query: &'w ShieldtankQuery<'w, 's>,
}

impl<'w, 's, A, D> Item<'w, 's, A, D>
where
    A: LdtkAsset + 'w,
    D: QueryData + 'w,
{
    pub(crate) fn new(
        asset: &'w A,
        config: &'w ProjectConfig,
        shieldtank_query_data: ShieldtankQueryData<'w, A>,
        component_query_data: D,
        query: &'w ShieldtankQuery<'w, 's>,
    ) -> Self
    where
        'w: 's,
    {
        Self {
            asset,
            config,
            shieldtank_query_data,
            component_query_data,
            query,
        }
    }
}

impl<A: LdtkAsset, D: QueryData> Item<'_, '_, A, D> {
    pub fn get_ecs_entity(&self) -> Entity {
        self.shieldtank_query_data.0
    }

    pub fn get_component(&self) -> &Ref<ShieldtankComponent<A>> {
        &self.shieldtank_query_data.1
    }

    pub fn get_asset_handle(&self) -> Handle<A> {
        self.shieldtank_query_data.1.handle.clone()
    }

    pub fn get_config_handle(&self) -> Handle<ProjectConfig> {
        self.shieldtank_query_data.1.config.clone()
    }

    pub fn get_transform(&self) -> &Ref<Transform> {
        &self.shieldtank_query_data.2
    }

    pub fn get_global_transform(&self) -> &Ref<GlobalTransform> {
        &self.shieldtank_query_data.3
    }

    pub fn get_visibility(&self) -> &Ref<Visibility> {
        &self.shieldtank_query_data.4
    }

    pub fn get_parent_component(&self) -> &Option<Ref<Parent>> {
        &self.shieldtank_query_data.5
    }

    pub fn get_children_component(&self) -> &Option<Ref<Children>> {
        &self.shieldtank_query_data.6
    }

    pub fn get_asset(&self) -> &A {
        self.asset
    }

    pub fn get_config(&self) -> &ProjectConfig {
        self.config
    }

    pub fn get_iid(&self) -> Iid {
        self.asset.get_iid()
    }

    pub fn get_identifier(&self) -> &str {
        self.asset.get_identifier()
    }

    pub fn get_query(&self) -> &ShieldtankQuery {
        self.query
    }

    pub fn get_component_finalized(&self) -> &Option<Ref<ShieldtankComponentFinalized>> {
        &self.shieldtank_query_data.7
    }
}

impl<A: LdtkAsset, D: QueryData> Item<'_, '_, A, D> {
    pub fn is_finalized(&self) -> bool {
        self.get_component_finalized().is_some()
    }

    pub fn is_just_finalized(&self) -> bool {
        self.get_component_finalized()
            .as_ref()
            .and_then(|component_loaded| component_loaded.just_finalized.then_some(()))
            .is_some()
    }

    pub fn asset_is_loaded(&self) -> bool {
        self.get_query()
            .get_asset_server()
            .is_loaded_with_dependencies(self.get_asset_handle().id())
    }

    pub fn location(&self) -> Vec2 {
        self.get_transform().translation.truncate()
    }
}

impl<A: LdtkAsset, D: QueryData> PartialEq for Item<'_, '_, A, D> {
    fn eq(&self, other: &Self) -> bool {
        self.get_ecs_entity() == other.get_ecs_entity()
    }
}
