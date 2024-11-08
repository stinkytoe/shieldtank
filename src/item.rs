use bevy_ldtk_asset::iid::Iid;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;

use crate::query::LdtkQuery;

pub struct LdtkItem<'a, A, D>
where
    A: LdtkAsset,
    D: 'a,
{
    pub(crate) asset: &'a A,
    pub(crate) data: D,
    pub(crate) _query: &'a LdtkQuery<'a, 'a>,
}

impl<'a, A, D> std::fmt::Debug for LdtkItem<'a, A, D>
where
    A: LdtkAsset + std::fmt::Debug,
    D: 'a,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LdtkItem")
            .field("asset", &self.asset)
            //.field("data", &self.data)
            //.field("_query", &self._query)
            .finish()
    }
}

impl<'a, A, D> LdtkItem<'a, A, D>
where
    A: LdtkAsset,
    D: 'a,
{
    pub fn get_asset(&self) -> &A {
        self.asset
    }

    pub fn get_data(&self) -> &D {
        &self.data
    }
}

pub trait LdtkItemIteratorExt<'a, A, D>
where
    Self: Iterator<Item = LdtkItem<'a, A, D>> + Sized,
    A: LdtkAsset,
    D: 'a,
{
    fn filter_identifier(self, identifier: &'a str) -> LdtkItemFilterIdentifier<'a, A, D, Self> {
        LdtkItemFilterIdentifier {
            iter: self,
            identifier,
        }
    }

    fn find_iid(mut self, iid: Iid) -> Option<LdtkItem<'a, A, D>> {
        self.find(|item| item.asset.iid() == iid)
    }
}

pub struct LdtkItemFilterIdentifier<'a, A, D, I>
where
    A: LdtkAsset,
    D: 'a,
    I: Iterator<Item = LdtkItem<'a, A, D>>,
{
    iter: I,
    identifier: &'a str,
}

impl<'a, A, D, I> std::fmt::Debug for LdtkItemFilterIdentifier<'a, A, D, I>
where
    A: LdtkAsset,
    D: 'a,
    I: Iterator<Item = LdtkItem<'a, A, D>>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LdtkItemFilterIdentifier")
            //.field("iter", &self.iter)
            //.field("identifier", &self.identifier)
            .finish()
    }
}

impl<'a, A, D, I> Iterator for LdtkItemFilterIdentifier<'a, A, D, I>
where
    A: LdtkAsset,
    D: 'a,
    I: Iterator<Item = LdtkItem<'a, A, D>>,
{
    type Item = LdtkItem<'a, A, D>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find(|item| item.asset.identifier() == self.identifier)
    }
}

impl<'a, A, D, I> LdtkItemIteratorExt<'a, A, D> for I
where
    A: LdtkAsset,
    D: 'a,
    I: Iterator<Item = LdtkItem<'a, A, D>>,
{
}
