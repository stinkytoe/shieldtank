use bevy_asset::Assets;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::system::{Query, Res, SystemParam};
use bevy_ecs::world::Ref;
use bevy_hierarchy::{Children, Parent};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::layer_definition::IntGridValue;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_ldtk_asset::world::World as WorldAsset;
use bevy_math::Vec2;
use bevy_sprite::Sprite;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::entity::{EntityComponent, EntityItem};
use crate::int_grid::IntGrid;
use crate::item::LdtkItemTrait;
use crate::layer::{LayerComponent, LayerItem};
use crate::level::{LevelComponent, LevelItem, LevelItemIteratorExt};
use crate::project::{ProjectComponent, ProjectItem};
use crate::world::{WorldComponent, WorldItem};
use crate::{bad_ecs_entity, bad_handle, Result};
//use crate::layer::{LayerData, LayerItem};
//use crate::level::{LevelData, LevelItem};

#[derive(SystemParam)]
pub struct LdtkQuery<'w, 's> {
    // For walking the tree
    pub(crate) parent_query: Query<'w, 's, &'static Parent>,
    pub(crate) children_query: Query<'w, 's, &'static Children>,
    // Various important components
    pub(crate) transform_query: Query<'w, 's, &'static Transform>,
    pub(crate) global_transform_query: Query<'w, 's, &'static GlobalTransform>,
    pub(crate) int_grid_query: Query<'w, 's, &'static IntGrid>,
    pub(crate) sprite_query: Query<'w, 's, &'static Sprite>,
    // For each component type
    pub(crate) project_assets: Res<'w, Assets<ProjectAsset>>,
    pub(crate) projects_query: Query<'w, 's, (EcsEntity, Ref<'static, ProjectComponent>)>,
    pub(crate) world_assets: Res<'w, Assets<WorldAsset>>,
    pub(crate) worlds_query: Query<'w, 's, (EcsEntity, Ref<'static, WorldComponent>)>,
    pub(crate) level_assets: Res<'w, Assets<LevelAsset>>,
    pub(crate) levels_query: Query<'w, 's, (EcsEntity, Ref<'static, LevelComponent>)>,
    pub(crate) layer_assets: Res<'w, Assets<LayerAsset>>,
    pub(crate) layers_query: Query<'w, 's, (EcsEntity, Ref<'static, LayerComponent>)>,
    pub(crate) entity_assets: Res<'w, Assets<EntityAsset>>,
    pub(crate) entities_query: Query<'w, 's, (EcsEntity, Ref<'static, EntityComponent>)>,
}

macro_rules! define_iterator {
    ($name:tt, $result_item:ident, $query_field:tt, $assets_field:tt) => {
        pub fn $name(&self) -> impl Iterator<Item = $result_item> {
            self.$query_field
                .iter()
                .filter_map(|(ecs_entity, component)| {
                    Some((
                        ecs_entity,
                        self.$assets_field.get(component.handle.id())?,
                        component,
                    ))
                })
                .map(|(ecs_entity, asset, component)| $result_item {
                    asset,
                    component,
                    ecs_entity,
                    query: self,
                })
        }
    };
}

impl LdtkQuery<'_, '_> {
    define_iterator!(projects, ProjectItem, projects_query, project_assets);
    define_iterator!(worlds, WorldItem, worlds_query, world_assets);
    define_iterator!(levels, LevelItem, levels_query, level_assets);
    define_iterator!(layers, LayerItem, layers_query, layer_assets);
    define_iterator!(entities, EntityItem, entities_query, entity_assets);
}

macro_rules! define_getter {
    ($name:tt, $result_item:ident, $query_field:tt, $assets_field:tt) => {
        pub fn $name(&self, ecs_entity: EcsEntity) -> Result<$result_item> {
            self.$query_field
                .get(ecs_entity)
                .map_err(|e| bad_ecs_entity!("{e} {ecs_entity:?}"))
                .and_then(|(ecs_entity, component)| {
                    Ok((
                        ecs_entity,
                        self.$assets_field
                            .get(component.handle.id())
                            .ok_or(bad_handle!("{:?}", component.handle))?,
                        component,
                    ))
                })
                .map(|(ecs_entity, asset, component)| $result_item {
                    asset,
                    component,
                    ecs_entity,
                    query: self,
                })
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

impl LdtkQuery<'_, '_> {
    pub fn int_grid_value_at_global_location(&self, global_location: Vec2) -> Option<IntGridValue> {
        let mut levels: Vec<_> = self
            .levels()
            .filter_global_location(global_location)
            .collect();

        // unwrap is OK here because the above collect wouldn't have yielded anything that didn't
        // have a global_transform component.
        #[allow(clippy::unwrap_used)]
        levels.sort_by(|a, b| {
            let a_z = a.get_global_transform().unwrap().translation().z;
            let b_z = b.get_global_transform().unwrap().translation().z;
            // intentionally reversed, so we will search nearest to farthest when looking down in
            // the world from above.
            b_z.partial_cmp(&a_z).unwrap()
        });

        levels
            .iter()
            .find_map(|level_item| level_item.int_grid_value_at_global_location(global_location))
    }
}
