//! [`Display`] implementations for [`StringError`]

// Fmt
use std::fmt::{Display, Formatter, Result as FmtResult};

// Crate
use crate::{StringError, MaybeBacktrace};

/// `Display` implementation for all `StringError<E, B>`
impl<E: ?Sized, B: MaybeBacktrace> Display for StringError<E, B>
{
	fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
		fmt.pad(&self.msg)
	}
}
