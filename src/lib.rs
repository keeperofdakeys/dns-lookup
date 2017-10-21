//! # `dns_lookup`
//! A small wrapper for libc to perform simple DNS lookups.
//!
//! Two main functions are provided.
//!
//! # `lookup_host`
//! Given a hostname, return an Iterator the IP Addresses associated with
//! it.
//!
//! ```rust
//!   use dns_lookup::lookup_host;
//!
//!   let hostname = "localhost";
//!   let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
//!   assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
//! ```
//!
//! # `lookup_addr`
//! Given an IP Address, return the reverse DNS entry (hostname) for the
//! given IP Address.
//!
//!
//! ```rust
//!   use dns_lookup::lookup_addr;
//!
//!   let ip: std::net::IpAddr = "127.0.0.1".parse().unwrap();
//!   let hostname = lookup_addr(&ip).unwrap();
//!   assert_eq!(hostname, "localhost");
//! ```
//!
//! # `getaddrinfo`
//! ```rust
//!   extern crate dns_lookup;
//!
//!   #[cfg(unix)]
//!   extern crate libc;
//!
//!   #[cfg(windows)]
//!   extern crate winapi;
//!
//!   use dns_lookup::{getaddrinfo, AddrInfoHints};
//!
//!   #[cfg(unix)]
//!   use libc::SOCK_STREAM;
//!
//!   #[cfg(windows)]
//!   use winapi::SOCK_STREAM;
//!
//!   fn main() {
//!     let hostname = "localhost";
//!     let service = "ssh";
//!     let hints = AddrInfoHints {
//!       socktype: SOCK_STREAM,
//!       .. AddrInfoHints::default()
//!     };
//!     let sockets =
//!       getaddrinfo(Some(hostname), Some(service), Some(hints))
//!         .unwrap().collect::<std::io::Result<Vec<_>>>().unwrap();
//!
//!     for socket in sockets {
//!       // Try connecting to socket
//!       let _ = socket;
//!     }
//!   }
//! ```
//!
//! # `getnameinfo`
//! ```rust
//!   use dns_lookup::getnameinfo;
//!   use std::net::{IpAddr, SocketAddr};
//!
//!   let ip: IpAddr = "127.0.0.1".parse().unwrap();
//!   let port = 22;
//!   let socket: SocketAddr = (ip, port).into();
//!
//!   let (name, service) = match getnameinfo(&socket, 0) {
//!     Ok((n, s)) => (n, s),
//!     Err(e) => panic!("Failed to lookup socket {:?}", e),
//!   };
//!
//!   println!("{:?} {:?}", name, service);
//!   let _ = (name, service);
//! ```

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[cfg(unix)] extern crate libc;
#[cfg(unix)] extern crate cfg_if;

#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate ws2_32;

extern crate socket2;

mod addrinfo;
mod nameinfo;
mod err;
mod lookup;

pub use lookup::{lookup_host, lookup_addr};
pub use addrinfo::{getaddrinfo, AddrInfoIter, AddrInfo, AddrInfoHints};
pub use nameinfo::getnameinfo;
