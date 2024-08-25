use bevy::ecs::query::QueryFilter;

use crate::assets::entity::LdtkEntityAsset;
use crate::query::traits::LdtkAssetQuery;
use crate::query::traits::LdtkItem;
use crate::query::traits::LdtkItemIterator;
use crate::query::traits::LdtkItemIteratorWithIdentifier;
use crate::query::traits::LdtkItemIteratorWithIdentifierError;
use crate::reexports::field_instance::FieldInstance;

pub type LdtkEntity<'a> = LdtkItem<'a, LdtkEntityAsset>;

impl<'a> LdtkEntity<'a> {
    pub fn field_instance(&self, identifier: &str) -> Option<&FieldInstance> {
        self.asset
            .field_instances
            .iter()
            .find(|field_instance| field_instance.identifier == identifier)
    }
}

pub type LdtkEntitiesQuery<'w, 's> = LdtkAssetQuery<'w, 's, LdtkEntityAsset>;

impl<'w, 's> LdtkEntitiesQuery<'w, 's> {
    pub fn get_single_with_identifier(
        &'w self,
        identifier: &'static str,
    ) -> Result<LdtkItem<'w, LdtkEntityAsset>, LdtkItemIteratorWithIdentifierError> {
        let mut iter = self.iter().with_identifier(identifier);

        let first = iter.next();
        let rest = iter.next();

        match (first, rest) {
            (None, None) => Err(LdtkItemIteratorWithIdentifierError::None),
            (None, Some(_)) => unreachable!(),
            (Some(item), None) => Ok(item),
            (Some(_), Some(_)) => Err(LdtkItemIteratorWithIdentifierError::MoreThanOne),
        }
    }

    pub fn single_with_identifier(
        &'w self,
        identifier: &'static str,
    ) -> LdtkItem<'w, LdtkEntityAsset> {
        self.get_single_with_identifier(identifier)
            .expect("an entity")
    }
}

impl<'w, 's, F> LdtkItemIterator<'w, 's, LdtkEntityAsset, F>
where
    F: QueryFilter,
    Self: Iterator<Item = LdtkItem<'w, LdtkEntityAsset>>,
{
    pub fn with_identifier(
        self,
        identifier: &'static str,
    ) -> LdtkItemIteratorWithIdentifier<'w, 's, LdtkEntityAsset, F> {
        LdtkItemIteratorWithIdentifier {
            identifier,
            iter: self,
        }
    }
}
