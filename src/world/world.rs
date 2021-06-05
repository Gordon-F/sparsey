use crate::components::{Component, ComponentStorage, Entity, Ticks};
use crate::layout::Layout;
use crate::world::{ComponentSet, ComponentStorages, EntityStorage, UsedGroupFamilies};
use std::any::TypeId;
use std::error::Error;
use std::fmt;

/// Container for component storages and entities.
#[derive(Default)]
pub struct World {
	entities: EntityStorage,
	components: ComponentStorages,
	tick: Ticks,
}

impl World {
	pub fn with_layout(layout: &Layout) -> Self {
		let mut world = Self::default();
		world.set_layout(layout);
		world
	}

	pub fn set_layout(&mut self, layout: &Layout) {
		self.components.set_layout(layout, self.entities.as_ref());
	}

	pub fn register<T>(&mut self)
	where
		T: Component,
	{
		self.components.register::<T>()
	}

	pub unsafe fn register_storage(&mut self, component: TypeId, storage: ComponentStorage) {
		self.components.register_storage(component, storage);
	}

	/// Create an `Entity` with the given components and return it.
	pub fn create<C>(&mut self, components: C) -> Entity
	where
		C: ComponentSet,
	{
		let entity = self.entities.create();
		let _ = self.insert(entity, components);
		entity
	}

	/// Extend the `World` with a component iterator.
	/// Return the newly created entities as a slice.
	pub fn extend<C, I>(&mut self, components_iter: I) -> &[Entity]
	where
		C: ComponentSet,
		I: IntoIterator<Item = C>,
	{
		let initial_entity_count = self.entities.as_ref().len();

		{
			let mut storages = unsafe { C::borrow_storages(&self.components) };
			let entities = &mut self.entities;
			let tick = self.tick;

			components_iter.into_iter().for_each(|components| {
				let entity = entities.create();

				unsafe {
					C::insert(&mut storages, entity, components, tick);
				}
			});
		}

		let used_group_families = self.used_group_families(C::type_ids().as_ref());
		let new_entities = &self.entities.as_ref()[initial_entity_count..];

		for i in used_group_families.indexes() {
			for &entity in new_entities {
				self.components.grouped.group_components(i, entity);
			}
		}

		new_entities
	}

	/// Destroy an `Entity` and all of its components.
	/// Return whether or not there was an `Entity` to destroy.
	pub fn destroy(&mut self, entity: Entity) -> bool {
		if !self.entities.destroy(entity) {
			return false;
		}

		for i in 0..self.components.grouped.group_family_count() {
			self.components.grouped.ungroup_components(i, entity);
		}

		for storage in self.components.iter_storages_mut() {
			storage.remove_and_drop(entity);
		}

		true
	}

	/// Insert a set of `Components` to the given `Entity`, if it exists.
	pub fn insert<C>(&mut self, entity: Entity, components: C) -> Result<(), NoSuchEntity>
	where
		C: ComponentSet,
	{
		if !self.contains(entity) {
			return Err(NoSuchEntity);
		}

		unsafe {
			let mut storages = C::borrow_storages(&self.components);
			C::insert(&mut storages, entity, components, self.tick);
		}

		let used_group_families = self.used_group_families(C::type_ids().as_ref());

		for i in used_group_families.indexes() {
			self.components.grouped.group_components(i, entity);
		}

		Ok(())
	}

	/// Remove a set of `Components` from an `Entity` and return them if they
	/// were all present before calling this function.
	pub fn remove<C>(&mut self, entity: Entity) -> Option<C>
	where
		C: ComponentSet,
	{
		if !self.contains(entity) {
			return None;
		}

		let used_group_families = self.used_group_families(C::type_ids().as_ref());

		for i in used_group_families.indexes() {
			self.components.grouped.ungroup_components(i, entity);
		}

		unsafe {
			let mut storages = C::borrow_storages(&self.components);
			C::remove(&mut storages, entity)
		}
	}

	/// Delete a set of components from an `Entity`.
	pub fn delete<C>(&mut self, entity: Entity)
	where
		C: ComponentSet,
	{
		if !self.contains(entity) {
			return;
		}

		let used_group_families = self.used_group_families(C::type_ids().as_ref());

		for i in used_group_families.indexes() {
			self.components.grouped.ungroup_components(i, entity);
		}

		unsafe {
			let mut storages = C::borrow_storages(&self.components);
			C::delete(&mut storages, entity);
		}
	}

	pub fn contains(&self, entity: Entity) -> bool {
		self.entities.contains(entity)
	}

	pub fn clear(&mut self) {
		self.entities.clear();
		self.components.clear();
	}

	pub(crate) fn entity_storage(&self) -> &EntityStorage {
		&self.entities
	}

	pub(crate) fn component_storages(&self) -> &ComponentStorages {
		&self.components
	}

	pub(crate) fn maintain(&mut self) {
		self.entities.maintain();
	}

	pub fn advance_ticks(&mut self) {
		self.tick += 1;
	}

	pub(crate) fn tick(&self) -> Ticks {
		self.tick
	}

	fn used_group_families(&self, type_ids: &[TypeId]) -> UsedGroupFamilies {
		let mut used_group_families = UsedGroupFamilies::new();

		for type_id in type_ids {
			if let Some(index) = self.components.grouped.group_family_index(type_id) {
				unsafe {
					used_group_families.add_unchecked(index);
				}
			}
		}

		used_group_families
	}
}

/// Error returned when trying to access entities
/// which are not contained in the `World`.
#[derive(Debug)]
pub struct NoSuchEntity;

impl Error for NoSuchEntity {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}
}

impl fmt::Display for NoSuchEntity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "No such entity was found in the World")
	}
}
