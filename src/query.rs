use bevy_asset::Assets;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::system::{Query, Res, SystemParam};
use bevy_ecs::world::Ref;
use bevy_hierarchy::{Children, Parent};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_render::view::Visibility;
use bevy_sprite::Sprite;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::entity::{EntityComponent, EntityItem};
use crate::int_grid::IntGrid;
use crate::layer::{LayerComponent, LayerItem};
use crate::level::{LevelComponent, LevelItem};
use crate::project::{ProjectComponent, ProjectItem};
use crate::world::{WorldComponent, WorldItem};
use crate::{bad_ecs_entity, bad_handle, Result};

#[allow(clippy::type_complexity)]
#[derive(SystemParam)]
pub struct LdtkQuery<'w, 's> {
    // For walking the tree
    pub(crate) parent_query: Query<'w, 's, &'static Parent>,
    pub(crate) children_query: Query<'w, 's, &'static Children>,
    // Various important components
    // pub(crate) transform_query: Query<'w, 's, &'static Transform>,
    // pub(crate) global_transform_query: Query<'w, 's, &'static GlobalTransform>,
    pub(crate) int_grid_query: Query<'w, 's, &'static IntGrid>,
    pub(crate) sprite_query: Query<'w, 's, &'static Sprite>,
    // For each component type
    pub(crate) project_assets: Res<'w, Assets<ProjectAsset>>,
    pub(crate) projects_query: Query<
        'w,
        's,
        (
            EcsEntity,
            Ref<'static, ProjectComponent>,
            Ref<'static, Transform>,
            Ref<'static, GlobalTransform>,
            Ref<'static, Visibility>,
        ),
    >,
    pub(crate) world_assets: Res<'w, Assets<WorldAsset>>,
    pub(crate) worlds_query: Query<
        'w,
        's,
        (
            EcsEntity,
            Ref<'static, WorldComponent>,
            Ref<'static, Transform>,
            Ref<'static, GlobalTransform>,
            Ref<'static, Visibility>,
        ),
    >,
    pub(crate) level_assets: Res<'w, Assets<LevelAsset>>,
    pub(crate) levels_query: Query<
        'w,
        's,
        (
            EcsEntity,
            Ref<'static, LevelComponent>,
            Ref<'static, Transform>,
            Ref<'static, GlobalTransform>,
            Ref<'static, Visibility>,
        ),
    >,
    pub(crate) layer_assets: Res<'w, Assets<LayerAsset>>,
    pub(crate) layers_query: Query<
        'w,
        's,
        (
            EcsEntity,
            Ref<'static, LayerComponent>,
            Ref<'static, Transform>,
            Ref<'static, GlobalTransform>,
            Ref<'static, Visibility>,
        ),
    >,
    pub(crate) entity_assets: Res<'w, Assets<EntityAsset>>,
    pub(crate) entities_query: Query<
        'w,
        's,
        (
            EcsEntity,
            Ref<'static, EntityComponent>,
            Ref<'static, Transform>,
            Ref<'static, GlobalTransform>,
            Ref<'static, Visibility>,
        ),
    >,
}

macro_rules! define_iterator {
    ($name:tt, $result_item:ident, $query_field:tt, $assets_field:tt) => {
        pub fn $name(&self) -> impl Iterator<Item = $result_item> {
            self.$query_field
                .iter()
                .filter_map(
                    |(ecs_entity, component, transform, global_transform, visibility)| {
                        Some((
                            ecs_entity,
                            self.$assets_field.get(component.handle.id())?,
                            component,
                            transform,
                            global_transform,
                            visibility,
                        ))
                    },
                )
                .map(
                    |(ecs_entity, asset, component, transform, global_transform, visibility)| {
                        $result_item {
                            asset,
                            component,
                            transform,
                            global_transform,
                            visibility,
                            ecs_entity,
                            query: self,
                        }
                    },
                )
        }
    };
}

impl LdtkQuery<'_, '_> {
    define_iterator!(iter_projects, ProjectItem, projects_query, project_assets);
    define_iterator!(iter_worlds, WorldItem, worlds_query, world_assets);
    define_iterator!(iter_levels, LevelItem, levels_query, level_assets);
    define_iterator!(iter_layers, LayerItem, layers_query, layer_assets);
    define_iterator!(iter_entities, EntityItem, entities_query, entity_assets);
}

macro_rules! define_getter {
    ($name:tt, $result_item:ident, $query_field:tt, $assets_field:tt) => {
        pub fn $name(&self, ecs_entity: EcsEntity) -> Result<$result_item> {
            self.$query_field
                .get(ecs_entity)
                .map_err(|e| bad_ecs_entity!("{e} {ecs_entity:?}"))
                .and_then(
                    |(ecs_entity, component, transform, global_transform, visibility)| {
                        Ok((
                            ecs_entity,
                            self.$assets_field
                                .get(component.handle.id())
                                .ok_or(bad_handle!("{:?}", component.handle))?,
                            component,
                            transform,
                            global_transform,
                            visibility,
                        ))
                    },
                )
                .map(
                    |(ecs_entity, asset, component, transform, global_transform, visibility)| {
                        $result_item {
                            asset,
                            component,
                            transform,
                            global_transform,
                            visibility,
                            ecs_entity,
                            query: self,
                        }
                    },
                )
        }
    };
}

impl LdtkQuery<'_, '_> {
    define_getter!(get_project, ProjectItem, projects_query, project_assets);
    define_getter!(get_world, WorldItem, worlds_query, world_assets);
    define_getter!(get_level, LevelItem, levels_query, level_assets);
    define_getter!(get_layer, LayerItem, layers_query, layer_assets);
    define_getter!(get_entity, EntityItem, entities_query, entity_assets);
}
