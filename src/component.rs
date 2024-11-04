use std::marker::PhantomData;

use bevy_asset::{Asset, AssetEvent, AssetServer, Handle};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::event::{Event, EventReader, EventWriter};
use bevy_ecs::query::Changed;
use bevy_ecs::system::Resource;
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_log::{debug, trace};
use bevy_reflect::Reflect;
use bevy_utils::HashSet;

use crate::project_config::ProjectConfig;
use crate::Result;

#[derive(Component, Debug, Reflect)]
pub struct LdtkComponent<A: Asset> {
    pub handle: Handle<A>,
    pub config: Handle<ProjectConfig>,
}

impl<A: Asset> LdtkComponentExt<A> for LdtkComponent<A> {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        asset_server.is_loaded_with_dependencies(self.handle.id())
            && asset_server.is_loaded_with_dependencies(self.config.id())
    }

    fn get_handle(&self) -> Handle<A> {
        self.handle.clone()
    }

    fn get_config_handle(&self) -> Handle<ProjectConfig> {
        self.config.clone()
    }
}

pub(crate) trait LdtkComponentExt<A: Asset> {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool;
    fn get_handle(&self) -> Handle<A>;
    fn get_config_handle(&self) -> Handle<ProjectConfig>;
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn handle_ldtk_component_added<A: Asset>(
    mut project_events: EventReader<AssetEvent<A>>,
    mut config_events: EventReader<AssetEvent<ProjectConfig>>,
    mut awaiting_finalize: ResMut<AwaitingFinalize>,
    query: Query<(&LdtkComponent<A>, Entity)>,
    query_changed: Query<(&LdtkComponent<A>, Entity), Changed<LdtkComponent<A>>>,
) -> Result<()> {
    let with_added_component_handle: HashSet<Entity> = project_events
        .read()
        .filter_map(|event| match event {
            AssetEvent::Modified { id } => Some(*id),
            _ => None,
        })
        .inspect(|_| debug!("Modified event for {}!", stringify!(A)))
        .flat_map(|id| {
            query
                .iter()
                .filter(move |(component, ..)| component.get_handle().id() == id)
                .map(|(_, entity, ..)| entity)
        })
        .collect();

    let with_added_config_handle: HashSet<Entity> = config_events
        .read()
        .filter_map(|event| match event {
            AssetEvent::Modified { id } => Some(*id),
            _ => None,
        })
        .inspect(|_| debug!("Modified event for Project Config!"))
        .flat_map(|id| {
            query
                .iter()
                .filter(move |(component, ..)| component.get_config_handle().id() == id)
                .map(|(_, entity, ..)| entity)
        })
        .collect();

    let with_changed_component: HashSet<Entity> = query_changed
        .iter()
        .inspect(|(_, entity, ..)| debug!("Component changed for: {entity:?}"))
        .map(|(_, entity, ..)| entity)
        .collect();

    let entities_to_finalize: HashSet<Entity> = with_added_component_handle
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

#[derive(Debug, Default, Reflect, Resource)]
pub(crate) struct AwaitingFinalize {
    pub(crate) map: HashSet<Entity>,
}

#[derive(Event, Debug, Reflect)]
pub(crate) struct DoFinalizeEvent<A: Asset> {
    pub entity: Entity,
    _phantom: PhantomData<A>,
}

impl<A: Asset> DoFinalizeEvent<A> {
    pub(crate) fn new(entity: Entity) -> Self {
        Self {
            entity,
            _phantom: PhantomData,
        }
    }
}

pub(crate) fn send_finalize_if_ready<A: Asset>(
    asset_server: Res<AssetServer>,
    mut finalize_events: EventWriter<DoFinalizeEvent<A>>,
    mut awaiting_finalize: ResMut<AwaitingFinalize>,
    query: Query<(Entity, &LdtkComponent<A>)>,
) {
    awaiting_finalize.map.retain(|&entity| {
        trace!("waiting on entity: {entity:?}");

        let Ok((entity, component)) = query.get(entity) else {
            return true;
        };

        if component.is_loaded(&asset_server) {
            trace!("Sending finalize message to: {entity:?}");
            finalize_events.send(DoFinalizeEvent::new(entity));
            false
        } else {
            true
        }
    });
}
