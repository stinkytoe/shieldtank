pub mod entity;
pub mod layer;
pub mod level;
pub mod project;
pub mod world;

use bevy_asset::Handle;
use bevy_ecs::change_detection::Ref;
use bevy_ecs::entity::Entity;
use bevy_ecs::query::QueryData;
use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::prelude::LdtkAsset;
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::component::{ShieldtankComponent, ShieldtankQueryData};
use crate::project_config::ProjectConfig;
use crate::query::ShieldtankQuery;

#[derive()]
pub struct Item<'w, 's, A: LdtkAsset + 'w, D: QueryData + 'w> {
    pub(crate) asset: &'w A,
    pub(crate) config: &'w ProjectConfig,
    pub(crate) shieldtank_query_data: ShieldtankQueryData<'w, A>,
    pub(crate) component_query_data: D,
    pub(crate) query: &'w ShieldtankQuery<'w, 's>,
}

impl<A: LdtkAsset, D: QueryData> Item<'_, '_, A, D> {
    pub fn get_ecs_entity(&self) -> Entity {
        self.shieldtank_query_data.0
    }

    pub fn get_component(&self) -> &Ref<ShieldtankComponent<A>> {
        &self.shieldtank_query_data.1
    }

    pub fn get_transform(&self) -> &Ref<Transform> {
        &self.shieldtank_query_data.2
    }

    pub fn get_visibility(&self) -> &Ref<Visibility> {
        &self.shieldtank_query_data.3
    }

    pub fn get_asset(&self) -> &A {
        self.asset
    }

    pub fn get_asset_handle(&self) -> Handle<A> {
        self.shieldtank_query_data.1.handle.clone()
    }

    pub fn get_config(&self) -> &ProjectConfig {
        self.config
    }

    pub fn get_config_handle(&self) -> Handle<ProjectConfig> {
        self.shieldtank_query_data.1.config.clone()
    }

    pub fn get_iid(&self) -> Iid {
        self.asset.get_iid()
    }

    pub fn get_identifier(&self) -> &str {
        self.asset.get_identifier()
    }
}

impl<A: LdtkAsset, D: QueryData> Item<'_, '_, A, D> {
    pub fn is_finalized(&self) -> bool {
        self.shieldtank_query_data.4.is_some()
    }

    pub fn is_just_finalized(&self) -> bool {
        let Some(component_loaded) = self.shieldtank_query_data.4.as_ref() else {
            return false;
        };

        component_loaded.just_finalized
    }

    pub fn asset_is_loaded(&self) -> bool {
        self.query
            .asset_server
            .is_loaded_with_dependencies(self.get_asset_handle().id())
    }
}

// pub trait ItemTrait<A: LdtkAsset> {}
