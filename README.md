String error type.

This crate contains a type `StringError`, to be used to report top-level errors to a user,
such as in a binary when executing `main`.

# Examples

```rust
use std::{io::Read, error::Error};

use string_err::{StringError, ResultExt};

pub fn main() -> Result<(), StringError<Box<dyn Error>>> {
	let mut file = std::fs::File::open("README.md")
		.map_err_msg("Could not open the file")?;
	
	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.map_err_msg("Could not read the file")?;
	
	assert!( contents.starts_with("String error type.") );
	
	Ok(())
}
```

This example attempts to load the `README.md` file, read it and make sure it starts
with the string `"String error type."` (which this readme does).

See the [documentation](https://docs.rs/string-err/) for more details on how to use
the `StringError` type
