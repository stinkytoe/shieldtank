use crate::item::layer::LayerItem;

pub trait LayerItemIteratorExt<'w, 's>
where
    Self: Iterator<Item = LayerItem<'w, 's>> + Sized,
    's: 'w,
{
    fn filter_tiles_layer(self) -> impl Iterator<Item = LayerItem<'w, 's>> {
        self.filter(|item| item.is_tiles_layer())
    }

    fn filter_entities_layer(self) -> impl Iterator<Item = LayerItem<'w, 's>> {
        self.filter(|item| item.is_entities_layer())
    }
}

impl<'w, 's, I: Iterator<Item = LayerItem<'w, 's>>> LayerItemIteratorExt<'w, 's> for I where 's: 'w {}
