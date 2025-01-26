use bevy_ecs::change_detection::DetectChanges as _;

use crate::item::entity::EntityItem;

pub trait EntityItemIteratorExt<'w, 's>
where
    Self: Iterator<Item = EntityItem<'w, 's>> + Sized,
    's: 'w,
{
    fn filter_tileset_rectangle_changed(self) -> impl Iterator<Item = EntityItem<'w, 's>> {
        self.filter(|item| {
            item.get_tileset_rectangle()
                .as_ref()
                .and_then(|tileset_rectangle| tileset_rectangle.is_changed().then_some(()))
                .is_some()
        })
    }
}

impl<'w, 's, I: Iterator<Item = EntityItem<'w, 's>>> EntityItemIteratorExt<'w, 's> for I where 's: 'w
{}

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
