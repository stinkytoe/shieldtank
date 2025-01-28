use bevy_ecs::world::Ref;
use bevy_ldtk_asset::level::Level as LevelAsset;

use crate::component::ShieldtankComponent;
use crate::level_background::LevelBackground;

pub type LevelComponent = ShieldtankComponent<LevelAsset>;

pub type LevelComponentQueryData<'a> = Option<Ref<'a, LevelBackground>>;
