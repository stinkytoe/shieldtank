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
