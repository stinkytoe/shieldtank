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

use crate::entity::{Entity, EntityItem};
use crate::int_grid::IntGrid;
use crate::item::LdtkItemTrait;
use crate::layer::{Layer, LayerItem};
use crate::level::{Level, LevelItem, LevelItemIteratorExt};
use crate::project::Project;
use crate::world::{WorldComponent, WorldItem};
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
    pub(crate) _project_assets: Res<'w, Assets<ProjectAsset>>,
    pub(crate) _projects_query: Query<'w, 's, (EcsEntity, Ref<'static, Project>)>,
    pub(crate) world_assets: Res<'w, Assets<WorldAsset>>,
    pub(crate) worlds_query: Query<'w, 's, (EcsEntity, Ref<'static, WorldComponent>)>,
    pub(crate) level_assets: Res<'w, Assets<LevelAsset>>,
    pub(crate) levels_query: Query<'w, 's, (EcsEntity, Ref<'static, Level>)>,
    pub(crate) layer_assets: Res<'w, Assets<LayerAsset>>,
    pub(crate) layers_query: Query<'w, 's, (EcsEntity, Ref<'static, Layer>)>,
    pub(crate) entity_assets: Res<'w, Assets<EntityAsset>>,
    pub(crate) entities_query: Query<'w, 's, (EcsEntity, Ref<'static, Entity>)>,
}

impl LdtkQuery<'_, '_> {
    pub fn worlds(&self) -> impl Iterator<Item = WorldItem> {
        self.worlds_query
            .iter()
            .filter_map(|(ecs_entity, component)| {
                Some((
                    ecs_entity,
                    self.world_assets.get(component.handle.id())?,
                    component,
                ))
            })
            //.inspect(|(_, asset, component)| {
            //    debug!("query: world asset: {:?}", asset.identifier);
            //    debug!(
            //        "query: world component is added: {:?}",
            //        component.is_added()
            //    );
            //})
            .map(|(ecs_entity, asset, component)| WorldItem {
                asset,
                component,
                ecs_entity,
                query: self,
            })
    }

    pub fn levels(&self) -> impl Iterator<Item = LevelItem> {
        self.levels_query
            .iter()
            .filter_map(|(ecs_entity, component)| {
                Some((
                    ecs_entity,
                    self.level_assets.get(component.handle.id())?,
                    component,
                ))
            })
            .map(|(ecs_entity, asset, component)| LevelItem {
                asset,
                component,
                ecs_entity,
                query: self,
            })
    }

    pub fn layers(&self) -> impl Iterator<Item = LayerItem> {
        self.layers_query
            .iter()
            .filter_map(|(ecs_entity, component)| {
                Some((
                    ecs_entity,
                    self.layer_assets.get(component.handle.id())?,
                    component,
                ))
            })
            .map(|(ecs_entity, asset, component)| LayerItem {
                asset,
                component,
                ecs_entity,
                query: self,
            })
    }

    pub fn entities(&self) -> impl Iterator<Item = EntityItem> {
        //EntityItem::make_entity_iterator(self)
        self.entities_query
            .iter()
            .filter_map(|(ecs_entity, component)| {
                Some((
                    ecs_entity,
                    self.entity_assets.get(component.handle.id())?,
                    component,
                ))
            })
            .map(|(ecs_entity, asset, component)| EntityItem {
                asset,
                component,
                ecs_entity,
                query: self,
            })
    }
    //
    //pub fn get_level(&self, ecs_entity: EcsEntity) -> Option<LevelItem> {
    //    LevelItem::get_level(self, ecs_entity)
    //}
    //
    //pub fn get_layer(&self, ecs_entity: EcsEntity) -> Option<LayerItem> {
    //    LayerItem::get_layer(self, ecs_entity)
    //}
    //
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
