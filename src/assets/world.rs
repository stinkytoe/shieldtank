use std::str::FromStr;

use bevy::prelude::*;
use bevy::utils::HashSet;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::{Iid, IidError, IidSet};
use crate::ldtk;

#[derive(Debug, Error)]
pub enum LdtkWorldError {
    #[error(transparent)]
    IidError(#[from] IidError),
}

#[derive(Asset, Debug, Reflect)]
pub struct LdtkWorld {
    iid: Iid,
    children: IidSet,
}

impl LdtkWorld {
    pub(crate) fn new(value: &ldtk::World) -> Result<Self, LdtkWorldError> {
        let iid = Iid::from_str(&value.iid)?;

        let children = value
            .levels
            .iter()
            .map(|child| Iid::from_str(&child.iid))
            .collect::<Result<_, _>>()?;

        Ok(Self { iid, children })
    }
}

impl LdtkAsset for LdtkWorld {
    fn iid(&self) -> crate::iid::Iid {
        self.iid
    }

    fn children(&self) -> &IidSet {
        &self.children
    }
}
