use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::ReadAssetBytesError;
use bevy::prelude::*;
use bevy::tasks::block_on;
use bevy::utils::HashMap;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

use crate::assets::entity::LdtkEntityAsset;
use crate::assets::entity::LdtkEntityAssetError;
use crate::assets::layer::LdtkLayerAsset;
use crate::assets::layer::LdtkLayerAssetError;
use crate::assets::level::LdtkLevelAsset;
use crate::assets::level::LdtkLevelAssetError;
use crate::assets::project::LdtkProject;
use crate::assets::project::LdtkProjectError;
use crate::assets::project::LdtkProjectSettings;
use crate::assets::world::LdtkWorldAsset;
use crate::assets::world::LdtkWorldAssetError;
use crate::iid::Iid;
use crate::iid::IidError;
use crate::ldtk;
use crate::reexports::layer_definition::LayerDefinition;
use crate::reexports::layer_definition::LayerDefinitionFromError;
use crate::reexports::tileset_definition::TilesetDefinition;
use crate::util::bevy_color_from_ldtk;
use crate::util::ldtk_path_to_bevy_path;
use crate::util::ColorParseError;

#[derive(Debug, Error)]
pub(crate) enum LdtkProjectLoaderError {
    #[error(transparent)]
    IoErrpr(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    LdtkProjectError(#[from] LdtkProjectError),
    #[error(transparent)]
    ReadAssetBytesError(#[from] ReadAssetBytesError),
    #[error(transparent)]
    IidError(#[from] IidError),
    #[error(transparent)]
    LdtkWorldError(#[from] LdtkWorldAssetError),
    #[error(transparent)]
    LdtkLevelError(#[from] LdtkLevelAssetError),
    #[error(transparent)]
    LdtkLayerError(#[from] LdtkLayerAssetError),
    #[error(transparent)]
    LdtkEntityError(#[from] LdtkEntityAssetError),
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
    #[error(transparent)]
    LayerDefinitionFromError(#[from] LayerDefinitionFromError),
    #[error("field is None in non-multi world project! field: {0}")]
    FieldIsNone(String),
    #[error("failed to get project directory!")]
    ProjectDirFail,
}

#[derive(Default)]
pub(crate) struct LdtkProjectLoader;

impl AssetLoader for LdtkProjectLoader {
    type Asset = LdtkProject;
    type Settings = LdtkProjectSettings;
    type Error = LdtkProjectLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let project_directory = load_context
                .path()
                .parent()
                .ok_or(LdtkProjectLoaderError::ProjectDirFail)?
                .to_path_buf();

            let value: ldtk::LdtkJson = {
                let mut bytes: Vec<u8> = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                serde_json::from_slice(&bytes)?
            };

            let project_iid = Iid::from_str(&value.iid)?;

            let world_values = if value.worlds.is_empty() {
                vec![ldtk::World {
                    default_level_height: value.default_level_height.ok_or(
                        LdtkProjectLoaderError::FieldIsNone("default_level_height".to_string()),
                    )?,
                    default_level_width: value.default_level_width.ok_or(
                        LdtkProjectLoaderError::FieldIsNone("default_level_width".to_string()),
                    )?,
                    identifier: "World".to_string(),
                    iid: value.iid.clone(),
                    levels: value.levels,
                    world_grid_width: value.world_grid_width.ok_or(
                        LdtkProjectLoaderError::FieldIsNone("world_grid_width".to_string()),
                    )?,
                    world_grid_height: value.world_grid_height.ok_or(
                        LdtkProjectLoaderError::FieldIsNone("world_grid_height".to_string()),
                    )?,
                    world_layout: value.world_layout.clone(),
                }]
            } else {
                value.worlds
            };

            let level_values: Vec<ldtk::Level> = world_values
                .iter()
                .flat_map(|world_value| world_value.levels.iter())
                .map(|level_value| {
                    if value.external_levels {
                        let external_rel_path = level_value.external_rel_path.as_ref().ok_or(
                            LdtkProjectLoaderError::FieldIsNone("external_rel_path".to_string()),
                        )?;
                        let ldtk_path = Path::new(external_rel_path);
                        let bevy_path = ldtk_path_to_bevy_path(&project_directory, ldtk_path);
                        let bytes =
                            block_on(async { load_context.read_asset_bytes(bevy_path).await })?;
                        let level: ldtk::Level = serde_json::from_slice(&bytes)?;
                        Ok(level)
                    } else {
                        Ok(level_value.clone())
                    }
                })
                .collect::<Result<_, LdtkProjectLoaderError>>()?;

            let layer_values: Vec<ldtk::LayerInstance> = level_values
                .iter()
                .filter_map(|level_value| level_value.layer_instances.as_ref())
                .flat_map(|layer_instances| layer_instances.iter())
                .cloned()
                .collect();

            let entity_values: Vec<ldtk::EntityInstance> = layer_values
                .iter()
                .flat_map(|layer_instance| layer_instance.entity_instances.iter())
                .cloned()
                .collect();

            let worlds = world_values
                .iter()
                .map(|value| {
                    let label = value.iid.clone();
                    let iid = Iid::from_str(&value.iid)?;
                    let asset = LdtkWorldAsset::new(value)?;
                    trace!("world sub asset: {asset:?}");
                    let handle = load_context.add_labeled_asset(label, asset);
                    Ok((iid, handle))
                })
                .collect::<Result<_, LdtkProjectLoaderError>>()?;

            let levels = level_values
                .iter()
                .map(|value| {
                    let label = value.iid.clone();
                    let iid = Iid::from_str(&value.iid)?;
                    let asset = LdtkLevelAsset::new(
                        value,
                        settings.level_separation,
                        load_context,
                        &project_directory,
                    )?;
                    trace!("level sub asset: {asset:?}");
                    let handle = load_context.add_labeled_asset(label, asset);
                    Ok((iid, handle))
                })
                .collect::<Result<_, LdtkProjectLoaderError>>()?;

            let layers = layer_values
                .iter()
                .rev()
                .enumerate()
                .map(|(index, value)| {
                    let label = value.iid.clone();
                    let iid = Iid::from_str(&value.iid)?;
                    let asset = LdtkLayerAsset::new(
                        value,
                        index,
                        settings.layer_separation,
                        load_context,
                        &project_directory,
                    )?;
                    trace!("layer sub asset: {asset:?}");
                    let handle = load_context.add_labeled_asset(label, asset);
                    Ok((iid, handle))
                })
                .collect::<Result<_, LdtkProjectLoaderError>>()?;

            let entities = entity_values
                .iter()
                .map(|value| {
                    let label = value.iid.clone();
                    let iid = Iid::from_str(&value.iid)?;
                    let asset = LdtkEntityAsset::new(value, project_iid)?;
                    trace!("entity sub asset: {asset:?}");
                    let handle = load_context.add_labeled_asset(label, asset);
                    Ok((iid, handle))
                })
                .collect::<Result<_, LdtkProjectLoaderError>>()?;

            let layer_defs: HashMap<i64, LayerDefinition> = value
                .defs
                .layers
                .iter()
                .map(|layer_def| {
                    let layer_def = LayerDefinition::new(layer_def)?;
                    Ok((layer_def.uid, layer_def))
                })
                .collect::<Result<_, LdtkProjectLoaderError>>()?;

            let tileset_defs: HashMap<i64, TilesetDefinition> = value
                .defs
                .tilesets
                .iter()
                .map(TilesetDefinition::new)
                .map(|tile| (tile.uid, tile))
                .collect();

            let tilesets = tileset_defs
                .values()
                .filter_map(|tile_def| tile_def.rel_path.as_ref())
                .map(|rel_path| {
                    (
                        rel_path.clone(),
                        load_context.load(ldtk_path_to_bevy_path(
                            &project_directory,
                            Path::new(&rel_path),
                        )),
                    )
                })
                .collect();

            Ok(LdtkProject {
                iid: project_iid,
                settings: settings.clone(),
                worlds,
                levels,
                layer_defs,
                layers,
                entities,
                tileset_defs,
                tilesets,
                bg_color: bevy_color_from_ldtk(&value.bg_color)?,
                json_version: value.json_version.clone(),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
