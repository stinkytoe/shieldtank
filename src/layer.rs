use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
use bevy_ldtk_asset::layer::Layer as LayerAsset;
use bevy_ldtk_asset::layer_definition::LayerDefinition;
use bevy_ldtk_asset::prelude::IntGridValue;
use bevy_math::{Rect, Vec2};
use bevy_utils::error;

use crate::component::{FinalizeEvent, LdtkComponent};
use crate::entity::EntityItem;
use crate::impl_recurrent_identifer_iterator;
use crate::int_grid::IntGrid;
use crate::item::{LdtkItem, LdtkItemTrait};
use crate::level::LevelItem;
use crate::project::ProjectItem;
use crate::tiles::Tiles;
use crate::world::WorldItem;
use crate::{bad_ecs_entity, bad_handle, Result};

pub type LayerComponent = LdtkComponent<LayerAsset>;
pub type LayerItem<'a> = LdtkItem<'a, LayerAsset>;
impl_recurrent_identifer_iterator!(LayerAsset);

impl LayerItem<'_> {
    pub fn get_project(&self) -> Option<ProjectItem> {
        todo!()
    }
    pub fn get_world(&self) -> Option<WorldItem> {
        todo!()
    }
    pub fn get_level(&self) -> Option<LevelItem> {
        todo!()
    }
    pub fn iter_entities(&self) -> Option<EntityItem> {
        todo!()
    }

    pub fn entities_at(&self, location: Vec2) -> impl Iterator<Item = EntityItem> {
        vec![].into_iter()
    }

    pub fn location(&self) -> Vec2 {
        todo!()
    }
    pub fn location_in_project(&self) -> Vec2 {
        todo!()
    }
    pub fn location_in_world(&self) -> Vec2 {
        todo!()
    }
    pub fn location_in_level(&self) -> Vec2 {
        todo!()
    }

    pub fn contains_project_location(&self, location: Vec2) -> bool {
        todo!()
    }

    pub fn contains_world_location(&self, location: Vec2) -> bool {
        todo!()
    }

    pub fn contains_level_location(&self, location: Vec2) -> bool {
        todo!()
    }

    pub fn contains_layer_location(&self, location: Vec2) -> bool {
        let self_location = self.location();
        Rect::from_corners(
            self_location,
            self_location + Vec2::new(1.0, -1.0) * self.get_asset().grid_size.as_vec2(),
        )
        .contains(location)
    }
}

impl LayerItem<'_> {
    pub fn int_grid_at(&self, location: Vec2) -> Option<IntGridValue> {
        todo!()
    }
}
// impl LayerItem<'_> {
//     pub fn get_entities(&self) -> impl Iterator<Item = EntityItem> {
//         self.query
//             .entities()
//             .filter_map(|item| {
//                 let ecs_entity = item.get_ecs_entity();
//                 Some((item, self.query.parent_query.get(ecs_entity).ok()?))
//             })
//             .filter(|(_, parent)| parent.get() == self.get_ecs_entity())
//             .map(|(item, _)| item)
//     }
// }
//
// impl LayerItem<'_> {
//     pub fn get_level(&self) -> Option<LevelItem> {
//         let level_ecs_entity = self
//             .query
//             .parent_query
//             .get(self.get_ecs_entity())
//             .ok()
//             .map(|parent| parent.get())?;
//
//         self.query.get_level(level_ecs_entity).ok()
//     }
//
//     pub fn get_world(&self) -> Option<WorldItem> {
//         let level_ecs_entity = self
//             .query
//             .parent_query
//             .get(self.get_ecs_entity())
//             .ok()
//             .map(|parent| parent.get())?;
//
//         let world_ecs_entity = self
//             .query
//             .parent_query
//             .get(level_ecs_entity)
//             .ok()
//             .map(|parent| parent.get())?;
//
//         self.query.get_world(world_ecs_entity).ok()
//     }
//
//     pub fn get_project(&self) -> Option<ProjectItem> {
//         let level_ecs_entity = self
//             .query
//             .parent_query
//             .get(self.get_ecs_entity())
//             .ok()
//             .map(|parent| parent.get())?;
//
//         let world_ecs_entity = self
//             .query
//             .parent_query
//             .get(level_ecs_entity)
//             .ok()
//             .map(|parent| parent.get())?;
//
//         let project_ecs_entity = self
//             .query
//             .parent_query
//             .get(world_ecs_entity)
//             .ok()
//             .map(|parent| parent.get())?;
//
//         self.query.get_project(project_ecs_entity).ok()
//     }
// }
//
// impl LayerItem<'_> {
//     pub fn local_location_to_grid(&self, local_location: Vec2) -> Option<I64Vec2> {
//         let local_location = local_location.as_i64vec2() * I64Vec2::new(1, -1);
//
//         let local_grid = local_location / self.asset.grid_cell_size;
//
//         ((local_grid.x >= 0 && local_grid.y >= 0)
//             && (local_grid.x < self.asset.grid_size.x && local_grid.y < self.asset.grid_size.y))
//             .then_some(local_grid)
//     }
//
//     pub fn get_int_grid(&self) -> Option<&IntGrid> {
//         self.query.int_grid_query.get(self.get_ecs_entity()).ok()
//     }
//
//     pub fn get_grid_cell_size(&self) -> i64 {
//         self.get_asset().grid_cell_size
//     }
// }
//

pub struct LayerPlugin;
impl Plugin for LayerPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(Update, layer_finalize_int_grid_and_tiles.map(error));
    }
}

pub(crate) fn layer_finalize_int_grid_and_tiles(
    mut commands: Commands,
    mut events: EventReader<FinalizeEvent<LayerAsset>>,
    layer_assets: Res<Assets<LayerAsset>>,
    layer_definitions: Res<Assets<LayerDefinition>>,
    query: Query<&LayerComponent>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let FinalizeEvent { ecs_entity, .. } = event;

        let component = query
            .get(*ecs_entity)
            .map_err(|e| bad_ecs_entity!("bad ecs entity! {ecs_entity:?}: {e}"))?;

        let asset = layer_assets
            .get(component.handle.id())
            .ok_or(bad_handle!("bad handle! {:?}", component.handle))?;

        if let Some(tiles_layer) = asset.layer_type.get_tiles_layer() {
            let mut entity_commands = commands.entity(*ecs_entity);

            if !tiles_layer.tiles.is_empty() {
                let tiles = Tiles::new(tiles_layer);
                entity_commands.insert(tiles);
            }

            if !tiles_layer.int_grid.is_empty() {
                // TODO: Unguarded Assets::get(..) here. Probably fine, but should fix.
                let layer_definition = layer_definitions
                    .get(asset.layer_definition.id())
                    .ok_or(bad_handle!("bad handle! {:?}", asset.layer_definition))?;
                let int_grid = IntGrid::from_layer(asset, layer_definition)?;
                entity_commands.insert(int_grid);
            }
        }

        Ok(())
    })
}
