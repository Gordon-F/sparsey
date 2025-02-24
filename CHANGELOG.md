# 0.5.0 (2021-11-09)
## Added
- Implemented `fold` for `EntityIter`, greatly improving the performance of `for_each`.
- Implemented `fmt::Debug` for `ComponentView` and `ResourceView`.
- Added `World::delete_resource` to remove a `Resource` with a given `TypeId`.

## Changed
- Cleaned up `QueryElement` to make the code faster and easier to maintain.
- Replaced `ComponentView::storage` with methods on `ComponentView`.
- `World::increment_tick` no longer sets the world tick to zero on overflow.

## Fixed
- `IntoEntityIterator` is now only implemented for `EntityIterator`s, not all `Iterator`s. 

# 0.4.0 (2021-10-17)
## Added
- `World::borrow` now accepts `Option<Res<T>>` and `Option<ResMut<T>>`.
- Systems can now have `Option<Res<T>>` and `Option<ResMut<T>>` as parameters.
- Added `ComponentView::storage` to get a reference to the view's `ComponentStorage`.
- Added `#[must_use]` to functions whose returns should not be discarded.  

## Changed
- Reworked `ComponentStorage` to make adding, removing and swapping components faster.
- Optimize the implementation of `ComponentSet` for the unit type.
- All methods of `ComponentSet` are now safe.
- Reduce size of `System`, `LocalSystem` and `GroupInfo` structs.
- Items not meant to be used outside `Sparsey` are now `pub(crate)`.

## Removed
- Removed `World::resources` and `World::storages` iterators.

## Fixed
- Removed debug `println` from `ComponentStorages`.

# 0.3.1 (2021-10-04)
## Changed
- Inlined some functions to improve iteration performance.

# 0.3.0 (2021-09-28)
## Added
- Added `World::resources` for iterating over all `Resource`s and their `TypeId`s.
- Added `World::storages` for iterating over all `ComponentStorage`s and the `TypeId`s of the components they hold.

## Changed
- Refactor `BlobVec`, improving the performance of adding, removing and updating components.
- Improved performance of `World::create_entities` when the components belong to groups.
- Simplify `QueryModifier` to improve the performance of creating iterators.
- Changed visibility of `TypedComponentStorage` to public.
- Improved performance of grouping and ungrouping components.

## Removed
- Removed all methods from `ComponentView`.

## Fixed
- Removing a component from a nested group no longer ungroups the components of the parent groups.

# 0.2.0 (2021-09-19)
## Added
- Queries over a single component view no longer need to be wrapped in a tuple.
- Added `World::destroy_entities` for destroying entities in bulk.
- Added `World::set_tick` for setting the tick used in change detection.

## Changed
- Iterators over a single component view are now dense, greatly improving performance.
- Refactor `ComponentSet`, improving the performance of adding and removing components.

## Fixed
- Fixed `!added`, `!mutated`, `!changed` not being usable in queries.

# 0.1.0 (2021-09-12)
- First version.
