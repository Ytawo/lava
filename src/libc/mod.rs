mod memory;
mod bindings_libc;

pub use self::bindings_libc::*;
pub use self::memory::*;

#[allow(non_camel_case_types)]
pub type void = i8;