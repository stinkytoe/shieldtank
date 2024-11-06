use bevy_asset::Assets;
use bevy_ecs::system::{Query, Res, SystemParam};
use bevy_ldtk_asset::entity::Entity as EntityAsset;

use crate::entity::{EntityData, EntityItem};

#[derive(SystemParam)]
pub struct LdtkQuery<'w, 's> {
    ldtk_entities: Res<'w, Assets<EntityAsset>>,
    entities_query: Query<'w, 's, EntityData>,
}

impl LdtkQuery<'_, '_> {
    pub fn entities(&self) -> impl Iterator<Item = EntityItem> {
        self.entities_query
            .iter()
            .filter_map(|(ecs_entity, shieldtank_entity, visibility, transform)| {
                self.ldtk_entities
                    .get(shieldtank_entity.handle.id())
                    .map(|ldtk_entity| {
                        (
                            ldtk_entity,
                            ecs_entity,
                            shieldtank_entity,
                            visibility,
                            transform,
                        )
                    })
            })
            .map(
                |(ldtk_entity, ecs_entity, shieldtank_entity, visibility, transform)| EntityItem {
                    ldtk_entity,
                    ecs_entity,
                    shieldtank_entity,
                    visibility,
                    transform,
                    query: self,
                },
            )
    }
}
