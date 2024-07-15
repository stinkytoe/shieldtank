use bevy::prelude::*;
use std::str::FromStr;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::Iid;
use crate::iid::IidError;
use crate::iid::IidSet;
use crate::ldtk;

#[derive(Debug, Error)]
pub enum LdtkLayerError {
    #[error(transparent)]
    IidError(#[from] IidError),
}

#[derive(Asset, Debug, Reflect)]
pub struct LdtkLayer {
    iid: Iid,
    children: IidSet,
    identifier: String,
}

impl LdtkLayer {
    pub(crate) fn new(value: &ldtk::LayerInstance) -> Result<Self, LdtkLayerError> {
        let iid = Iid::from_str(&value.iid)?;

        let children = value
            .entity_instances
            .iter()
            .map(|child| Iid::from_str(&child.iid))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            iid,
            children,
            identifier: value.identifier.clone(),
        })
    }
}

impl LdtkAsset for LdtkLayer {
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
        project.layers.get(&iid).cloned()
    }
}
