use std::str::FromStr;

use bevy::prelude::*;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::{Iid, IidError, IidSet};
use crate::ldtk;

#[derive(Debug, Error)]
pub enum LdtkLevelError {
    #[error(transparent)]
    IidError(#[from] IidError),
}

#[derive(Asset, Debug, Reflect)]
pub struct LdtkLevel {
    iid: Iid,
    children: IidSet,
}

impl LdtkLevel {
    pub(crate) fn new(value: &ldtk::Level) -> Result<Self, LdtkLevelError> {
        let iid = Iid::from_str(&value.iid)?;

        let children = value
            .layer_instances
            .iter()
            .flatten()
            .map(|child| Iid::from_str(&child.iid))
            .collect::<Result<_, _>>()?;

        Ok(Self { iid, children })
    }
}

impl LdtkAsset for LdtkLevel {
    fn iid(&self) -> crate::iid::Iid {
        self.iid
    }

    fn children(&self) -> &IidSet {
        &self.children
    }
}
