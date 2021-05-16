use crate::components::{Component, Entity, SparseArrayView, Ticks};
use crate::dispatcher::{Comp, CompMut};
use crate::query::IterData;
use crate::world::{CombinedGroupInfo, GroupInfo};

pub trait ComponentFilterElement {
	fn includes(&self, entity: Entity) -> bool;
}

impl<'a, T> ComponentFilterElement for &'a Comp<'a, T>
where
	T: Component,
{
	fn includes(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}
}

impl<'a, T> ComponentFilterElement for &'a CompMut<'a, T>
where
	T: Component,
{
	fn includes(&self, entity: Entity) -> bool {
		self.storage.contains(entity)
	}
}

impl<'a> ComponentFilterElement for SparseArrayView<'a> {
	fn includes(&self, entity: Entity) -> bool {
		self.contains(entity)
	}
}

pub trait BaseComponentFilterElement<'a>
where
	Self: ComponentFilterElement,
{
	type Split: ComponentFilterElement;

	fn world_tick(&self) -> Ticks;

	fn last_system_tick(&self) -> Ticks;

	fn group_info(&self) -> GroupInfo;

	fn split(self) -> (&'a [Entity], Self::Split);
}

impl<'a, T> BaseComponentFilterElement<'a> for &'a Comp<'a, T>
where
	T: Component,
{
	type Split = SparseArrayView<'a>;

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn group_info(&self) -> GroupInfo {
		self.group_info
	}

	fn split(self) -> (&'a [Entity], Self::Split) {
		let (sparse, entities, _, _) = self.storage.split();
		(entities, sparse)
	}
}

impl<'a, T> BaseComponentFilterElement<'a> for &'a CompMut<'a, T>
where
	T: Component,
{
	type Split = SparseArrayView<'a>;

	fn world_tick(&self) -> Ticks {
		self.world_tick
	}

	fn last_system_tick(&self) -> Ticks {
		self.last_system_tick
	}

	fn group_info(&self) -> GroupInfo {
		self.group_info
	}

	fn split(self) -> (&'a [Entity], Self::Split) {
		let (sparse, entities, _, _) = self.storage.split();
		(entities, sparse)
	}
}

pub trait ComponentFilter {
	fn includes_all(&self, entity: Entity) -> bool;

	fn excludes_all(&self, entity: Entity) -> bool;
}

pub trait BaseComponentFilter<'a>
where
	Self: ComponentFilter,
{
	type Split: ComponentFilter;

	fn group_info(&self) -> CombinedGroupInfo;

	fn split_sparse(self) -> (Option<IterData<'a>>, Self::Split);

	fn split_dense(self) -> (Option<IterData<'a>>, Self::Split);
}

impl ComponentFilter for () {
	fn includes_all(&self, _entity: Entity) -> bool {
		true
	}

	fn excludes_all(&self, _entity: Entity) -> bool {
		true
	}
}

impl<'a> BaseComponentFilter<'a> for () {
	type Split = ();

	fn group_info(&self) -> CombinedGroupInfo {
		CombinedGroupInfo::Empty
	}

	fn split_sparse(self) -> (Option<IterData<'a>>, Self::Split) {
		(None, ())
	}

	fn split_dense(self) -> (Option<IterData<'a>>, Self::Split) {
		(None, ())
	}
}

macro_rules! impl_filter {
	($(($elem:ident, $idx:tt)),+) => {
		impl<$($elem),+> ComponentFilter for ($($elem,)+)
		where
			$($elem: ComponentFilterElement,)+
		{
			fn includes_all(&self, entity: Entity) -> bool {
				true && $(self.$idx.includes(entity))&&+
			}

			fn excludes_all(&self, entity: Entity) -> bool {
				true && $(!self.$idx.includes(entity))&&+
			}
		}

		impl<'a, $($elem),+> BaseComponentFilter<'a> for ($($elem,)+)
		where
			$($elem: BaseComponentFilterElement<'a>,)+
		{
			type Split = ($($elem::Split,)+);

			fn group_info(&self) -> CombinedGroupInfo {
				CombinedGroupInfo::new() $(.combine(self.$idx.group_info()))+
			}

			fn split_sparse(self) -> (Option<IterData<'a>>, Self::Split) {
				split_sparse!(split, $(($elem, self.$idx)),+)
			}

			fn split_dense(self) -> (Option<IterData<'a>>, Self::Split) {
				split_dense!(split, $(($elem, self.$idx)),+)
			}
		}
	};
}

impl_filter!((A, 0));
impl_filter!((A, 0), (B, 1));
impl_filter!((A, 0), (B, 1), (C, 2));
impl_filter!((A, 0), (B, 1), (C, 2), (D, 3));
