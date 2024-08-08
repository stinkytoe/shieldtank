use bevy::ecs::query::QueryEntityError;
use bevy::math::I64Vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::str::FromStr;
use thiserror::Error;

use crate::assets::event::LdkAssetEvent;
use crate::assets::traits::LdtkAsset;
use crate::iid::Iid;
use crate::iid::IidError;
use crate::iid::IidSet;
use crate::ldtk;
use crate::reexports::field_instance::FieldInstance;
use crate::reexports::field_instance::FieldInstanceError;
use crate::reexports::tileset_rectangle::TilesetRectangle;
use crate::system_params::LdtkQuery;
// use crate::system_params::query::LdtkQuery;
use crate::util::bevy_anchor_from_ldtk;
use crate::util::bevy_color_from_ldtk;
use crate::util::AnchorIntoError;
use crate::util::ColorParseError;

#[derive(Debug, Error)]
pub enum LdtkEntityAssetError {
    #[error(transparent)]
    IidError(#[from] IidError),
    #[error(transparent)]
    AnchorIntoError(#[from] AnchorIntoError),
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
    #[error(transparent)]
    FieldInstanceError(#[from] FieldInstanceError),
    #[error(transparent)]
    QueryEntityError(#[from] QueryEntityError),
    #[error("One world coord is Some(...) and the other is None!")]
    WorldCoordMixedOption,
    #[error("bad handle? {0:?}")]
    BadHandle(Handle<LdtkEntityAsset>),
    #[error("bad project iid? {0:?}")]
    BadProjectIid(Iid),
    #[error("bad tileset_definition uid? {0:?}")]
    BadTilesetDefUid(i64),
    #[error("Sprite where rel path is None?")]
    RelPathIsNone,
    #[error("Bad relative path! {0:?}")]
    BadRelPath(String),
}

#[derive(Asset, Debug, Reflect)]
pub struct LdtkEntityAsset {
    // NOTE: Internal fields
    pub(crate) iid: Iid,
    pub(crate) children: IidSet,
    pub(crate) identifier: String,
    pub(crate) project_iid: Iid,
    // NOTE: LDtk exports
    pub(crate) grid: I64Vec2,
    pub(crate) anchor: Anchor,
    pub(crate) smart_color: Color,
    pub(crate) tags: Vec<String>,
    pub(crate) tile: Option<TilesetRectangle>,
    pub(crate) world_location: Option<I64Vec2>,
    pub(crate) def_uid: i64,
    pub(crate) field_instances: Vec<FieldInstance>,
    pub(crate) size: I64Vec2,
    pub(crate) location: I64Vec2,
}

impl LdtkEntityAsset {
    pub(crate) fn new(
        value: &ldtk::EntityInstance,
        project_iid: Iid,
    ) -> Result<Self, LdtkEntityAssetError> {
        let iid = Iid::from_str(&value.iid)?;

        let children = IidSet::default();

        Ok(Self {
            iid,
            children,
            identifier: value.identifier.clone(),
            project_iid,
            grid: (value.grid[0], value.grid[1]).into(),
            anchor: bevy_anchor_from_ldtk(&value.pivot)?,
            smart_color: bevy_color_from_ldtk(&value.smart_color)?,
            tags: value.tags.clone(),
            tile: value.tile.as_ref().map(TilesetRectangle::new),
            world_location: match (value.world_x, value.world_y) {
                (None, None) => None,
                (Some(world_x), Some(world_y)) => Some((world_x, world_y).into()),
                _ => {
                    return Err(LdtkEntityAssetError::WorldCoordMixedOption);
                }
            },
            def_uid: value.def_uid,
            field_instances: value
                .field_instances
                .iter()
                .map(FieldInstance::new)
                .collect::<Result<_, _>>()?,
            size: (value.width, value.height).into(),
            location: (value.px[0], value.px[1]).into(),
        })
    }

    pub(crate) fn entity_tile_system(
        mut commands: Commands,
        mut events: EventReader<LdkAssetEvent<LdtkEntityAsset>>,
        entity_assets: Res<Assets<LdtkEntityAsset>>,
    ) -> Result<(), LdtkEntityAssetError> {
        for LdkAssetEvent::<LdtkEntityAsset>::Modified { entity, handle } in events.read() {
            trace!("entity_tile_system: {entity:?}");

            let entity_asset = entity_assets
                .get(handle.id())
                .ok_or(LdtkEntityAssetError::BadHandle(handle.clone()))?;

            if let Some(tile) = entity_asset.tile.as_ref() {
                commands.entity(*entity).insert((tile.clone(),));
            } else {
                commands.entity(*entity).remove::<TilesetRectangle>();
            }
        }
        Ok(())
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn handle_tile_system(
        mut commands: Commands,
        ldtk_query: LdtkQuery,
        entity_assets: Res<Assets<LdtkEntityAsset>>,
        mut modified_query: Query<
            (
                Entity,
                &Handle<LdtkEntityAsset>,
                &TilesetRectangle,
                Option<&mut Sprite>,
            ),
            Changed<TilesetRectangle>,
        >,
        mut removed_tiles: RemovedComponents<TilesetRectangle>,
    ) -> Result<(), LdtkEntityAssetError> {
        modified_query.iter_mut().try_for_each(
            |(entity, handle, tile, sprite)| -> Result<(), LdtkEntityAssetError> {
                let entity_asset = entity_assets
                    .get(handle.id())
                    .ok_or(LdtkEntityAssetError::BadHandle(handle.clone()))?;

                let project_asset = ldtk_query.get_project(entity_asset.project_iid).ok_or(
                    LdtkEntityAssetError::BadProjectIid(entity_asset.project_iid),
                )?;

                let tileset_definition = project_asset
                    .tileset_defs
                    .get(&tile.tileset_uid)
                    .ok_or(LdtkEntityAssetError::BadTilesetDefUid(tile.tileset_uid))?;

                let rel_path = tileset_definition
                    .rel_path
                    .as_ref()
                    .ok_or(LdtkEntityAssetError::RelPathIsNone)?;

                let tile_handle = project_asset
                    .tilesets
                    .get(rel_path)
                    .ok_or(LdtkEntityAssetError::BadRelPath(rel_path.clone()))?;

                commands.entity(entity).insert(tile_handle.clone());

                let custom_size = Some(tile.size);
                let rect = Some(Rect::from_corners(tile.location, tile.location + tile.size));
                let anchor = entity_asset.anchor;

                if let Some(mut sprite) = sprite {
                    sprite.custom_size = custom_size;
                    sprite.rect = rect;
                    sprite.anchor = anchor;
                } else {
                    commands.entity(entity).insert(Sprite {
                        custom_size,
                        rect,
                        anchor,
                        ..default()
                    });
                }

                Ok(())
            },
        )?;

        removed_tiles.read().for_each(|entity| {
            commands
                .entity(entity)
                .remove::<Handle<Image>>()
                .remove::<Sprite>();
        });

        Ok(())
    }
}

impl LdtkAsset for LdtkEntityAsset {
    fn iid(&self) -> crate::iid::Iid {
        self.iid
    }

    fn children(&self) -> &IidSet {
        &self.children
    }

    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn location(&self) -> Vec3 {
        Vec3::new(self.location.x as f32, -self.location.y as f32, 0.0)
    }

    fn asset_handle_from_project(
        project: &crate::prelude::LdtkProject,
        iid: Iid,
    ) -> Option<Handle<Self>> {
        project.entities.get(&iid).cloned()
    }
}
