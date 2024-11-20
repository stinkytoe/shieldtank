use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAssetWithFieldInstances;
use bevy_math::{I64Vec2, Rect, Vec2};
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
    /// Coordinates are in screen space: positive left, and negative down.
    ///
    /// If there is no parent layer, or if we don't have a Transform component, then this returns
    /// None.
    ///
    /// NOTE: This is not necessarily 'in-bounds'.
    pub fn get_layer_local_location(&self) -> Option<Vec2> {
        Some(self.get_transform()?.translation.truncate())
    }

    /// Returns the grid coordinates of the entity on its containing layer.
    ///
    /// Coordinates are in grid space: positive left, and positive down.
    ///
    /// If there is no parent layer, or if we don't have a Transform component, then this returns
    /// None.
    ///
    /// NOTE: This is not necessarily 'in-bounds'.
    pub fn get_layer_local_grid(&self) -> Option<I64Vec2> {
        let layer_location = self.get_layer_local_location()?.as_i64vec2();
        let layer_location = layer_location * I64Vec2::new(1, -1);

        let layer = self.get_layer()?;

        let layer_grid_cell_size = layer.get_asset().grid_cell_size;

        Some(layer_location / layer_grid_cell_size)
    }

    /// Returns the location of this entity on its containing layer.
    ///
    /// Coordinates are in screen space: positive left, and negative down.
    ///
    /// If there is no parent layer, or grantparent level, or if we don't have a Transform component,
    /// then this returns None. Otherwise it returns the location on the layer in pixel space, wrapped in Ok(..)
    pub fn get_level_local_location(&self) -> Option<Vec2> {
        let layer_location = self.get_layer_local_location()?;
        let layer_offset = self.get_layer()?.get_transform()?.translation.truncate();

        Some(layer_offset + layer_location)
    }

    /// Returns the location of this entity in global space.
    ///
    /// Coordinates are in screen space: positive left, and negative down.
    ///
    /// If entity does not have a GlobalTransform component, then this returns None.
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

    // NOTE: This might not correlate with the sprite image!
    pub fn get_region(&self) -> Rect {
        let size = self.get_asset().size;
        let anchor = self.get_asset().anchor.as_vec();
        let x = -anchor.x - 0.5;
        let y = -anchor.y + 0.5;

        let p0 = Vec2::new(x, y) * size;
        let p1 = p0 + size * Vec2::new(1.0, -1.0);

        Rect::from_corners(p0, p1)
    }

    pub fn global_location_is_in_bounds(&self, global_location: Vec2) -> bool {
        let Some(self_global_location) = self.get_global_location() else {
            return false;
        };

        let relative_location = global_location - self_global_location;

        self.get_region().contains(relative_location)
    }

    pub fn get_field_tile(&self, identifier: &str) -> Option<TilesetRectangle> {
        self.get_asset()
            .get_field_instance(identifier)?
            .get_tile()
            .map(|value| TilesetRectangle {
                anchor: self.get_asset().anchor,
                tile: value.clone(),
            })
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.get_asset().has_tag(tag)
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

    fn filter_global_location(
        self,
        global_location: Vec2,
    ) -> EntityFilterGlobalLocationIterator<'a, Self> {
        EntityFilterGlobalLocationIterator {
            iter: self,
            global_location,
        }
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

pub struct EntityFilterGlobalLocationIterator<'a, Iter>
where
    Iter: Iterator<Item = EntityItem<'a>>,
{
    iter: Iter,
    global_location: Vec2,
}

impl<'a, Iter> std::fmt::Debug for EntityFilterGlobalLocationIterator<'a, Iter>
where
    Iter: Iterator<Item = EntityItem<'a>>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityFilterTagsIterator")
            //.field("iter", &self.iter)
            .field("location", &self.global_location)
            .finish()
    }
}

impl<'a, Iter> Iterator for EntityFilterGlobalLocationIterator<'a, Iter>
where
    Iter: Iterator<Item = EntityItem<'a>>,
{
    type Item = EntityItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find_map(|item| {
            item.global_location_is_in_bounds(self.global_location)
                .then_some(item)
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
