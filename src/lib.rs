extern crate libc;

mod lookup;
mod addr;
mod addrinfo;

pub use lookup::{lookup_host, lookup_addr};
pub use lookup::Error as LookupError;
