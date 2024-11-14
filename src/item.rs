use std::marker::PhantomData;

use bevy_app::{Plugin, Update};
use bevy_asset::Assets;
use bevy_core::Name;
use bevy_ecs::entity::Entity as EcsEntity;
use bevy_ecs::event::EventReader;
use bevy_ecs::system::{Commands, IntoSystem, Query, Res};
use bevy_ecs::world::Ref;
use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_log::{debug, trace};
use bevy_math::Vec2;
use bevy_render::view::Visibility;
use bevy_sprite::Sprite;
use bevy_transform::components::{GlobalTransform, Transform};
use bevy_utils::error;

use crate::asset_translation::LdtkAssetTranslation;
use crate::component::handle_ldtk_component_added;
use crate::component::send_finalize_if_ready;
use crate::component::AwaitingFinalize;
use crate::component::FinalizeEvent;
use crate::component::LdtkComponent;
use crate::project_config::ProjectConfig;
use crate::query::LdtkQuery;
use crate::{bad_ecs_entity, bad_handle, Result};

pub struct LdtkItem<'a, Asset>
where
    Asset: LdtkAsset + Sized + std::fmt::Debug,
{
    pub asset: &'a Asset,
    pub component: Ref<'a, LdtkComponent<Asset>>,
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

    fn get_query(&self) -> &LdtkQuery {
        self.query
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
    fn get_query(&self) -> &LdtkQuery;

    fn get_transform(&self) -> Option<&Transform> {
        self.get_query()
            .transform_query
            .get(self.get_ecs_entity())
            .ok()
    }

    fn get_global_transform(&self) -> Option<&GlobalTransform> {
        self.get_query()
            .global_transform_query
            .get(self.get_ecs_entity())
            .ok()
    }

    // TODO: Consider breaking this out into a specialization, so that only items which are
    // expected to have a sprite will inherit this method. (Level, Layer, Entity, ...)
    fn get_sprite(&self) -> Option<&Sprite> {
        self.get_query()
            .sprite_query
            .get(self.get_ecs_entity())
            .ok()
    }

    fn global_location_to_local_location(&self, global_location: Vec2) -> Option<Vec2> {
        let offset = self.get_global_transform()?.translation().truncate();

        Some(global_location - offset)
    }

    fn relative_location_to<OtherAsset>(&self, item: &LdtkItem<OtherAsset>) -> Option<Vec2>
    where
        OtherAsset: LdtkAsset + std::fmt::Debug,
    {
        let our_global_location = self.get_global_transform()?.translation().truncate();
        let their_global_location = item.get_global_transform()?.translation().truncate();

        Some(their_global_location - our_global_location)
    }
}

fn handle_finalize_event<Asset>(
    mut commands: Commands,
    mut events: EventReader<FinalizeEvent<Asset>>,
    assets: Res<Assets<Asset>>,
    configs: Res<Assets<ProjectConfig>>,
    query: Query<(EcsEntity, Ref<'static, LdtkComponent<Asset>>)>,
) -> Result<()>
where
    Asset: LdtkAsset + LdtkAssetTranslation + std::fmt::Debug,
{
    events.read().try_for_each(|event| -> Result<()> {
        trace!("FinalizeEvent: {event:?}");
        let FinalizeEvent { ecs_entity, .. } = event;

        let (ecs_entity, component) = query
            .get(*ecs_entity)
            .inspect(|ecs_entity| trace!("Finalizing ecs entity: {ecs_entity:?}"))
            .map_err(|e| bad_ecs_entity!("Bad ecs entity on finalize event! {e}"))?;

        let asset = assets.get(component.handle.id()).ok_or(bad_handle!(
            "Bad handle on finalize event! {:?}",
            component.handle
        ))?;

        let config = configs
            .get(component.config.id())
            .ok_or(bad_handle!("Bad config handle! {:?}", component.config))?;

        let name = asset.get_identifier().to_string();
        commands.entity(ecs_entity).insert(Name::new(name));

        let translation = asset.get_translation(config);
        commands
            .entity(ecs_entity)
            .insert(Transform::from_translation(translation));

        commands.entity(ecs_entity).insert(Visibility::default());

        debug!("Finalized {}!", asset.get_identifier());

        Ok(())
    })
}

pub struct LdtkAssetPlugin<Asset>
where
    Asset: LdtkAsset + Sized,
{
    _phantom: PhantomData<Asset>,
}

impl<Asset> Plugin for LdtkAssetPlugin<Asset>
where
    Asset: LdtkAsset + LdtkAssetTranslation + Sized + std::fmt::Debug,
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
