//! [`Error`] implementations for [`StringError`]

// Std
use std::error::Error;
use std::backtrace::Backtrace;

// Crate
use crate::{StringError, MaybeBacktrace};

/// Default `Error` impl for any all `StringError<E, B>`
impl<E: ?Sized, B: MaybeBacktrace> Error for StringError<E, B>
{
	default fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}
	
	fn backtrace(&self) -> Option<&Backtrace> { self.backtrace_maybe() }
}

/// Specialization `Error` implementation for `Error + 'static` types
/// Note: `E` can't be `?Sized` because the pointer we'd return is a fat pointer
///       with the size and the vtable, which itself is `?Sized`
impl<E: Error + 'static, B: MaybeBacktrace> Error for StringError<E, B>
{
	/// Note: Not `default` because if we do have a `Error + 'static` underlying
	///       error, we can always return it.
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some( &self.err )
	}
	
	// Note: No `fn backtrace` impl because the global impl already covers it
	//       without specialization.
}
