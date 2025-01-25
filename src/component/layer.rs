use bevy_ecs::world::Ref;
use bevy_ldtk_asset::layer::Layer as LayerAsset;

use crate::int_grid::IntGrid;
use crate::tiles::Tiles;

use super::ShieldtankComponent;

pub type LayerComponent = ShieldtankComponent<LayerAsset>;

pub type LayerComponentQueryData<'a> = (Option<Ref<'a, IntGrid>>, Option<Ref<'a, Tiles>>);
