use downcast_rs::{impl_downcast, Downcast};

/// Trait automatically implemented for all types that can be stored in
/// `Resources`.
pub trait Resource
where
    Self: Downcast,
{
    // Empty
}

impl_downcast!(Resource);

impl<T> Resource for T
where
    T: Downcast,
{
    // Empty
}
