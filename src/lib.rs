//! # dns_lookup
//! A small wrapper for libc to perform simple DNS lookups.
//!
//! Two main functions are provided.
//!
//! # `lookup_host`
//! Given a hostname, return an Iterator the IP Addresses associated with
//! it.
//!
//! ```rust
//!  use dns_lookup::{lookup_host, lookup_addr};
//!
//!  let hostname = "localhost";
//!  let ips: Vec<std::net::IpAddr> =
//!    lookup_host(hostname).unwrap().collect::<std::io::Result<_>>().unwrap();
//!  assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
//! ```
//!
//! # `lookup_addr`
//! Given an IP Address, return the reverse DNS entry (hostname) for the
//! given IP Address.
//!
//!
//! ```rust
//!  use dns_lookup::{lookup_host, lookup_addr};
//!
//!  let ip: std::net::IpAddr = "127.0.0.1".parse().unwrap();
//!  let hostname = lookup_addr(&ip).unwrap();
//!  assert_eq!(hostname, "localhost");
//! ```

extern crate libc;

mod addr;
// mod addrinfo;
mod err;
mod lookup;

pub use lookup::{lookup_host, lookup_addr};
