use bevy::ecs::query::QueryIter;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::assets::entity::LdtkEntityAsset;
use crate::prelude::{FieldInstance, Iid, LdtkProject};

#[derive(SystemParam)]
pub struct LdtkQuery<'w, 's> {
    project_query: Query<'w, 's, &'static Handle<LdtkProject>>,
    entity_query: Query<'w, 's, (Entity, &'static Handle<LdtkEntityAsset>)>,
    project_assets: Res<'w, Assets<LdtkProject>>,
    entity_assets: Res<'w, Assets<LdtkEntityAsset>>,
    asset_server: Res<'w, AssetServer>,
}

impl LdtkQuery<'_, '_> {
    pub fn entities(&self) -> LdtkEntityIterator<'_> {
        LdtkEntityIterator {
            query: self,
            iter: self.entity_query.into_iter(),
        }
    }

    pub fn get_project(&self, iid: Iid) -> Option<&LdtkProject> {
        self.project_assets
            .iter()
            .map(|(_, project)| project)
            .find(|project| project.iid == iid)
    }

    pub fn all_projects_loaded(&self) -> bool {
        !self
            .project_query
            .iter()
            .any(|handle| !self.asset_server.is_loaded_with_dependencies(handle.id()))
    }
}

pub struct LdtkEntityIterator<'a> {
    query: &'a LdtkQuery<'a, 'a>,
    iter: QueryIter<'a, 'a, (Entity, &'static Handle<LdtkEntityAsset>), ()>,
}

impl LdtkEntityIterator<'_> {}

impl<'a> Iterator for LdtkEntityIterator<'a> {
    type Item = LdtkEntity<'a>;

    // TODO: Decide if this function should panic (or otherwise fail)
    // if asset isn't found in the assets resource.
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .and_then(|(entity, handle)| Some((entity, self.query.entity_assets.get(handle.id())?)))
            .map(|(entity, asset)| LdtkEntity {
                entity,
                asset,
                // query: self.query,
            })
    }
}

pub trait LdtkEntityIteratorExt<'a>
where
    Self: Iterator<Item = LdtkEntity<'a>> + Sized,
{
    fn get_single_with_identifier(&mut self, identifier: &str) -> Option<LdtkEntity<'a>> {
        let predicate = |item: &LdtkEntity| item.asset.identifier == identifier;

        let first = self.find(predicate);
        let rest = self.find(predicate);

        match (first, rest) {
            (None, None) => None,
            (None, Some(_)) => unreachable!(),
            (Some(item), None) => Some(item),
            (Some(_), Some(_)) => None,
        }
    }

    fn single_with_identifier(&mut self, identifier: &str) -> LdtkEntity<'a> {
        self.get_single_with_identifier(identifier)
            .expect("an LdtkEntity item reference")
    }
}

impl<'a, I: Iterator<Item = LdtkEntity<'a>>> LdtkEntityIteratorExt<'a> for I {}

pub struct LdtkEntity<'a> {
    entity: Entity,
    asset: &'a LdtkEntityAsset,
    // query: &'a LdtkQuery<'a, 'a>,
}

impl LdtkEntity<'_> {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn asset(&self) -> &LdtkEntityAsset {
        self.asset
    }
}

impl LdtkEntity<'_> {
    pub fn get_field_instance<'a>(&'a self, identifier: &str) -> Option<&'a FieldInstance> {
        self.asset
            .field_instances
            .iter()
            .find(|field_instance| field_instance.identifier == identifier)
    }
}
