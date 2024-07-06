use bevy::asset::AssetLoader;
use bevy::prelude::*;
use thiserror::Error;

use crate::assets::project::LdtkProject;
use crate::assets::project::LdtkProjectError;
use crate::assets::project::LdtkProjectSettings;

#[derive(Debug, Error)]
pub(crate) enum LdtkProjectLoaderError {
    #[error(transparent)]
    LdtkProjectError(#[from] LdtkProjectError),
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
        Box::pin(async move { todo!() })
    }

    fn extensions(&self) -> &[&str] {
        &[]
    }
}
