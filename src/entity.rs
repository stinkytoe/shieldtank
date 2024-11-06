use bevy_asset::{Asset, Assets};
use bevy_core::Name;
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity as EcsEntity; // NOTE: Is this a good idea?
use bevy_ecs::event::EventReader;
use bevy_ecs::query::{self, QueryData};
use bevy_ecs::system::{Commands, Query, Res, SystemParam};
use bevy_ecs::world::Ref;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_ldtk_asset::iid::Iid;
use bevy_log::debug;
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::component::{DoFinalizeEvent, LdtkComponent, LdtkComponentExt};
use crate::query::LdtkQuery;
use crate::tileset_rectangle::TilesetRectangle;
use crate::{bad_handle, Result};

pub type Entity = LdtkComponent<EntityAsset>;

pub struct EntityData2 {}

pub type EntityData<'a> = (
    EcsEntity,
    Ref<'a, Entity>,
    Ref<'a, Visibility>,
    Ref<'a, Transform>,
);

pub struct EntityItem<'a> {
    pub asset: &'a EntityAsset,
    pub data: EntityData<'a>,
    pub query: &'a LdtkQuery<'a, 'a>,
}

impl EntityItem<'_> {
    pub fn entity_asset(&self) -> &EntityAsset {
        self.asset
    }

    pub fn ecs_entity(&self) -> EcsEntity {
        self.data.0
    }

    pub fn entity(&self) -> &Entity {
        &self.data.1
    }

    pub fn visibility(&self) -> &Visibility {
        &self.data.2
    }

    pub fn transform(&self) -> &Transform {
        &self.data.3
    }
}

impl std::fmt::Debug for EntityItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityItem")
            .field("ecs_entity", &self.data.0)
            .field("identifier", &self.asset.identifier)
            .field("iid", &self.asset.iid)
            .finish()
    }
}

impl EntityItem<'_> {}

pub struct FilterIdentifier<'a, I>
where
    I: Iterator<Item = EntityItem<'a>>,
{
    iter: I,
    identifier: &'a str,
}

impl<'a, I> std::fmt::Debug for FilterIdentifier<'a, I>
where
    I: Iterator<Item = EntityItem<'a>>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WithIdentifier")
            //.field("iter", &self.iter)
            .field("identifier", &self.identifier)
            .finish()
    }
}

impl<'a, I> FilterIdentifier<'a, I>
where
    I: Iterator<Item = EntityItem<'a>>,
{
    pub fn new(iter: I, identifier: &'a str) -> Self {
        Self { iter, identifier }
    }
}

impl<'a, I> Iterator for FilterIdentifier<'a, I>
where
    I: Iterator<Item = EntityItem<'a>>,
{
    type Item = EntityItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find(|item| item.asset.identifier == self.identifier)
    }
}

pub trait EntityWithIdentifierExt<'a>: Iterator<Item = EntityItem<'a>> + Sized {
    fn added(self) -> impl Iterator<Item = EntityItem<'a>> {
        self.filter(|item| item.data.1.is_added())
    }

    fn changed(self) -> impl Iterator<Item = EntityItem<'a>> {
        self.filter(|item| item.data.1.is_changed())
    }

    fn filter_identifier(self, identifier: &'a str) -> FilterIdentifier<'a, Self> {
        FilterIdentifier::new(self, identifier)
    }

    fn find_iid(mut self, iid: Iid) -> Option<EntityItem<'a>> {
        self.find(|item| item.asset.iid == iid)
    }
}

impl<'a, I: Iterator<Item = EntityItem<'a>>> EntityWithIdentifierExt<'a> for I {}

pub(crate) fn entity_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<DoFinalizeEvent<EntityAsset>>,
    entity_assets: Res<Assets<EntityAsset>>,
    query: Query<(EcsEntity, &Entity)>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let DoFinalizeEvent {
            entity: event_entity,
            ..
        } = event;

        query
            .iter()
            .filter(|(ecs_entity, ..)| ecs_entity == event_entity)
            .try_for_each(|data| -> Result<()> { finalize(&mut commands, data, &entity_assets) })
    })
}

fn finalize(
    commands: &mut Commands,
    (ecs_entity, entity): (EcsEntity, &Entity),
    entity_assets: &Assets<EntityAsset>,
) -> Result<()> {
    let entity_asset = entity_assets
        .get(entity.get_handle().id())
        .ok_or(bad_handle!(entity.get_handle()))?;

    let name = Name::from(entity_asset.identifier.clone());

    let transform = Transform::from_translation(entity_asset.location.extend(0.0));

    let visibility = Visibility::default();

    let mut entity_commands = commands.entity(ecs_entity);

    entity_commands.insert((name, transform, visibility));

    if let Some(tile) = entity_asset.tile.as_ref() {
        entity_commands.insert(TilesetRectangle {
            anchor: entity_asset.anchor,
            tile: tile.clone(),
        });
    }

    debug!(
        "Entity {}@{} finalized!",
        entity_asset.identifier, entity_asset.iid
    );

    Ok(())
}
