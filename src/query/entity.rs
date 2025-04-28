use bevy_ecs::query::{QueryData, QueryFilter};
use bevy_math::Vec2;
use bevy_transform::components::GlobalTransform;

use crate::component::entity::LdtkEntity;
use crate::component::global_bounds::LdtkGlobalBounds;
use crate::component::tags::LdtkTags;

use super::component::ShieldtankComponentQuery;

#[derive(QueryData)]
pub struct EntityComponentData {
    global_transform: &'static GlobalTransform,
    global_bounds: &'static LdtkGlobalBounds,
    tags: Option<&'static LdtkTags>,
}

#[allow(private_interfaces)]
pub type LdtkEntityQuery<'w, 's, D, F = ()> =
    ShieldtankComponentQuery<'w, 's, LdtkEntity, EntityComponentData, D, F>;

impl<D, F> LdtkEntityQuery<'_, '_, D, F>
where
    D: QueryData<ReadOnly = D>,
    F: QueryFilter,
{
    pub fn location_in_bounds(
        &self,
        location: Vec2,
    ) -> impl Iterator<Item = <D as QueryData>::Item<'_>> {
        self.query
            .iter()
            .filter(move |(_, data, _)| data.global_bounds.contains(location))
            .map(|(_, _, data)| data)
    }

    pub fn has_tag(&self, tag: &str) -> impl Iterator<Item = <D as QueryData>::Item<'_>> {
        self.query
            .iter()
            .filter(|(_, data, _)| {
                if let Some(tags) = data.tags.as_ref() {
                    tags.contains(tag)
                } else {
                    false
                }
            })
            .map(|(_, _, data)| data)
    }
}
