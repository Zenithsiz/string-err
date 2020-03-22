//! Getters for `StringError`

// Backtrace
use std::backtrace::Backtrace;

// Crate
use crate::{StringError, MaybeBacktrace};

// Getters for any `StringError<E, B>`
impl<E: ?Sized, B: MaybeBacktrace> StringError<E, B>
{
	/// Returns the current message within this error
	#[must_use]
	pub fn msg(&self) -> &str {
		&self.msg
	}
	
	/// Returns the current underlying error within error
	#[must_use]
	pub fn err(&self) -> &E {
		&self.err
	}
	
	/// Returns the backtrace, if any
	#[must_use]
	pub fn backtrace_maybe(&self) -> Option<&Backtrace> {
		self.bt.as_backtrace()
	}
}

// Backtrace getters for any `StringError<E, Backtrace>`
impl<E: ?Sized> StringError<E, Backtrace>
{
	/// Returns the existing backtrace
	#[must_use]
	pub const fn backtrace(&self) -> &Backtrace {
		&self.bt
	}
}
