use bevy::prelude::*;
use thiserror::Error;

use crate::ldtk;

#[derive(Debug, Error)]
pub enum NeighbourError {
    #[error("Given unknown neighbour string from LDtk project! {0}")]
    BadDir(String),
}

#[derive(Clone, Debug, Reflect)]
pub enum NeighbourDir {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Lower,
    Greater,
    Overlap,
}

#[derive(Clone, Debug, Reflect)]
pub struct Neighbour {
    pub level_iid: String,
    pub dir: NeighbourDir,
}

impl Neighbour {
    pub(crate) fn new(value: &ldtk::NeighbourLevel) -> Result<Self, NeighbourError> {
        Ok(Self {
            level_iid: value.level_iid.clone(),
            dir: match value.dir.as_str() {
                "n" => NeighbourDir::North,
                "s" => NeighbourDir::South,
                "w" => NeighbourDir::West,
                "e" => NeighbourDir::East,
                "<" => NeighbourDir::Lower,
                ">" => NeighbourDir::Greater,
                "o" => NeighbourDir::Overlap,
                "nw" => NeighbourDir::NorthWest,
                "ne" => NeighbourDir::NorthEast,
                "sw" => NeighbourDir::SouthWest,
                "se" => NeighbourDir::SouthEast,
                _ => return Err(NeighbourError::BadDir(value.dir.clone())),
            },
        })
    }
}
