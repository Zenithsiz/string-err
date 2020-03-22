//! A trait representing a type that may be either `()` or [`Backtrace`].
//! 
//! See the trait [`MaybeBacktrace`]

// Backtrace
use std::backtrace::Backtrace;


/// Implemented for `()` and `Backtrace` only.
/// 
/// This trait is sealed, meaning it may not be implemented
/// by any other types.
pub trait MaybeBacktrace: std::fmt::Debug + seal::Sealed {
	/// Returns a possible backtrace
	fn as_backtrace(&self) -> Option<&Backtrace>;
}

impl MaybeBacktrace for () {
	#[inline]
	fn as_backtrace(&self) -> Option<&Backtrace> { None }
}

impl MaybeBacktrace for Backtrace {
	#[inline]
	fn as_backtrace(&self) -> Option<&Backtrace> { Some( self ) }
}

/// Seal module
mod seal {
	// Super
	use super::*;
	
	/// Seal for `MaybeBacktrace`
	pub trait Sealed {}
	
	impl Sealed for ()        {}
	impl Sealed for Backtrace {}
}
