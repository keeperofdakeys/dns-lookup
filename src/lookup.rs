use libc as c;
use std::ffi::{CStr};
use std::io;
use std::net::IpAddr;
use std::ptr;
use std::str;

use addr::ip_to_sockaddr;
use addrinfo::{getaddrinfo, AddrInfoHints};
use err::lookup_errno;
use types::*;

// fn init_windows_sockets() {
//   use std::sync;
//   static START: Once = sync::Once::new();
// 
//   START.call_once(|| unsafe {
//       let mut data: c::WSADATA = mem::zeroed();
//       let ret = c::WSAStartup(0x202, // version 2.2
//                               &mut data);
//       assert_eq!(ret, 0);
// 
//       let _ = sys_common::at_exit(|| { c::WSACleanup(); });
//     });
// }

/// Lookup the address for a given hostname via DNS.
///
/// Returns an iterator of IP Addresses, or an io::Error on failure.
pub fn lookup_host(host: &str) -> io::Result<Vec<IpAddr>> {
  // FIXME: Initialise windows sockets somehow :/
  // #[cfg(windows)]
  // init_windows_sockets();

  let hints = AddrInfoHints {
    socktype: SockType::Stream,
    ..AddrInfoHints::default()
  };

  match getaddrinfo(Some(host), None, Some(hints)) {
    Ok(addrs) => {
      let addrs: io::Result<Vec<_>> = addrs.map(|r| r.map(|a| a.sockaddr.ip())).collect();
      addrs
    },
    #[cfg(unix)]
    Err(e) => {
        // The lookup failure could be caused by using a stale /etc/resolv.conf.
        // See https://github.com/rust-lang/rust/issues/41570.
        // We therefore force a reload of the nameserver information.
        unsafe {
          c::res_init();
        }
        Err(e)
    },
    // the cfg is needed here to avoid an "unreachable pattern" warning
    #[cfg(not(unix))]
    Err(e) => Err(e),
  }
}

/// Lookup the hostname of a given IP Address via DNS.
///
/// Returns the hostname as a String, or an io::Error on failure.
pub fn lookup_addr(addr: &IpAddr) -> io::Result<String> {
  let socket = ip_to_sockaddr(addr);
  let (inner, len) = socket.into_inner();
  let mut hostbuf = [0 as c::c_char; c::NI_MAXHOST as usize];

  // FIXME: We need some flags, IE: NI_NAMEREQD
  let data = unsafe {
    lookup_errno(c::getnameinfo(inner, len,
                  hostbuf.as_mut_ptr(),
                  c::NI_MAXHOST,
                  ptr::null_mut(), 0, 0))?;

    CStr::from_ptr(hostbuf.as_ptr())
  };

  match str::from_utf8(data.to_bytes()) {
    Ok(name) => Ok(name.to_owned()),
    Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                   "failed to lookup address information"))
  }
}

#[test]
fn test_localhost() {
  // TODO: Find a better test here?
  let ips = lookup_host("localhost").unwrap();
  assert!(ips.contains(&IpAddr::V4("127.0.0.1".parse().unwrap())));
  assert!(!ips.contains(&IpAddr::V4("10.0.0.1".parse().unwrap())));

  let name = lookup_addr(&IpAddr::V4("127.0.0.1".parse().unwrap()));
  assert_eq!(name.unwrap(), "localhost");
}
