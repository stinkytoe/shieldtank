use bevy_ecs::world::Ref;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_sprite::Sprite;

use crate::tileset_rectangle::TilesetRectangle;

use super::ShieldtankComponent;

pub type EntityComponent = ShieldtankComponent<EntityAsset>;

pub type EntityComponentQueryData<'a> =
    (Option<Ref<'a, TilesetRectangle>>, Option<Ref<'a, Sprite>>);
