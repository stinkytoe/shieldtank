use std::str::FromStr;

use bevy::prelude::*;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::Iid;
use crate::iid::IidError;
use crate::iid::IidSet;
use crate::ldtk;

#[derive(Debug, Error)]
pub enum LdtkEntityError {
    #[error(transparent)]
    IidError(#[from] IidError),
}

#[derive(Asset, Debug, Reflect)]
pub struct LdtkEntity {
    iid: Iid,
    children: IidSet,
    identifier: String,
}

impl LdtkEntity {
    pub(crate) fn new(value: &ldtk::EntityInstance) -> Result<Self, LdtkEntityError> {
        let iid = Iid::from_str(&value.iid)?;

        let children = IidSet::default();

        Ok(Self {
            iid,
            children,
            identifier: value.identifier.clone(),
        })
    }
}

impl LdtkAsset for LdtkEntity {
    fn iid(&self) -> crate::iid::Iid {
        self.iid
    }

    fn children(&self) -> &IidSet {
        &self.children
    }

    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn asset_handle_from_project(
        project: &crate::prelude::LdtkProject,
        iid: Iid,
    ) -> Option<Handle<Self>> {
        project.entities.get(&iid).cloned()
    }
}
