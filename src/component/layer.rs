use bevy_ecs::world::Ref;
use bevy_ldtk_asset::layer::Layer as LayerAsset;

use crate::component::ShieldtankComponent;
use crate::int_grid::IntGrid;
use crate::tiles::Tiles;

pub type LayerComponent = ShieldtankComponent<LayerAsset>;

pub type LayerComponentQueryData<'a> = (Option<Ref<'a, IntGrid>>, Option<Ref<'a, Tiles>>);
