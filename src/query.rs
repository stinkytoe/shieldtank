use bevy_asset::Assets;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::system::{Query, Res, SystemParam};
use bevy_hierarchy::Parent;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_ldtk_asset::project::Project as ProjectAsset;
use bevy_ldtk_asset::world::World as WorldAsset;

use crate::entity::{Entity, EntityItem};
use crate::layer::Layer;
use crate::level::Level;
use crate::project::Project;
use crate::world::World;
//use crate::layer::{LayerData, LayerItem};
//use crate::level::{LevelData, LevelItem};

#[derive(SystemParam)]
pub struct LdtkQuery<'w, 's> {
    // For walking the tree
    pub(crate) _parent_query: Query<'w, 's, &'static Parent>,
    pub(crate) _children_query: Query<'w, 's, &'static Parent>,
    // For each component type
    pub(crate) _project_assets: Res<'w, Assets<ProjectAsset>>,
    pub(crate) _projects_query: Query<'w, 's, (EcsEntity, &'static Project)>,
    pub(crate) _level_assets: Res<'w, Assets<LevelAsset>>,
    pub(crate) _levels_query: Query<'w, 's, (EcsEntity, &'static Level)>,
    pub(crate) _world_assets: Res<'w, Assets<WorldAsset>>,
    pub(crate) _worlds_query: Query<'w, 's, (EcsEntity, &'static World)>,
    pub(crate) _layer_assets: Res<'w, Assets<LayerAsset>>,
    pub(crate) _layers_query: Query<'w, 's, (EcsEntity, &'static Layer)>,
    pub(crate) entity_assets: Res<'w, Assets<EntityAsset>>,
    pub(crate) entities_query: Query<'w, 's, (EcsEntity, &'static Entity)>,
}

impl LdtkQuery<'_, '_> {
    //pub fn levels(&self) -> impl Iterator<Item = LevelItem> {
    //    LevelItem::make_level_iterator(self)
    //}
    //
    //pub fn layers(&self) -> impl Iterator<Item = LayerItem> {
    //    LayerItem::make_layer_iterator(self)
    //}
    //
    pub fn entities(&self) -> impl Iterator<Item = EntityItem> {
        //EntityItem::make_entity_iterator(self)
        self.entities_query
            .iter()
            .filter_map(|(ecs_entity, component)| {
                Some((ecs_entity, self.entity_assets.get(component.handle.id())?))
            })
            .map(|(ecs_entity, asset)| EntityItem {
                asset,
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
