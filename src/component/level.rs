use bevy_ecs::world::Ref;
use bevy_ldtk_asset::level::Level as LevelAsset;
use bevy_sprite::Sprite;

use crate::tiles::Tiles;

use super::ShieldtankComponent;

pub type LevelComponent = ShieldtankComponent<LevelAsset>;

pub type LevelComponentQueryData<'a> = (Option<Ref<'a, Tiles>>, Option<Ref<'a, Sprite>>);
