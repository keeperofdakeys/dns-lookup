use libc as c;
use std::ffi::{CString, CStr};
use std::io;
use std::mem;
use std::net::{SocketAddr, IpAddr};
use std::ptr;
use std::str;

use addr::{MySocketAddrV4, MySocketAddrV6, ip_to_sockaddr};
use err::lookup_errno;

fn sockaddr_to_addr(storage: &c::sockaddr_storage,
          len: usize) -> io::Result<SocketAddr> {
  match storage.ss_family as c::c_int {
    c::AF_INET => {
      assert!(len as usize >= mem::size_of::<c::sockaddr_in>());
      Ok(
        MySocketAddrV4
          ::from(unsafe { *(storage as *const _ as *const c::sockaddr_in) })
          .into()
      )
    }
    c::AF_INET6 => {
      assert!(len as usize >= mem::size_of::<c::sockaddr_in6>());
      Ok(
        MySocketAddrV6
          ::from(unsafe { *(storage as *const _ as *const c::sockaddr_in6) })
          .into()
      )
    }
    _ => {
      Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid argument"))
    }
  }
}

#[derive(Debug)]
/// A struct that holds a linked list of lookup results.
pub struct LookupHost {
  original: *mut c::addrinfo,
  cur: *mut c::addrinfo,
}

impl Iterator for LookupHost {
  type Item = io::Result<IpAddr>;

  /// Loop through the linked list, returning the next IP.
  fn next(&mut self) -> Option<io::Result<IpAddr>> {
    unsafe {
      if self.cur.is_null() { return None }
      let ret = sockaddr_to_addr(mem::transmute((*self.cur).ai_addr),
             (*self.cur).ai_addrlen as usize);
      self.cur = (*self.cur).ai_next as *mut c::addrinfo;
      Some(ret.map(|s| s.ip()))
    }
  }
}

unsafe impl Sync for LookupHost {}
unsafe impl Send for LookupHost {}

impl Drop for LookupHost {
  fn drop(&mut self) {
    unsafe { c::freeaddrinfo(self.original) }
  }
}

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
pub fn lookup_host(host: &str) -> io::Result<LookupHost> {
  // FIXME: Initialise windows sockets somehow :/
  // #[cfg(windows)]
  // init_windows_sockets();

  let c_host = (CString::new(host))?;
  let mut hints: c::addrinfo = unsafe { mem::zeroed() };
  hints.ai_socktype = c::SOCK_STREAM;
  let mut res = ptr::null_mut();
  unsafe {
    match lookup_errno(c::getaddrinfo(c_host.as_ptr(), ptr::null(), &hints, &mut res)) {
      Ok(_) => {
          Ok(LookupHost { original: res, cur: res })
      },
      #[cfg(unix)]
      Err(e) => {
          // The lookup failure could be caused by using a stale /etc/resolv.conf.
          // See https://github.com/rust-lang/rust/issues/41570.
          // We therefore force a reload of the nameserver information.
          c::res_init();
          Err(e)
      },
      // the cfg is needed here to avoid an "unreachable pattern" warning
      #[cfg(not(unix))]
      Err(e) => Err(e),
    }
  }
}

/// Lookup the hostname of a given IP Address via DNS.
///
/// Returns the hostname as a String, or an io::Error on failure.
pub fn lookup_addr(addr: &IpAddr) -> io::Result<String> {
  let (inner, len) = ip_to_sockaddr(addr);
  let mut hostbuf = [0 as c::c_char; c::NI_MAXHOST as usize];

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
  let ips = lookup_host("localhost").unwrap().collect::<io::Result<Vec<_>>>().unwrap();
  assert!(ips.contains(&IpAddr::V4("127.0.0.1".parse().unwrap())));
  assert!(!ips.contains(&IpAddr::V4("10.0.0.1".parse().unwrap())));

  let name = lookup_addr(&IpAddr::V4("127.0.0.1".parse().unwrap()));
  assert_eq!(name.unwrap(), "localhost");
}
