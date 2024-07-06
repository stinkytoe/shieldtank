use bevy::prelude::*;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::Iid;

#[derive(Debug, Error)]
pub enum LdtkWorldError {}

#[derive(Asset, Debug, Reflect)]
pub struct LdtkWorld {
    iid: Iid,
    project: Entity,
}

impl LdtkAsset for LdtkWorld {
    fn iid(&self) -> crate::iid::Iid {
        self.iid
    }

    fn project(&self) -> Entity {
        self.project
    }
}
