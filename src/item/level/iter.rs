use bevy_ecs::change_detection::DetectChanges as _;

use crate::item::level::LevelItem;

pub trait LevelItemIteratorExt<'w, 's>
where
    Self: Iterator<Item = LevelItem<'w, 's>> + Sized,
    's: 'w,
{
    fn filter_level_background_changed(self) -> impl Iterator<Item = LevelItem<'w, 's>> {
        self.filter(|item| {
            item.get_level_background()
                .as_ref()
                .and_then(|level_background| level_background.is_changed().then_some(()))
                .is_some()
        })
    }
}

impl<'w, 's, I: Iterator<Item = LevelItem<'w, 's>>> LevelItemIteratorExt<'w, 's> for I where 's: 'w {}
