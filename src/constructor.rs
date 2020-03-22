//! Constructors for [`StringError`]

// Backtrace
use std::backtrace::Backtrace;

// Crate
use crate::StringError;

// Message only
impl StringError<(), ()>
{
	/// Creates a new string error with no underlying error nor backtrace
	/// 
	/// This is not recommended to be used unless you really don't have an underlying
	/// error, and even so, it might be more useful to use [`StringError::with_backtrace`],
	/// as at least you would have some reference.
	pub fn new(msg: impl Into<String>) -> Self {
		Self {
			msg: msg.into(),
			bt : (),
			err: (),
		}
	}
}

// Message + Backtrace
impl StringError<(), Backtrace>
{
	/// Creates a new string error with no underlying error, but with a backtrace
	/// 
	/// This is reccomended only when you don't have an underlying error.
	pub fn with_backtrace(msg: impl Into<String>) -> Self {
		let msg = msg.into();
		let bt = Backtrace::force_capture();
		Self { msg, bt, err: () }
	}
}

// Message + Error
impl<E: Sized> StringError<E, ()>
{
	/// Returns a function to be passed to `map_err`, `map_panic_err` or alike
	/// that returns a string error with the underlying error but no backtrace
	pub fn with_err<R: Into<E>>(msg: impl Into<String>) -> impl FnOnce(R) -> Self {
		let msg = msg.into();
		move |err| Self { msg, bt: (), err: err.into() }
	}
}

// Message + Error + Backtrace
impl<E: Sized> StringError<E, Backtrace>
{
	/// Returns a function to be passed to `map_err`, `map_panic_err` or alike
	/// that returns a string error with the underlying error and a backtrace
	pub fn with_err_backtrace<R: Into<E>>(msg: impl Into<String>) -> impl FnOnce(R) -> Self {
		let msg = msg.into();
		let bt = Backtrace::force_capture();
		move |err| Self { msg, bt, err: err.into() }
	}
}
