use bevy::prelude::*;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::Iid;

#[derive(Debug, Error)]
pub enum LdtkLevelError {}

#[derive(Asset, Debug, Reflect)]
pub struct LdtkLevel {
    iid: Iid,
    project: Entity,
}

impl LdtkAsset for LdtkLevel {
    fn iid(&self) -> crate::iid::Iid {
        self.iid
    }

    fn project(&self) -> Entity {
        self.project
    }
}
