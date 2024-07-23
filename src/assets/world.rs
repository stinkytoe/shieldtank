use bevy::math::I64Vec2;
use bevy::prelude::*;
use ldtk::WorldLayout;
use std::str::FromStr;
use thiserror::Error;

use crate::assets::project::LdtkProject;
use crate::assets::traits::LdtkAsset;
use crate::iid::Iid;
use crate::iid::IidError;
use crate::iid::IidSet;
use crate::ldtk;

#[derive(Debug, Error)]
pub enum LdtkWorldAssetError {
    #[error(transparent)]
    IidError(#[from] IidError),
    #[error("missing worldLayout? {0}")]
    MissingWorldLayout(Iid),
}

#[derive(Asset, Debug, Reflect)]
pub struct LdtkWorldAsset {
    // NOTE: Internal fields
    iid: Iid,
    identifier: String,
    children: IidSet,
    // NOTE: LDtk exports
    world_grid_size: I64Vec2, // world_grid_width, world_grid_height
    world_layout: WorldLayout,
}

impl LdtkWorldAsset {
    pub(crate) fn new(value: &ldtk::World) -> Result<Self, LdtkWorldAssetError> {
        let iid = Iid::from_str(&value.iid)?;

        let children = value
            .levels
            .iter()
            .map(|child| Iid::from_str(&child.iid))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            iid,
            identifier: value.identifier.clone(),
            children,
            world_grid_size: (value.world_grid_width, value.world_grid_height).into(),
            world_layout: value
                .world_layout
                .clone()
                .ok_or(LdtkWorldAssetError::MissingWorldLayout(iid))?,
        })
    }
}

impl LdtkAsset for LdtkWorldAsset {
    fn iid(&self) -> Iid {
        self.iid
    }

    fn children(&self) -> &IidSet {
        &self.children
    }

    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn asset_handle_from_project(project: &LdtkProject, iid: Iid) -> Option<Handle<Self>> {
        project.worlds.get(&iid).cloned()
    }
}
