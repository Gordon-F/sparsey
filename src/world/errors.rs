use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Error returned when trying to access entities
/// which are not contained in the `World`.
#[derive(Debug)]
pub struct NoSuchEntity;

impl Error for NoSuchEntity {}

impl Display for NoSuchEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "No such entity was found in the World")
    }
}

/// Error returned when the `World` tick cannot be incremented.
#[derive(Debug)]
pub struct CannotIncrementWorldTick;

impl Error for CannotIncrementWorldTick {}

impl Display for CannotIncrementWorldTick {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cannot increment the world tick without overflowing the counter"
        )
    }
}
