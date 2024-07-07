use std::str::FromStr;

use bevy::prelude::*;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::{Iid, IidError, IidSet};
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
}

impl LdtkEntity {
    pub(crate) fn new(value: &ldtk::EntityInstance) -> Result<Self, LdtkEntityError> {
        let iid = Iid::from_str(&value.iid)?;

        let children = IidSet::default();

        Ok(Self { iid, children })
    }
}

impl LdtkAsset for LdtkEntity {
    fn iid(&self) -> crate::iid::Iid {
        self.iid
    }

    fn children(&self) -> &IidSet {
        &self.children
    }
}
