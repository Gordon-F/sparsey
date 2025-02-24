use crate::components::{Component, GroupInfo};
use crate::query::{
    ComponentRefMut, ImmutableUnfilteredQueryElement, QueryElementFilter, UnfilteredQueryElement,
};
use crate::storage::{ComponentStorage, Entity, EntitySparseArray, IndexEntity};
use crate::utils::{ChangeTicks, Ticks};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// View over a `ComponentStorage` of type `T`.
pub struct ComponentView<'a, T, S> {
    storage: S,
    group_info: Option<GroupInfo<'a>>,
    world_tick: Ticks,
    change_tick: Ticks,
    _phantom: PhantomData<*const T>,
}

impl<'a, T, S> ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    pub(crate) unsafe fn new(
        storage: S,
        group_info: Option<GroupInfo<'a>>,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Self {
        Self {
            storage,
            group_info,
            world_tick,
            change_tick,
            _phantom: PhantomData,
        }
    }

    /// Returns the `ChangeTicks` of `entity`'s component.
    #[inline]
    pub fn get_ticks(&self, entity: Entity) -> Option<&ChangeTicks> {
        self.storage.get_ticks(entity)
    }

    /// Returns the component and `ChangeTicks` of `entity`.
    #[inline]
    pub fn get_with_ticks(&self, entity: Entity) -> Option<(&T, &ChangeTicks)> {
        unsafe { self.storage.get_with_ticks::<T>(entity) }
    }

    /// Returns the number of components in the view.
    #[inline]
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Returns `true` if the view is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    /// Returns all entities in the view as a slice.
    #[inline]
    pub fn entities(&self) -> &[Entity] {
        self.storage.entities()
    }

    /// Returns all components in the view as a slice.
    #[inline]
    pub fn components(&self) -> &[T] {
        unsafe { self.storage.components::<T>() }
    }

    /// Returns all `ChangeTicks` in the view as a slice.
    #[inline]
    pub fn ticks(&self) -> &[ChangeTicks] {
        self.storage.ticks()
    }
}

impl<'a, T, S> fmt::Debug for ComponentView<'a, T, S>
where
    T: Component + fmt::Debug,
    S: Deref<Target = ComponentStorage>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries = unsafe { self.storage.iter::<T>() };
        f.debug_list().entries(entries).finish()
    }
}

unsafe impl<'a, T, S> UnfilteredQueryElement<'a> for &'a ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    type Item = &'a T;
    type Component = T;

    #[inline]
    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.group_info
    }

    #[inline]
    fn world_tick(&self) -> Ticks {
        self.world_tick
    }

    #[inline]
    fn change_tick(&self) -> Ticks {
        self.change_tick
    }

    #[inline]
    fn contains<F>(&self, entity: Entity, filter: &F) -> bool
    where
        F: QueryElementFilter<Self::Component>,
    {
        if F::IS_PASSTHROUGH {
            return self.storage.contains(entity);
        }

        let (component, ticks) = unsafe {
            match self.storage.get_with_ticks::<T>(entity) {
                Some(data) => data,
                None => return false,
            }
        };

        F::matches(filter, component, ticks, self.world_tick, self.change_tick)
    }

    #[inline]
    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity> {
        self.storage.get_index_entity(entity)
    }

    #[inline]
    unsafe fn get_unchecked<F>(self, index: usize, filter: &F) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>,
    {
        if F::IS_PASSTHROUGH {
            return Some(self.storage.get_unchecked::<T>(index));
        }

        let (component, ticks) = self.storage.get_with_ticks_unchecked(index);

        if filter.matches(component, ticks, self.world_tick, self.change_tick) {
            Some(component)
        } else {
            None
        }
    }

    #[inline]
    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        NonNull<Self::Component>,
        NonNull<ChangeTicks>,
    ) {
        self.storage.split()
    }

    #[inline]
    unsafe fn get_from_parts_unchecked<F>(
        components: NonNull<Self::Component>,
        ticks: NonNull<ChangeTicks>,
        index: usize,
        filter: &F,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>,
    {
        if F::IS_PASSTHROUGH {
            return Some(&*components.cast::<T>().as_ptr().add(index));
        }

        let component = &*components.cast::<T>().as_ptr().add(index);
        let ticks = &*ticks.as_ptr().add(index);

        if F::matches(filter, component, ticks, world_tick, change_tick) {
            Some(component)
        } else {
            None
        }
    }
}

unsafe impl<'a, T, S> ImmutableUnfilteredQueryElement<'a> for &'a ComponentView<'a, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage>,
{
    #[inline]
    fn entities(&self) -> &'a [Entity] {
        self.storage.entities()
    }

    #[inline]
    fn components(&self) -> &'a [Self::Component] {
        unsafe { self.storage.components::<T>() }
    }
}

unsafe impl<'a, 'b, T, S> UnfilteredQueryElement<'a> for &'a mut ComponentView<'b, T, S>
where
    T: Component,
    S: Deref<Target = ComponentStorage> + DerefMut,
{
    type Item = ComponentRefMut<'a, T>;
    type Component = T;

    #[inline]
    fn group_info(&self) -> Option<GroupInfo<'a>> {
        self.group_info
    }

    #[inline]
    fn world_tick(&self) -> Ticks {
        self.world_tick
    }

    #[inline]
    fn change_tick(&self) -> Ticks {
        self.change_tick
    }

    #[inline]
    fn contains<F>(&self, entity: Entity, filter: &F) -> bool
    where
        F: QueryElementFilter<Self::Component>,
    {
        if F::IS_PASSTHROUGH {
            return self.storage.contains(entity);
        }

        let (component, ticks) = unsafe {
            match self.storage.get_with_ticks(entity) {
                Some(data) => data,
                None => return false,
            }
        };

        filter.matches(component, ticks, self.world_tick, self.change_tick)
    }

    #[inline]
    fn get_index_entity(&self, entity: Entity) -> Option<&IndexEntity> {
        self.storage.get_index_entity(entity)
    }

    #[inline]
    unsafe fn get_unchecked<F>(self, index: usize, filter: &F) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>,
    {
        let (component, ticks) = self.storage.get_with_ticks_unchecked_mut(index);

        if F::IS_PASSTHROUGH {
            return Some(ComponentRefMut::new(component, ticks, self.world_tick));
        }

        if filter.matches(component, ticks, self.world_tick, self.change_tick) {
            Some(ComponentRefMut::new(component, ticks, self.world_tick))
        } else {
            None
        }
    }

    #[inline]
    fn split(
        self,
    ) -> (
        &'a [Entity],
        &'a EntitySparseArray,
        NonNull<Self::Component>,
        NonNull<ChangeTicks>,
    ) {
        self.storage.split()
    }

    #[inline]
    unsafe fn get_from_parts_unchecked<F>(
        components: NonNull<Self::Component>,
        ticks: NonNull<ChangeTicks>,
        index: usize,
        filter: &F,
        world_tick: Ticks,
        change_tick: Ticks,
    ) -> Option<Self::Item>
    where
        F: QueryElementFilter<Self::Component>,
    {
        let component = &mut *components.cast::<T>().as_ptr().add(index);
        let ticks = &mut *ticks.as_ptr().add(index);

        if F::IS_PASSTHROUGH {
            return Some(ComponentRefMut::new(component, ticks, world_tick));
        }

        if filter.matches(component, ticks, world_tick, change_tick) {
            Some(ComponentRefMut::new(component, ticks, world_tick))
        } else {
            None
        }
    }
}
