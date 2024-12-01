use bevy_ecs::system::Commands;
use bevy_ldtk_asset::ldtk_asset_trait::LdtkAsset;
use bevy_reflect::Reflect;

use crate::item::LdtkItem;

#[derive(Reflect)]
pub struct LdtkItemCommands<'a, Asset>
where
    Asset: LdtkAsset + std::fmt::Debug,
{
    pub(crate) commands: Commands<'a, 'a>,
    pub(crate) item: &'a LdtkItem<'a, Asset>,
}

impl<'a, Asset> LdtkItemCommands<'a, Asset>
where
    Asset: LdtkAsset + std::fmt::Debug,
{
    //pub fn set_translation(&mut self, translation: Vec3) {
    //    let ecs_entity = self.item.get_ecs_entity();
    //
    //    let old_transform = if let Some(old_transform) = self.item.get_transform() {
    //        old_transform.with_translation(translation)
    //    } else {
    //        Transform::from_translation(translation)
    //    };
    //
    //    self.commands
    //        .entity(ecs_entity)
    //        .insert(old_transform.with_translation(translation));
    //}

    //pub fn set_visibility(&mut self, visibility: Visibility) {
    //    let ecs_entity = self.item.get_ecs_entity();
    //    self.commands.entity(ecs_entity).insert(visibility);
    //}
}
