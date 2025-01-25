use bevy_ecs::world::Ref;
use bevy_ldtk_asset::level::Level as LevelAsset;

use crate::level_background::LevelBackground;

use super::ShieldtankComponent;

pub type LevelComponent = ShieldtankComponent<LevelAsset>;

pub type LevelComponentQueryData<'a> = Option<Ref<'a, LevelBackground>>;
