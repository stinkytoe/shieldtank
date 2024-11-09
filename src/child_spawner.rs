use std::marker::PhantomData;

use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res, ResMut};
use bevy_ecs::world::Ref;
use bevy_hierarchy::{BuildChildren, Children};
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAssetWithChildren;
use bevy_log::trace;
use bevy_utils::error;

use crate::component::AwaitingFinalize;
use crate::component::FinalizeEvent;
use crate::component::LdtkComponent;
use crate::{bad_ecs_entity, bad_handle, Result};

fn handle_spawn_children<ParentAsset, ChildAsset>(
    mut commands: Commands,
    mut events: EventReader<FinalizeEvent<ParentAsset>>,
    mut awaiting_finalize: ResMut<AwaitingFinalize<ParentAsset>>,
    parent_assets: Res<Assets<ParentAsset>>,
    parent_query: Query<(Ref<'static, LdtkComponent<ParentAsset>>, Option<&Children>)>,
    children_query: Query<Ref<'static, LdtkComponent<ChildAsset>>>,
) -> Result<()>
where
    ParentAsset: LdtkAsset + LdtkAssetWithChildren<ChildAsset> + std::fmt::Debug,
    ChildAsset: LdtkAsset,
{
    events.read().try_for_each(|event| -> Result<()> {
        let FinalizeEvent { ecs_entity, .. } = event;

        let (parent_component, children) = parent_query.get(*ecs_entity).map_err(|e| {
            bad_ecs_entity!("bad entity when spawning children! {:?}! {e}", ecs_entity)
        })?;

        let parent_asset = parent_assets
            .get(parent_component.handle.id())
            .ok_or(bad_handle!("bad handle! {:?}", parent_component.handle))?;

        parent_asset
            .get_children()
            .try_for_each(|child_handle| -> Result<()> {
                let children_with_handle: Vec<EcsEntity> = if let Some(children) = children {
                    // FIXME: This whole section could use some love
                    children
                        .iter()
                        // TODO: rewrite and don't ignore the error
                        .filter_map(|&child_ecs_entity| {
                            Some((child_ecs_entity, children_query.get(child_ecs_entity).ok()?))
                        })
                        .filter(|(_, spawned_child_component)| {
                            spawned_child_component.handle == *child_handle
                        })
                        .map(|(child_ecs_entity, _)| child_ecs_entity)
                        .collect()
                } else {
                    vec![]
                };

                if children_with_handle.is_empty() {
                    trace!("Spawning new entity! {child_handle:?}");
                    commands
                        .entity(*ecs_entity)
                        .with_child(LdtkComponent::<ChildAsset> {
                            handle: child_handle.clone(),
                            config: parent_component.config.clone(),
                        });
                } else {
                    children_with_handle.iter().for_each(|&child_ecs_entity| {
                        trace!(
                            "Parent {} spawning child: {child_ecs_entity:?}",
                            parent_asset.get_identifier()
                        );
                        awaiting_finalize.map.insert(child_ecs_entity);
                    });
                }

                Ok(())
            })
    })
}

pub struct LdtkChildSpawnerPlugin<ParentAsset, ChildAsset>
where
    ParentAsset: LdtkAsset + LdtkAssetWithChildren<ChildAsset>,
    ChildAsset: LdtkAsset,
{
    _phantom: PhantomData<(ParentAsset, ChildAsset)>,
}

impl<ParentAsset, ChildAsset> Plugin for LdtkChildSpawnerPlugin<ParentAsset, ChildAsset>
where
    ParentAsset: LdtkAsset + LdtkAssetWithChildren<ChildAsset> + std::fmt::Debug,
    ChildAsset: LdtkAsset,
{
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            Update,
            handle_spawn_children::<ParentAsset, ChildAsset>.map(error),
        );
    }
}

impl<ParentAsset, ChildAsset> Default for LdtkChildSpawnerPlugin<ParentAsset, ChildAsset>
where
    ParentAsset: LdtkAsset + LdtkAssetWithChildren<ChildAsset>,
    ChildAsset: LdtkAsset,
{
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}
