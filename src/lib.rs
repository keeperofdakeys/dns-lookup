extern crate libc;

mod addr;
mod addrinfo;
mod err;
mod lookup;

pub use lookup::{lookup_host, lookup_addr};
