use bevy_math::Vec2;

use crate::item::entity::EntityItem;

pub struct HasTagIterator<'w, 's, I>
where
    's: 'w,
    I: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
    iter: I,
    tag: &'w str,
}

impl<'w, 's, I> Iterator for HasTagIterator<'w, 's, I>
where
    I: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
    type Item = EntityItem<'w, 's>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|item| item.has_tag(self.tag))
    }
}

pub trait HasTagIteratorExt<'w, 's>
where
    's: 'w,
    Self: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
    fn filter_tag(self, tag: &'w str) -> HasTagIterator<'w, 's, Self> {
        HasTagIterator { iter: self, tag }
    }
}

impl<'w, 's, I> HasTagIteratorExt<'w, 's> for I
where
    's: 'w,
    I: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
}

pub struct LocationInRegionIterator<'w, 's, I>
where
    's: 'w,
    I: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
    iter: I,
    location: Vec2,
}

impl<'w, 's, I> Iterator for LocationInRegionIterator<'w, 's, I>
where
    I: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
    type Item = EntityItem<'w, 's>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find(|item| item.location_in_region(self.location))
    }
}

pub trait LocationInRegionIteratorExt<'w, 's>
where
    's: 'w,
    Self: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
    fn filter_location_in_region(self, location: Vec2) -> LocationInRegionIterator<'w, 's, Self> {
        LocationInRegionIterator {
            iter: self,
            location,
        }
    }
}

impl<'w, 's, I> LocationInRegionIteratorExt<'w, 's> for I
where
    's: 'w,
    I: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
}

pub struct WorldLocationInRegionIterator<'w, 's, I>
where
    's: 'w,
    I: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
    iter: I,
    location: Vec2,
}

impl<'w, 's, I> Iterator for WorldLocationInRegionIterator<'w, 's, I>
where
    I: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
    type Item = EntityItem<'w, 's>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .find(|item| item.world_location_in_region(self.location))
    }
}

pub trait WorldLocationInRegionIteratorExt<'w, 's>
where
    's: 'w,
    Self: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
    fn filter_world_location_in_region(
        self,
        location: Vec2,
    ) -> WorldLocationInRegionIterator<'w, 's, Self> {
        WorldLocationInRegionIterator {
            iter: self,
            location,
        }
    }
}

impl<'w, 's, I> WorldLocationInRegionIteratorExt<'w, 's> for I
where
    's: 'w,
    I: Iterator<Item = EntityItem<'w, 's>> + Sized,
{
}
