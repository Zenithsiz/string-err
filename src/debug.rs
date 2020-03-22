//! [`Debug`] implementations for [`StringError`]

// Fmt
use std::fmt::{Debug, Formatter, Result as FmtResult};

// Crate
use crate::{StringError, MaybeBacktrace};

/// Default `Debug` impl for all `StringError<E, B>`
impl<E: ?Sized, B: MaybeBacktrace> Debug for StringError<E, B>
{
	default fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
		fmt.debug_struct("StringError")
			.field("msg", &self.msg)
			.field("bt" , &self.bt )
			.finish_non_exhaustive()
	}
}

/// Specialiation `Debug` impl for `E: Sized + Debug`
impl<E: Sized + Debug, B: MaybeBacktrace> Debug for StringError<E, B>
{
	// Note: Not `default` because if `E: Sized + Debug`, then we can
	//       always debug it, so there is no need for specialization.
	fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
		fmt.debug_struct("StringError")
			.field("msg", &self.msg)
			.field("err", &self.err)
			.field("bt" , &self.bt )
			.finish_non_exhaustive()
	}
}
