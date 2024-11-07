use bevy_asset::Assets;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::system::{Query, Res, SystemParam};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::level::Level as LevelAsset;

use crate::{
    entity::{EntityData, EntityItem},
    level::{LevelData, LevelItem},
};

#[derive(SystemParam)]
pub struct LdtkQuery<'w, 's> {
    pub(crate) level_assets: Res<'w, Assets<LevelAsset>>,
    pub(crate) entity_assets: Res<'w, Assets<EntityAsset>>,
    pub(crate) levels_query: Query<'w, 's, LevelData<'static>>,
    pub(crate) entities_query: Query<'w, 's, EntityData<'static>>,
}

impl LdtkQuery<'_, '_> {
    pub fn levels(&self) -> impl Iterator<Item = LevelItem> {
        LevelItem::make_entity_iterator(self)
    }

    pub fn entities(&self) -> impl Iterator<Item = EntityItem> {
        EntityItem::make_entity_iterator(self)
    }

    pub fn get_entity(&self, ecs_entity: EcsEntity) -> Option<EntityItem> {
        self.entities_query
            .get(ecs_entity)
            .ok()
            .and_then(|data| {
                self.entity_assets
                    .get(data.1.handle.id())
                    .map(|asset| (asset, data))
            })
            .map(|(asset, data)| EntityItem {
                asset,
                data,
                _query: self,
            })
    }
}
