extern crate libc;

mod lookup;
mod addr;
mod addrinfo;
mod err;

pub use lookup::{lookup_host, lookup_addr};
