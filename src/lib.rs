//! String error type
//! 
//! This crate provides a string error type, [`StringError`]
//! that may have an underlying error and backtrace too.
//! This is useful for binaries, that need only to display the top-level
//! error to the user and not describe it in code, using an `enum`.
//! 
//! It may also be used as the return type in main, provided the underlying
//! error type is specified as `Box<dyn Error + 'static>`
//! 
//! This crate currently uses various `nightly` features to implement the string
//! error type. Even when the used features are stabilized, this crate will likely
//! keep using other nightly features, as the crate is intended for usage with only
//! binaries. The binaries themselves can use `nightly` easily, without affecting
//! the libraries built along-side them.

// Features
#![feature(specialization, backtrace, debug_non_exhaustive, external_doc)]

// Lints
#![warn(
	clippy::all,
	clippy::restriction,
	clippy::pedantic,
	clippy::nursery,
	clippy::cargo,
)]
#![allow(
	clippy::implicit_return,                // We prefer to use implicit returns where possible
	clippy::items_after_statements,         // Sometimes we only add things from where they are first used, even if they are available in the whole scope
	clippy::shadow_reuse,                   // We shadow values when accepting generics `file: impl AsRef<U>` and `let file = file.as_ref()`, for example
	clippy::multiple_inherent_impl,         // We separate `impl`s where possible based on their purpose
	clippy::let_underscore_must_use,        // We'll only use `let _ = ...` if we really want to ignore the result
	clippy::module_name_repetitions,        // Sometimes we'll have modules such as `Error` and `XYZError` inside of it.
	clippy::wildcard_imports,               // We only use wildcards for example, when doing `super::*`, when separating implementations into modules.
	clippy::missing_inline_in_public_items, // Debated if this does anything
	clippy::needless_doctest_main,          // We often need it when declaring 2 functions.
)]

// Modules
pub mod maybe_backtrace;

// Exports
pub use maybe_backtrace::MaybeBacktrace;

/// The string error type
/// 
/// See the crate documentation for examples on how to use this type.
#[derive(PartialEq, Eq, Hash, Default)]
pub struct StringError<E: ?Sized, B: MaybeBacktrace = ()> {
	/// The error message
	msg: String,
	
	/// A possible backtrace
	bt: B,
	
	/// The unlderying error type
	err: E,
}

// Constructors
mod constructor;

// Getters
mod getter;

// `Debug` impl
mod debug;

// `Display` impl
mod display;

// `Error` impl
mod error;

// `README.md` tests
#[doc(include = "../README.md")]
type _ReadmeDocTests = ();
