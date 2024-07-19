use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use thiserror::Error;

use crate::assets::entity::LdtkEntity;
use crate::assets::project::LdtkProject;
use crate::iid::Iid;

#[derive(SystemParam)]
pub struct LdtkEntityCommands<'w, 's> {
    _commands: Commands<'w, 's>,
    entity_assets: Res<'w, Assets<LdtkEntity>>,
    project_assets: Res<'w, Assets<LdtkProject>>,
    entity_query: Query<'w, 's, (Entity, &'static Handle<LdtkEntity>)>,
}

impl<'w> LdtkEntityCommands<'w, '_> {
    pub fn get(&self, iid: Iid) -> Option<&LdtkEntity> {
        self.project_assets
            .iter()
            .map(|(_, project_asset)| project_asset)
            .flat_map(|project_asset| project_asset.entities.values())
            .filter_map(|handle| self.entity_assets.get(handle.id()))
            .find(|entity_asset| entity_asset.iid == iid)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &LdtkEntity)> {
        self.entity_query
            .iter()
            // TODO: Is this safe/sane to .filter_map(..) here?
            .filter_map(|(entity, handle)| Some((entity, self.entity_assets.get(handle.id())?)))
    }
}

#[derive(Debug, Error)]
pub enum LdtkEntityWithIdentifierExError {
    #[error("Itentifier yielded no values: {0}")]
    NoValues(String),
    #[error("Identifier yielded more than one value: {0}")]
    MoreThanOneValue(String),
}

pub trait LdtkEntityWithIdentifierEx<'w>:
    Iterator<Item = (Entity, &'w LdtkEntity)> + Sized
{
    fn with_identifier(self, identifier: &str) -> impl Iterator<Item = (Entity, &'w LdtkEntity)> {
        self.filter(move |(_, asset)| asset.identifier == identifier)
    }

    fn single_with_identifier(
        self,
        identifier: &str,
    ) -> Result<(Entity, &'w LdtkEntity), LdtkEntityWithIdentifierExError> {
        let mut iter = self.with_identifier(identifier);
        let first = iter.next();
        let rest = iter.next();

        match (first, rest) {
            (None, None) => Err(LdtkEntityWithIdentifierExError::NoValues(
                identifier.to_string(),
            )),
            (None, Some(_)) => unreachable!(),
            (Some(x), None) => Ok(x),
            (Some(_), Some(_)) => Err(LdtkEntityWithIdentifierExError::MoreThanOneValue(
                identifier.to_string(),
            )),
        }
    }
}

// magic!
impl<'w, I: Iterator<Item = (Entity, &'w LdtkEntity)>> LdtkEntityWithIdentifierEx<'w> for I {}
