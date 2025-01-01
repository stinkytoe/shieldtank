use bevy_ldtk_asset::prelude::LdtkAsset;
use bevy_math::Vec2;
use bevy_reflect::Reflect;

use crate::item::LdtkItemTrait;
use crate::layer::LayerItem;
use crate::level::LevelItem;
use crate::project::ProjectItem;
use crate::query::LdtkQuery;
use crate::world::WorldItem;

pub struct LayerLocation<'a> {
    location: Vec2,
    item: LayerItem<'a>,
}

impl<'a> LayerLocation<'a> {
    pub fn new(location: Vec2, item: LayerItem<'a>) -> Self {
        Self { location, item }
    }
}

// #[derive(Reflect)]
// pub enum Location<'a> {
//     Global {
//         location: Vec2,
//         query: LdtkQuery<'a, 'a>,
//     },
//
//     Project {
//         location: Vec2,
//         query: LdtkQuery<'a, 'a>,
//         project: &'a ProjectItem<'a>,
//     },
//
//     World {
//         location: Vec2,
//         query: LdtkQuery<'a, 'a>,
//         world: &'a WorldItem<'a>,
//     },
//
//     Level {
//         location: Vec2,
//         query: LdtkQuery<'a, 'a>,
//         level: &'a LevelItem<'a>,
//     },
//
//     Layer {
//         location: Vec2,
//         query: LdtkQuery<'a, 'a>,
//         layer: &'a LayerItem<'a>,
//     },
// }
