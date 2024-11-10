use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
use bevy_ldtk_asset::layer_definition::IntGridValue;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_math::{Rect, Vec2};
use bevy_utils::error;

use crate::component::{FinalizeEvent, LdtkComponent};
use crate::item::{LdtkItem, LdtkItemTrait};
use crate::item_iterator::LdtkItemIterator;
use crate::layer::LayerItem;
use crate::level_background::LevelBackground;
use crate::{bad_ecs_entity, bad_handle, impl_unique_identifer_iterator, Result};

pub type Level = LdtkComponent<LevelAsset>;
pub type LevelItem<'a> = LdtkItem<'a, LevelAsset>;

impl_unique_identifer_iterator!(LevelAsset);

impl LevelItem<'_> {
    /// Checks if a global location is in this level's bounds
    ///
    /// We define the region as:
    /// * top-left corner being the level's translation in the ECS, projected to the XY plane.
    /// * bottom-right corner being the top-left corner plus the layer asset's reported size.
    ///
    /// Comparison is inclusive for the top left corner, and exclusive for the bottom right corner.
    pub fn global_location_is_in_bounds(&self, global_location: Vec2) -> bool {
        let Some(global_transform) = self.get_global_transform() else {
            return false;
        };

        let p0 = global_transform.translation().truncate();
        let p1 = p0 + self.asset.size * Vec2::new(1.0, -1.0);

        Rect::from_corners(p0, p1).contains(global_location)
    }

    pub fn int_grid_value_at_global_location(&self, global_location: Vec2) -> Option<IntGridValue> {
        let children = self.query.children_query.get(self.get_ecs_entity()).ok()?;

        let mut children: Vec<LayerItem> = children
            .iter()
            .filter_map(|&child_ecs_entity| self.query.layers().find_ecs_entity(child_ecs_entity))
            .collect();

        // unwrap is OK here because the above collect wouldn't have yielded anything that didn't
        // have a global_transform component.
        #[allow(clippy::unwrap_used)]
        children.sort_by(|a, b| {
            let a_z = a.get_global_transform().unwrap().translation().z;
            let b_z = b.get_global_transform().unwrap().translation().z;
            // intentionally reversed, so we will search nearest to farthest when looking down in
            // the world from above.
            b_z.partial_cmp(&a_z).unwrap()
        });

        children.iter().find_map(|layer_item| {
            let local_location = layer_item.global_location_to_local_location(global_location)?;

            let layer_grid = layer_item.local_location_to_grid(local_location)?;

            let int_grid = layer_item.get_int_grid()?;

            int_grid.get(layer_grid)
        })
    }
}

pub trait LevelItemIteratorExt<'a>
where
    Self: Iterator<Item = LevelItem<'a>> + Sized,
{
    fn filter_global_location(
        self,
        global_location: Vec2,
    ) -> LevelFilterGlobalLocationIterator<'a, Self> {
        LevelFilterGlobalLocationIterator {
            iter: self,
            global_location,
        }
    }
}

impl<'a, Iter> LevelItemIteratorExt<'a> for Iter where Iter: Iterator<Item = LevelItem<'a>> {}

pub struct LevelFilterGlobalLocationIterator<'a, Iter>
where
    Iter: Iterator<Item = LevelItem<'a>>,
{
    iter: Iter,
    global_location: Vec2,
}

impl<'a, Iter> std::fmt::Debug for LevelFilterGlobalLocationIterator<'a, Iter>
where
    Iter: Iterator<Item = LevelItem<'a>>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LevelFilterTagsIterator")
            //.field("iter", &self.iter)
            .field("location", &self.global_location)
            .finish()
    }
}

impl<'a, Iter> Iterator for LevelFilterGlobalLocationIterator<'a, Iter>
where
    Iter: Iterator<Item = LevelItem<'a>>,
{
    type Item = LevelItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|item| {
            item.global_location_is_in_bounds(self.global_location)
                .then_some(item)
        })
    }
}

pub(crate) fn level_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<FinalizeEvent<LevelAsset>>,
    level_assets: Res<Assets<LevelAsset>>,
    query: Query<&Level>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let FinalizeEvent { ecs_entity, .. } = event;

        let component = query
            .get(*ecs_entity)
            .map_err(|e| bad_ecs_entity!("bad ecs entity! {ecs_entity:?}: {e}"))?;

        let asset = level_assets
            .get(component.handle.id())
            .ok_or(bad_handle!("bad handle! {:?}", component.handle))?;

        let level_background = LevelBackground::new(asset);

        commands.entity(*ecs_entity).insert(level_background);

        Ok(())
    })
}

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(Update, level_finalize_on_event.map(error));
    }
}
