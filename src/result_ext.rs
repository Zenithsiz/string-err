//! Extentions to [`Result`]-like types that implement `Try`
//! for working with [`StringError`]

// Std
use std::ops::Try;

// Crate
use crate::StringError;


/// Extention trait for [`Result`]-like types
/// 
/// See the module documentation for examples
pub trait ResultExt
where
	Self: Sized + Try
{
	/// Maps an error into `StringError` with a message
	/// 
	/// # Errors
	/// This will return `Err( StringError { .. } )` if the original
	/// result was `Err`, else it will return `Ok`
	fn map_err_msg<R: From<Self::Error>>(self, msg: impl Into<String>) -> Result<Self::Ok, StringError<R>> {
		self.into_result()
			.map_err(StringError::with_err(msg))
	}
}

// Default impl
impl<T: Sized + Try> ResultExt for T {}
