use std::str::FromStr;

use bevy::prelude::*;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::{Iid, IidError, IidSet};
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
}

impl LdtkLayer {
    pub(crate) fn new(value: &ldtk::LayerInstance) -> Result<Self, LdtkLayerError> {
        let iid = Iid::from_str(&value.iid)?;

        let children = value
            .entity_instances
            .iter()
            .map(|child| Iid::from_str(&child.iid))
            .collect::<Result<_, _>>()?;

        Ok(Self { iid, children })
    }
}

impl LdtkAsset for LdtkLayer {
    fn iid(&self) -> crate::iid::Iid {
        self.iid
    }

    fn children(&self) -> &IidSet {
        &self.children
    }
}
