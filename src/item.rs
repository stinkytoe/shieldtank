use std::marker::PhantomData;

use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res, ResMut};
use bevy_ecs::world::Ref;
use bevy_hierarchy::{BuildChildren, Children};
use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAssetWithChildren;
use bevy_log::{debug, trace};
use bevy_render::view::Visibility;
use bevy_transform::components::Transform;
use bevy_utils::error;

use crate::component::handle_ldtk_component_added;
use crate::component::send_finalize_if_ready;
use crate::component::AwaitingFinalize;
use crate::component::FinalizeEvent;
use crate::component::LdtkComponent;
use crate::query::LdtkQuery;
use crate::{bad_ecs_entity, bad_handle, Result};

pub struct LdtkItem<'a, Asset>
where
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
    pub asset: &'a Asset,
    pub ecs_entity: EcsEntity,
    pub query: &'a LdtkQuery<'a, 'a>,
}

impl<'a, Asset> std::fmt::Debug for LdtkItem<'a, Asset>
where
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LdtkItem")
            .field("asset", &self.asset)
            .field("ecs_entity", &self.ecs_entity)
            //.field("query", &self.query)
            .finish()
    }
}

impl<'a, Asset> LdtkItemTrait<Asset> for LdtkItem<'a, Asset>
where
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
    fn get_ecs_entity(&self) -> EcsEntity {
        self.ecs_entity
    }

    fn get_asset(&self) -> &Asset {
        self.asset
    }

    fn get_iid(&self) -> Iid {
        self.asset.get_iid()
    }

    fn get_identifier(&self) -> &str {
        self.asset.get_identifier()
    }
}

pub trait LdtkItemTrait<Asset>
where
    Self: std::fmt::Debug,
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
    fn get_ecs_entity(&self) -> EcsEntity;
    fn get_asset(&self) -> &Asset;
    fn get_iid(&self) -> Iid;
    fn get_identifier(&self) -> &str;
    //fn get_transform(&self) -> &Ref<Transform>;
    //fn get_query(&self) -> &LdtkQuery;
}

fn handle_finalize_event<Asset>(
    mut commands: Commands,
    mut events: EventReader<FinalizeEvent<Asset>>,
    assets: Res<Assets<Asset>>,
    query: Query<(EcsEntity, Ref<'static, LdtkComponent<Asset>>)>,
) -> Result<()>
where
    Asset: LdtkAsset + std::fmt::Debug,
{
    events.read().try_for_each(|event| -> Result<()> {
        trace!("FinalizeEvent: {event:?}");
        let FinalizeEvent {
            entity: ecs_entity, ..
        } = event;

        let (ecs_entity, component) = query
            .get(*ecs_entity)
            .inspect(|ecs_entity| trace!("Finalizing ecs entity: {ecs_entity:?}"))
            .map_err(|e| bad_ecs_entity!("Bad ecs entity on finalize event! {e}"))?;

        let asset = assets.get(component.handle.id()).ok_or(bad_handle!(
            "Bad handle on finalize event! {:?}",
            component.handle
        ))?;

        let name = asset.get_identifier().to_string();
        commands.entity(ecs_entity).insert(Name::new(name));

        let translation = asset.get_translation();
        commands
            .entity(ecs_entity)
            .insert(Transform::from_translation(translation));

        commands.entity(ecs_entity).insert(Visibility::default());

        debug!("Finalized {}!", asset.get_identifier());

        Ok(())
    })
}

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
        let FinalizeEvent {
            entity: ecs_entity, ..
        } = event;

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

pub trait LdtkItemIterator<Asset>
where
    Self: Iterator + Sized,
    Self::Item: LdtkItemTrait<Asset>,
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
    fn find_iid(mut self, iid: Iid) -> Option<impl LdtkItemTrait<Asset>> {
        self.find(|item| item.get_iid() == iid)
    }
}

impl<Asset, Iter> LdtkItemIterator<Asset> for Iter
where
    Iter: Iterator + Sized,
    Iter::Item: LdtkItemTrait<Asset>,
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
}

pub struct LdtkAssetPlugin<Asset>
where
    Asset: LdtkAsset + Sized,
{
    _phantom: PhantomData<Asset>,
}

impl<Asset> Plugin for LdtkAssetPlugin<Asset>
where
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            Update,
            (
                handle_ldtk_component_added::<Asset>.map(error),
                send_finalize_if_ready::<Asset>,
                handle_finalize_event::<Asset>.map(error),
            ),
        )
        .insert_resource(AwaitingFinalize::<Asset>::default())
        .add_event::<FinalizeEvent<Asset>>();
    }
}

impl<Asset> Default for LdtkAssetPlugin<Asset>
where
    Asset: LdtkAsset + Sized,
{
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
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
