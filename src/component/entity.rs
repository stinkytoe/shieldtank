use bevy_ecs::world::Ref;
use bevy_ldtk_asset::entity::Entity as EntityAsset;
use bevy_sprite::Sprite;

use crate::component::ShieldtankComponent;
use crate::tileset_rectangle::TilesetRectangle;

pub type EntityComponent = ShieldtankComponent<EntityAsset>;

pub type EntityComponentQueryData<'a> =
    (Option<Ref<'a, TilesetRectangle>>, Option<Ref<'a, Sprite>>);
