use bevy_ecs::world::Ref;
use bevy_sprite::Sprite;

use crate::int_grid::IntGrid;
use crate::tiles::Tiles;

pub type LayerComponentQueryData<'a> = (
    Option<Ref<'a, IntGrid>>,
    Option<Ref<'a, Tiles>>,
    Option<Ref<'a, Sprite>>,
);
