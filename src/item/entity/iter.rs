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
