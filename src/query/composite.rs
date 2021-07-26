use crate::query::{
	passthrough, IntoQueryParts, PassthroughFilter, QueryBase, QueryFilter, QueryModifier,
};

/// Query with an include modifier.
pub struct Include<B, I> {
	base: B,
	include: I,
}

impl<'b, B, I> Include<B, I> {
	pub(crate) fn new(base: B, include: I) -> Self {
		Self { base, include }
	}

	/// Applies an exclude modifier to the query.
	pub fn exclude<'a, E>(self, exclude: E) -> IncludeExclude<B, I, E>
	where
		E: QueryModifier<'a>,
	{
		IncludeExclude::new(self.base, self.include, exclude)
	}

	/// Applies a filter to the query.
	pub fn filter<'a, F>(self, filter: F) -> IncludeExcludeFilter<B, I, (), F>
	where
		F: QueryModifier<'a>,
	{
		IncludeExcludeFilter::new(self.base, self.include, (), filter)
	}
}

impl<'a, B, I> IntoQueryParts<'a> for Include<B, I>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
{
	type Base = B;
	type Include = I;
	type Exclude = ();
	type Filter = PassthroughFilter;

	fn into_parts(self) -> (Self::Base, Self::Include, Self::Exclude, Self::Filter) {
		(self.base, self.include, (), passthrough())
	}
}

/// Query with an include modifier and an exclude modifier.
pub struct IncludeExclude<B, I, E> {
	base: B,
	include: I,
	exclude: E,
}

impl<B, I, E> IncludeExclude<B, I, E> {
	pub(crate) fn new(base: B, include: I, exclude: E) -> Self {
		Self {
			base,
			include,
			exclude,
		}
	}

	/// Applies a filter to the query.
	pub fn filter<F>(self, filter: F) -> IncludeExcludeFilter<B, I, E, F>
	where
		F: QueryFilter,
	{
		IncludeExcludeFilter::new(self.base, self.include, self.exclude, filter)
	}
}

impl<'a, B, I, E> IntoQueryParts<'a> for IncludeExclude<B, I, E>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
{
	type Base = B;
	type Include = I;
	type Exclude = E;
	type Filter = PassthroughFilter;

	fn into_parts(self) -> (Self::Base, Self::Include, Self::Exclude, Self::Filter) {
		(self.base, self.include, self.exclude, passthrough())
	}
}

/// Query with an include modifier, an exclude modifier and a filter.
pub struct IncludeExcludeFilter<B, I, E, F> {
	base: B,
	include: I,
	exclude: E,
	filter: F,
}

impl<B, I, E, F> IncludeExcludeFilter<B, I, E, F> {
	pub(crate) fn new(base: B, include: I, exclude: E, filter: F) -> Self {
		Self {
			base,
			include,
			exclude,
			filter,
		}
	}
}

impl<'a, B, I, E, F> IntoQueryParts<'a> for IncludeExcludeFilter<B, I, E, F>
where
	B: QueryBase<'a>,
	I: QueryModifier<'a>,
	E: QueryModifier<'a>,
	F: QueryFilter,
{
	type Base = B;
	type Include = I;
	type Exclude = E;
	type Filter = F;

	fn into_parts(self) -> (Self::Base, Self::Include, Self::Exclude, Self::Filter) {
		(self.base, self.include, self.exclude, self.filter)
	}
}
