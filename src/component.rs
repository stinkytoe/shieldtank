use std::marker::PhantomData;

use bevy_asset::{AssetEvent, AssetServer, Handle};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::event::{Event, EventReader, EventWriter};
use bevy_ecs::query::Changed;
use bevy_ecs::system::Resource;
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_log::trace;
use bevy_reflect::Reflect;
use bevy_utils::HashSet;

use crate::project_config::ProjectConfig;
use crate::Result;

#[derive(Component, Debug, Reflect)]
pub struct LdtkComponent<Asset: LdtkAsset> {
    pub handle: Handle<Asset>,
    pub config: Handle<ProjectConfig>,
}

impl<Asset: LdtkAsset> LdtkComponentExt<Asset> for LdtkComponent<Asset> {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        asset_server.is_loaded_with_dependencies(self.handle.id())
        //&& asset_server.is_loaded_with_dependencies(self.config.id())
    }

    fn get_handle(&self) -> Handle<Asset> {
        self.handle.clone()
    }

    fn get_config_handle(&self) -> Handle<ProjectConfig> {
        self.config.clone()
    }
}

pub(crate) trait LdtkComponentExt<Asset: LdtkAsset> {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool;
    fn get_handle(&self) -> Handle<Asset>;
    fn get_config_handle(&self) -> Handle<ProjectConfig>;
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn handle_ldtk_component_added<Asset: LdtkAsset>(
    mut project_events: EventReader<AssetEvent<Asset>>,
    mut config_events: EventReader<AssetEvent<ProjectConfig>>,
    mut awaiting_finalize: ResMut<AwaitingFinalize<Asset>>,
    query: Query<(&LdtkComponent<Asset>, EcsEntity)>,
    query_changed: Query<(&LdtkComponent<Asset>, EcsEntity), Changed<LdtkComponent<Asset>>>,
) -> Result<()> {
    let with_added_component_handle: HashSet<EcsEntity> = project_events
        .read()
        .filter_map(|event| match event {
            AssetEvent::Modified { id } => Some(*id),
            _ => None,
        })
        .inspect(|_| trace!("Modified event for {}!", stringify!(A)))
        .flat_map(|id| {
            query
                .iter()
                .filter(move |(component, ..)| component.get_handle().id() == id)
                .map(|(_, entity, ..)| entity)
        })
        .collect();

    let with_added_config_handle: HashSet<EcsEntity> = config_events
        .read()
        .filter_map(|event| match event {
            AssetEvent::Modified { id } => Some(*id),
            _ => None,
        })
        .inspect(|_| trace!("Modified event for Project Config!"))
        .flat_map(|id| {
            query
                .iter()
                .filter(move |(component, ..)| component.get_config_handle().id() == id)
                .map(|(_, entity, ..)| entity)
        })
        .collect();

    let with_changed_component: HashSet<EcsEntity> = query_changed
        .iter()
        .inspect(|(_, entity, ..)| trace!("Component changed for: {entity:?}"))
        .map(|(_, entity, ..)| entity)
        .collect();

    let entities_to_finalize: HashSet<EcsEntity> = with_added_component_handle
        .into_iter()
        .chain(with_added_config_handle)
        .chain(with_changed_component)
        .collect();

    entities_to_finalize
        .into_iter()
        .try_for_each(|entity| -> Result<()> {
            awaiting_finalize.map.insert(entity);
            Ok(())
        })
}

#[derive(Debug, Reflect, Resource)]
pub(crate) struct AwaitingFinalize<Asset: LdtkAsset> {
    pub(crate) map: HashSet<EcsEntity>,
    _phantom: PhantomData<Asset>,
}

impl<Asset: LdtkAsset> Default for AwaitingFinalize<Asset> {
    fn default() -> Self {
        Self {
            map: Default::default(),
            _phantom: Default::default(),
        }
    }
}

#[derive(Event, Debug, Reflect)]
pub(crate) struct FinalizeEvent<Asset: LdtkAsset> {
    pub ecs_entity: EcsEntity,
    _phantom: PhantomData<Asset>,
}

impl<Asset: LdtkAsset> FinalizeEvent<Asset> {
    pub(crate) fn new(entity: EcsEntity) -> Self {
        Self {
            ecs_entity: entity,
            _phantom: PhantomData,
        }
    }
}

pub(crate) fn send_finalize_if_ready<Asset: LdtkAsset>(
    asset_server: Res<AssetServer>,
    mut finalize_events: EventWriter<FinalizeEvent<Asset>>,
    mut awaiting_finalize: ResMut<AwaitingFinalize<Asset>>,
    query: Query<(EcsEntity, &LdtkComponent<Asset>)>,
) {
    awaiting_finalize.map.retain(|&entity| {
        trace!("waiting on entity: {entity:?}");

        let Ok((entity, component)) = query.get(entity) else {
            return true;
        };

        if component.is_loaded(&asset_server) {
            trace!("Sending finalize message to: {entity:?}");
            finalize_events.send(FinalizeEvent::new(entity));
            false
        } else {
            true
        }
    });
}
