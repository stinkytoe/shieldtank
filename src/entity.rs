use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_log::debug;
use bevy_math::{I64Vec2, Vec2};
use bevy_utils::error;

use crate::component::{FinalizeEvent, LdtkComponent};
use crate::impl_recurrent_identifer_iterator;
use crate::item::{LdtkItem, LdtkItemTrait};
use crate::item_iterator::LdtkItemIterator;
use crate::layer::LayerItem;
use crate::level::LevelItem;
use crate::tileset_rectangle::TilesetRectangle;
use crate::{bad_ecs_entity, bad_handle, Result};

pub type Entity = LdtkComponent<EntityAsset>;
pub type EntityItem<'a> = LdtkItem<'a, EntityAsset>;

impl EntityItem<'_> {
    pub fn get_grid_location(&self) -> Option<I64Vec2> {
        self.query
            .transform_query
            .get(self.get_ecs_entity())
            // TODO: Add an inspect(..) step here to print an error if failed
            .ok()
            .map(|transform| transform.translation.truncate() * Vec2::new(1.0, -1.0))
            .and_then(|location| Some((location, self.get_layer()?)))
            .map(|(translation, layer)| {
                let anchor = self.asset.anchor.as_vec();
                debug!("get_grid_coordinates anchor {anchor}");
                let size = layer.asset.grid_cell_size;
                debug!("get_grid_coordinates grid_cell_size {size}");
                // NOTE: If there are any rounding shennanigans, look here first
                let ax_times_size = ((size as f32) * -anchor.x) as i64;
                let ay_times_size = ((size as f32) * anchor.y) as i64;

                let x = ax_times_size - size / 2;
                let y = ay_times_size - size / 2;

                let offset = I64Vec2::new(x, y);

                (translation.as_i64vec2() + offset, layer)
            })
            .filter(|(location, layer)| {
                let total_grid_size = layer.asset.grid_size * layer.asset.grid_cell_size;
                location.x < total_grid_size.x && location.y < total_grid_size.y
            })
            .map(|(location, layer)| location / layer.asset.grid_cell_size)
    }

    pub fn get_layer(&self) -> Option<LayerItem<'_>> {
        self.query
            .parent_query
            .get(self.get_ecs_entity())
            .ok()
            .map(|parent| parent.get())
            .and_then(|parent_ecs_entity| self.query.layers().find_ecs_entity(parent_ecs_entity))
    }

    /// Returns the location of this entity on its containing layer.
    ///
    /// If there is no parent layer, or if we don't have a Transform component, then this returns
    /// None. Otherwise it returns the location on the layer in pixel space, wrapped in Ok(..)
    pub fn get_layer_location(&self) -> Option<Vec2> {
        Some(self.get_transform()?.translation.truncate())
    }

    /// Returns the location of this entity on its containing layer.
    ///
    /// If there is no parent layer, or grantparent level, or if we don't have a Transform component,
    /// then this returns None. Otherwise it returns the location on the layer in pixel space, wrapped in Ok(..)
    pub fn get_level_location(&self) -> Option<Vec2> {
        let layer_location = self.get_layer_location()?;
        let layer_offset = self.get_layer()?.get_transform()?.translation.truncate();

        Some(layer_offset + layer_location)
    }

    pub fn get_local_location(&self) -> Option<Vec2> {
        let transform = self.get_transform()?;

        Some(transform.translation.truncate())
    }

    pub fn get_global_location(&self) -> Option<Vec2> {
        let global_transform = self.get_global_transform()?;

        Some(global_transform.translation().truncate())
    }

    pub fn get_level(&self) -> Option<LevelItem<'_>> {
        let layer_ecs_entity = self
            .query
            .parent_query
            .get(self.get_ecs_entity())
            .ok()
            .map(|parent| parent.get())?;

        let level_ecs_entity = self
            .query
            .parent_query
            .get(layer_ecs_entity)
            .ok()
            .map(|parent| parent.get())?;

        self.query.levels().find_ecs_entity(level_ecs_entity)
    }
}

impl_recurrent_identifer_iterator!(EntityAsset);

pub trait EntityItemIteratorExt<'a>
where
    Self: Iterator<Item = EntityItem<'a>> + Sized,
{
    fn filter_tag(self, tag: &'a str) -> EntityFilterTagsIterator<'a, Self> {
        EntityFilterTagsIterator { iter: self, tag }
    }
}

impl<'a, Iter> EntityItemIteratorExt<'a> for Iter where Iter: Iterator<Item = EntityItem<'a>> {}

pub struct EntityFilterTagsIterator<'a, Iter>
where
    Iter: Iterator<Item = EntityItem<'a>>,
{
    iter: Iter,
    tag: &'a str,
}

impl<'a, Iter> std::fmt::Debug for EntityFilterTagsIterator<'a, Iter>
where
    Iter: Iterator<Item = EntityItem<'a>>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityFilterTagsIterator")
            //.field("iter", &self.iter)
            .field("tag", &self.tag)
            .finish()
    }
}

impl<'a, Iter> Iterator for EntityFilterTagsIterator<'a, Iter>
where
    Iter: Iterator<Item = EntityItem<'a>>,
{
    type Item = EntityItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|item| {
            item.asset
                .tags
                .iter()
                .any(|tag_inner| tag_inner == self.tag)
        })
    }
}

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(Update, entity_finalize_tileset_rectangle.map(error));
    }
}

pub(crate) fn entity_finalize_tileset_rectangle(
    mut commands: Commands,
    mut events: EventReader<FinalizeEvent<EntityAsset>>,
    entity_assets: Res<Assets<EntityAsset>>,
    query: Query<&Entity>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let FinalizeEvent { ecs_entity, .. } = event;

        let component = query
            .get(*ecs_entity)
            .map_err(|e| bad_ecs_entity!("bad ecs entity! {ecs_entity:?}: {e}"))?;

        let asset = entity_assets
            .get(component.handle.id())
            .ok_or(bad_handle!("bad handle! {:?}", component.handle))?;

        if let Some(tile) = asset.tile.as_ref() {
            commands.entity(*ecs_entity).insert(TilesetRectangle {
                anchor: asset.anchor,
                tile: tile.clone(),
            });
        }

        Ok(())
    })
}
