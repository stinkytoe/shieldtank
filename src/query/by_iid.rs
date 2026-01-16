use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

use bevy_ecs::change_detection::Tick;
use bevy_ecs::query::ReadOnlyQueryData;
use bevy_ecs::query::{FilteredAccessSet, QueryState};
use bevy_ecs::query::{QueryData, QueryFilter};
use bevy_ecs::system::ReadOnlySystemParam;
use bevy_ecs::system::{Query, SystemParam};
use bevy_ecs::system::{SystemMeta, SystemParamValidationError};
use bevy_ecs::world::World;
use bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell;

use crate::bevy_ldtk_asset::iid::Iid;
use crate::component::iid::IidRegistry;

#[derive(Debug)]
pub struct SingleByIid<'w, 's, const IID: u128, D: QueryData, F: QueryFilter = ()> {
    item: D::Item<'w, 's>,
    _phantom: PhantomData<F>,
}

impl<'w, 's, const IID: u128, D: QueryData, F: QueryFilter> Deref
    for SingleByIid<'w, 's, IID, D, F>
{
    type Target = D::Item<'w, 's>;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<'w, 's, const IID: u128, D: QueryData, F: QueryFilter> DerefMut
    for SingleByIid<'w, 's, IID, D, F>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl<'w, 's, const IID: u128, D: QueryData, F: QueryFilter> SingleByIid<'w, 's, IID, D, F> {
    pub fn into_inner(self) -> D::Item<'w, 's> {
        self.item
    }
}

unsafe impl<'w, 's, const IID: u128, D: QueryData + 'static, F: QueryFilter + 'static> SystemParam
    for SingleByIid<'w, 's, IID, D, F>
{
    type State = QueryState<D, F>;

    type Item<'world, 'state> = SingleByIid<'world, 'state, IID, D, F>;

    fn init_state(world: &mut World) -> Self::State {
        Query::init_state(world)
    }

    fn init_access(
        state: &Self::State,
        system_meta: &mut SystemMeta,
        component_access_set: &mut FilteredAccessSet,
        world: &mut World,
    ) {
        Query::init_access(state, system_meta, component_access_set, world);
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &SystemMeta,
        world: UnsafeWorldCell<'world>,
        change_tick: Tick,
    ) -> Self::Item<'world, 'state> {
        let iid = Iid::from_u128(IID);

        // SAFETY: We're relying on the plugin being loaded by this point.
        let registry = unsafe {
            world
                .get_resource::<IidRegistry>()
                .expect("Couldn't get IidRegistry! Is the plugin loaded?")
        };

        let entity = *registry
            .registry
            .get(&iid)
            .expect("Entity not found! This should have been handled in `validate_param`!");

        // SAFETY: These shennanigans are dependent on `validate_param` only
        // passing on valid queries.
        let query = unsafe { Query::get_param(state, system_meta, world, change_tick) };

        let item = query
            .get_inner(entity)
            .expect("Entity not in query! This should have been handled in `validate_param`!");

        SingleByIid {
            item,
            _phantom: PhantomData,
        }
    }

    unsafe fn validate_param(
        state: &mut Self::State,
        _system_meta: &SystemMeta,
        world: UnsafeWorldCell,
    ) -> Result<(), SystemParamValidationError> {
        let iid = Iid::from_u128(IID);

        // SAFETY: We're relying on the plugin being loaded by this point.
        let registry = unsafe { world.get_resource::<IidRegistry>() }.ok_or(
            SystemParamValidationError::invalid::<Self>(
                "Failed to get IidRegistry resource. Is the plugin loaded?",
            ),
        )?;

        let query = unsafe {
            //state.query_unchecked_with_ticks(world, system_meta.get_last_run(), world.change_tick())
            state.query_unchecked_with_ticks(world, world.last_change_tick(), world.change_tick())
        };

        let entity = *registry
            .registry
            .get(&iid)
            .ok_or(SystemParamValidationError::skipped::<Self>(
                "Iid not found. Skipping...",
            ))?;

        query
            .contains(entity)
            .then_some(())
            .ok_or(SystemParamValidationError::skipped::<Self>(
                "Iid found, but does not match query. Skipping...",
            ))
    }
}

// Shamelessy stolen from Bevy's impl for Single.
// link: https://github.com/bevyengine/bevy/blob/c1d31bba96b7751e7cff3cf68a1adacb5f9c36ad/crates/bevy_ecs/src/system/system_param.rs#L460C1-L464C2
//
// SAFETY: QueryState is constrained to read-only fetches, so it only reads World.
unsafe impl<'a, 'b, const IID: u128, D: ReadOnlyQueryData + 'static, F: QueryFilter + 'static>
    ReadOnlySystemParam for SingleByIid<'a, 'b, IID, D, F>
{
}
