use bevy_ecs::world::Ref;
use bevy_ldtk_asset::entity::Entity as EntityAsset;

use crate::tileset_rectangle::TilesetRectangle;

use super::ShieldtankComponent;

pub type EntityComponent = ShieldtankComponent<EntityAsset>;

pub type EntityComponentQueryData<'a> = Option<Ref<'a, TilesetRectangle>>;
