use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::entity::Entity as EcsEntity; // NOTE: Is this a good idea?
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Res};
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

pub type EntityData = (
    EcsEntity,
    Ref<'static, Entity>,
    Ref<'static, Visibility>,
    Ref<'static, Transform>,
);

pub struct EntityItem<'a> {
    pub ldtk_entity: &'a EntityAsset,
    pub ecs_entity: EcsEntity,
    pub shieldtank_entity: Ref<'a, Entity>,
    pub visibility: Ref<'a, Visibility>,
    pub transform: Ref<'a, Transform>,
    pub query: &'a LdtkQuery<'a, 'a>,
}

impl std::fmt::Debug for EntityItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityItem")
            .field("ecs_entity", &self.ecs_entity)
            .field("shieldtank_entity", &self.shieldtank_entity)
            .field("ldtk_entity", &self.ldtk_entity)
            //.field("query", &self.query)
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
            .find(|item| item.ldtk_entity.identifier == self.identifier)
    }
}

pub trait EntityWithIdentifierExt<'a>: Iterator<Item = EntityItem<'a>> + Sized {
    fn added(self) -> impl Iterator<Item = EntityItem<'a>> {
        self.filter(|item| item.shieldtank_entity.is_added())
    }

    fn changed(self) -> impl Iterator<Item = EntityItem<'a>> {
        self.filter(|item| item.shieldtank_entity.is_changed())
    }

    fn filter_identifier(self, identifier: &'a str) -> FilterIdentifier<'a, Self> {
        FilterIdentifier::new(self, identifier)
    }

    fn find_iid(mut self, iid: Iid) -> Option<EntityItem<'a>> {
        self.find(|item| item.ldtk_entity.iid == iid)
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
