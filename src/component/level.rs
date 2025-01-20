use bevy_ecs::world::Ref;
use bevy_sprite::Sprite;

use crate::tiles::Tiles;

pub type LevelComponentQueryData<'a> = (Option<Ref<'a, Tiles>>, Option<Ref<'a, Sprite>>);
