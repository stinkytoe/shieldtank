use bevy_asset::Assets;
use bevy_ecs::system::{Query, Res, SystemParam};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::level::Level as LevelAsset;

use crate::{
    entity::{EntityData, EntityItem},
    level::{LevelData, LevelItem},
};

#[derive(SystemParam)]
pub struct LdtkQuery<'w, 's> {
    ldtk_levels: Res<'w, Assets<LevelAsset>>,
    ldtk_entities: Res<'w, Assets<EntityAsset>>,
    levels_query: Query<'w, 's, LevelData<'static>>,
    entities_query: Query<'w, 's, EntityData<'static>>,
}

impl LdtkQuery<'_, '_> {
    pub fn levels(&self) -> impl Iterator<Item = LevelItem> {
        self.levels_query
            .iter()
            .filter_map(|data| {
                self.ldtk_levels
                    .get(data.1.handle.id())
                    .map(|level_asset| (level_asset, data))
            })
            .map(|(level_asset, data)| LevelItem {
                asset: level_asset,
                data,
                query: self,
            })
    }

    pub fn entities(&self) -> impl Iterator<Item = EntityItem> {
        self.entities_query
            .iter()
            .filter_map(|data| {
                self.ldtk_entities
                    .get(data.1.handle.id())
                    .map(|entity_asset| (entity_asset, data))
            })
            .map(|(entity_asset, data)| EntityItem {
                asset: entity_asset,
                data,
                query: self,
            })
    }
}
