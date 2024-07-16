use bevy::asset::AssetLoader;
use bevy::asset::AsyncReadExt;
use bevy::asset::ReadAssetBytesError;
use bevy::prelude::*;
use bevy::tasks::block_on;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

use crate::assets::entity::LdtkEntity;
use crate::assets::entity::LdtkEntityError;
use crate::assets::layer::LdtkLayer;
use crate::assets::layer::LdtkLayerError;
use crate::assets::level::LdtkLevel;
use crate::assets::level::LdtkLevelError;
use crate::assets::project::LdtkProject;
use crate::assets::project::LdtkProjectError;
use crate::assets::project::LdtkProjectSettings;
use crate::assets::world::LdtkWorld;
use crate::assets::world::LdtkWorldError;
use crate::iid::Iid;
use crate::iid::IidError;
use crate::ldtk;
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
    LdtkWorldError(#[from] LdtkWorldError),
    #[error(transparent)]
    LdtkLevelError(#[from] LdtkLevelError),
    #[error(transparent)]
    LdtkLayerError(#[from] LdtkLayerError),
    #[error(transparent)]
    LdtkEntityError(#[from] LdtkEntityError),
    #[error(transparent)]
    ColorParseError(#[from] ColorParseError),
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
                    let asset = LdtkWorld::new(value)?;
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
                    let asset = LdtkLevel::new(
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
                    let asset = LdtkLayer::new(
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
                    let asset = LdtkEntity::new(value)?;
                    trace!("entity sub asset: {asset:?}");
                    let handle = load_context.add_labeled_asset(label, asset);
                    Ok((iid, handle))
                })
                .collect::<Result<_, LdtkProjectLoaderError>>()?;

            Ok(LdtkProject {
                settings: settings.clone(),
                worlds,
                levels,
                layers,
                entities,
                bg_color: bevy_color_from_ldtk(&value.bg_color)?,
                json_version: value.json_version.clone(),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ldtk"]
    }
}
