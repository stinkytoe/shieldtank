use bevy_asset::Assets;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::system::{Query, Res, SystemParam};
use bevy_ldtk_asset::entity::Entity as EntityAsset;

use crate::entity::{Entity, EntityItem};

//use crate::entity::{EntityData, EntityItem};
//use crate::layer::{LayerData, LayerItem};
//use crate::level::{LevelData, LevelItem};

#[derive(SystemParam)]
pub struct LdtkQuery<'w, 's> {
    //pub(crate) parent_query: Query<'w, 's, Ref<'static, Parent>>,
    //pub(crate) children_query: Query<'w, 's, Ref<'static, Children>>,
    //pub(crate) level_assets: Res<'w, Assets<LevelAsset>>,
    //pub(crate) levels_query: Query<'w, 's, LevelData<'static>>,
    //pub(crate) layer_assets: Res<'w, Assets<LayerAsset>>,
    //pub(crate) layers_query: Query<'w, 's, LayerData<'static>>,
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
