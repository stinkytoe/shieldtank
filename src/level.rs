use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::change_detection::DetectChanges;
use bevy_ecs::entity::Entity;
use bevy_ecs::entity::Entity as EcsEntity; // NOTE: Is this a good idea?
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, Query, Res};
use bevy_ecs::world::Ref;
use bevy_hierarchy::{BuildChildren, ChildBuild};
use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_log::debug;
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;

use crate::component::{DoFinalizeEvent, LdtkComponent, LdtkComponentExt};
use crate::layer::Layer;
use crate::level_background::LevelBackground;
use crate::project_config::ProjectConfig;
use crate::query::LdtkQuery;
use crate::{bad_handle, Result};

pub type Level = LdtkComponent<LevelAsset>;

pub type LevelData<'a> = (
    EcsEntity,
    Ref<'a, Level>,
    Ref<'a, Visibility>,
    Ref<'a, Transform>,
    Option<Ref<'a, LevelBackground>>,
);

pub struct LevelItem<'a> {
    pub asset: &'a LevelAsset,
    pub data: LevelData<'a>,
    pub query: &'a LdtkQuery<'a, 'a>,
}

impl LevelItem<'_> {
    pub fn level_asset(&self) -> &LevelAsset {
        self.asset
    }

    pub fn ecs_entity(&self) -> EcsEntity {
        self.data.0
    }

    pub fn level(&self) -> &Level {
        &self.data.1
    }

    pub fn visibility(&self) -> &Visibility {
        &self.data.2
    }

    pub fn transform(&self) -> &Transform {
        &self.data.3
    }

    pub fn level_background(&self) -> Option<&LevelBackground> {
        self.data.4.as_deref()
    }
}

impl std::fmt::Debug for LevelItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EntityItem")
            .field("ecs_entity", &self.data.0)
            .field("identifier", &self.asset.identifier)
            .field("iid", &self.asset.iid)
            .finish()
    }
}

impl LevelItem<'_> {}

pub struct FilterIdentifier<'a, I>
where
    I: Iterator<Item = LevelItem<'a>>,
{
    iter: I,
    identifier: &'a str,
}

impl<'a, I> std::fmt::Debug for FilterIdentifier<'a, I>
where
    I: Iterator<Item = LevelItem<'a>>,
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
    I: Iterator<Item = LevelItem<'a>>,
{
    pub fn new(iter: I, identifier: &'a str) -> Self {
        Self { iter, identifier }
    }
}

impl<'a, I> Iterator for FilterIdentifier<'a, I>
where
    I: Iterator<Item = LevelItem<'a>>,
{
    type Item = LevelItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find(|item| item.asset.identifier == self.identifier)
    }
}

pub trait LevelWithIdentifierExt<'a>: Iterator<Item = LevelItem<'a>> + Sized {
    fn added(self) -> impl Iterator<Item = LevelItem<'a>> {
        self.filter(|item| item.data.1.is_added())
    }

    fn changed(self) -> impl Iterator<Item = LevelItem<'a>> {
        self.filter(|item| item.data.1.is_changed())
    }

    fn filter_identifier(self, identifier: &'a str) -> FilterIdentifier<'a, Self> {
        FilterIdentifier::new(self, identifier)
    }

    fn find_iid(mut self, iid: Iid) -> Option<LevelItem<'a>> {
        self.find(|item| item.asset.iid == iid)
    }
}

impl<'a, I: Iterator<Item = LevelItem<'a>>> LevelWithIdentifierExt<'a> for I {}

pub(crate) fn level_finalize_on_event(
    mut commands: Commands,
    mut events: EventReader<DoFinalizeEvent<LevelAsset>>,
    level_assets: Res<Assets<LevelAsset>>,
    config_assets: Res<Assets<ProjectConfig>>,
    query: Query<(Entity, &Level)>,
) -> Result<()> {
    events.read().try_for_each(|event| -> Result<()> {
        let DoFinalizeEvent {
            entity: event_entity,
            ..
        } = event;

        query
            .iter()
            .filter(|(entity, ..)| entity == event_entity)
            .try_for_each(|data| -> Result<()> {
                finalize(&mut commands, data, &level_assets, &config_assets)
            })
    })
}

fn finalize(
    commands: &mut Commands,
    (entity, level): (Entity, &Level),
    level_assets: &Assets<LevelAsset>,
    config_assets: &Assets<ProjectConfig>,
) -> Result<()> {
    let level_asset = level_assets
        .get(level.get_handle().id())
        .ok_or(bad_handle!(level.get_handle()))?;

    let project_config = config_assets
        .get(level.get_config_handle().id())
        .ok_or(bad_handle!(level.get_config_handle()))?;

    let name = Name::from(level_asset.identifier.clone());

    let translation = level_asset
        .location
        .extend((level_asset.world_depth as f32) * project_config.level_z_scale);
    let transform = Transform::from_translation(translation);

    let visibility = Visibility::default();

    let color = level_asset.bg_color;
    let size = level_asset.size;
    let background = level_asset.background.clone();
    let background = LevelBackground {
        color,
        size,
        background,
    };

    commands
        .entity(entity)
        .insert((name, transform, visibility, background))
        .with_children(|parent| {
            level_asset.layers.values().for_each(|layer_handle| {
                if project_config
                    .load_pattern
                    .handle_matches_pattern(layer_handle)
                {
                    parent.spawn(Layer {
                        handle: layer_handle.clone(),
                        config: level.get_config_handle(),
                    });
                }
            })
        });

    debug!("Level {:?} finalized!", level_asset.identifier);

    Ok(())
}
