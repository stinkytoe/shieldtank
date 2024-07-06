use bevy::prelude::*;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::iid::Iid;

#[derive(Debug, Error)]
pub enum LdtkEntityError {}

#[derive(Asset, Debug, Reflect)]
pub struct LdtkEntity {
    iid: Iid,
    project: Entity,
}

impl LdtkAsset for LdtkEntity {
    fn iid(&self) -> crate::iid::Iid {
        self.iid
    }

    fn project(&self) -> Entity {
        self.project
    }
}
