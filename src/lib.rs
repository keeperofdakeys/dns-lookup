extern crate libc;

mod lookup;
mod addr;

pub use lookup::{lookup_host, lookup_addr};
