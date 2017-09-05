use libc as c;
use std::ffi::{CStr, CString};
use std::io;
use std::mem;
use std::net::SocketAddr;
use std::ptr;

use addr::MySocketAddr;
use err::lookup_errno;
use types::*;

#[derive(Copy, Clone, Debug, PartialEq)]
/// A struct used as the hints argument to getaddrinfo.
pub struct AddrInfoHints {
  /// Type of this socket, Unspec for none.
  pub socktype: SockType,
  /// Protcol family for this socket, Unspec for none..
  pub protocol: ProtoFamily,
  /// Address family for this socket (usually matches protocol family),
  /// Unspec for none..
  pub address: AddrFamily,
  /// Optional bitmask arguments. Bitwise OR this with flags from
  /// libc to change getaddrinfo's behaviour.
  pub flags: u32,
}

impl AddrInfoHints {
  unsafe fn as_addrinfo(&self) -> c::addrinfo {
    let mut addrinfo: c::addrinfo = mem::zeroed();
    addrinfo.ai_socktype = self.socktype.to_int();
    addrinfo.ai_protocol = self.protocol.to_int();
    addrinfo.ai_family = self.address.to_int();
    addrinfo.ai_flags = self.flags as c::c_int;
    addrinfo
  }
}

impl Default for AddrInfoHints {
  /// Generate a blank AddrInfoHints struct, so new values can easily
  /// be specified.
  fn default() -> Self {
    AddrInfoHints {
      socktype: SockType::Unspec,
      protocol: ProtoFamily::Unspec,
      address: AddrFamily::Unspec,
      flags: 0,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddrInfo {
  /// Type of this socket.
  pub socktype: SockType,
  /// Protcol family for this socket.
  pub protocol: ProtoFamily,
  /// Address family for this socket (usually matches protocol family).
  pub address: AddrFamily,
  /// Socket address for this socket, usually containing an actual
  /// IP Address and port.
  pub sockaddr: SocketAddr,
  /// If requested, this is the canonical name for this socket/host.
  pub canonname: Option<String>,
  /// Optional bitmask arguments, usually set to zero.
  pub flags: u32,
}

impl AddrInfo {
  /// Copy the informataion from the given libc::addrinfo pointer, and
  /// create a new AddrInfo struct with that information.
  ///
  /// Used for interfacing with libc::getaddrinfo.
  unsafe fn from_ptr<'a>(a: *mut c::addrinfo) -> io::Result<Self> {
    if a.is_null() {
      return Err(io::Error::new(io::ErrorKind::Other, "Supplied pointer is null."))?;
    }
    let addrinfo = *a;

    Ok(AddrInfo {
      socktype: SockType::from_int(addrinfo.ai_socktype)
        .ok_or(
          io::Error::new(io::ErrorKind::Other,
          format!("Could not find socket type for: {}", addrinfo.ai_socktype))
        )?,
      protocol: ProtoFamily::from_int(addrinfo.ai_protocol)
        .ok_or(
          io::Error::new(io::ErrorKind::Other,
          format!("Could not find protocol for: {}", addrinfo.ai_protocol))
        )?,
      address: AddrFamily::from_int(addrinfo.ai_family)
        .ok_or(
          io::Error::new(io::ErrorKind::Other,
          format!("Could not find family for: {}", addrinfo.ai_family))
        )?,
      sockaddr: MySocketAddr::from_inner(addrinfo.ai_addr, addrinfo.ai_addrlen)?.into(),
      canonname: addrinfo.ai_canonname.as_ref().map(|s|
        CStr::from_ptr(s).to_str().unwrap().to_owned()
      ),
      flags: 0,
    })
  }
}

pub struct AddrInfoIter {
  orig: *mut c::addrinfo,
  cur: *mut c::addrinfo,
}

impl Iterator for AddrInfoIter {
  type Item = io::Result<AddrInfo>;

  fn next(&mut self) -> Option<Self::Item> {
    unsafe {
      if self.cur.is_null() { return None; }
      let ret = AddrInfo::from_ptr(self.cur);
      self.cur = (*self.cur).ai_next as *mut c::addrinfo;
      Some(ret)
    }
  }
}

unsafe impl Sync for AddrInfoIter {}
unsafe impl Send for AddrInfoIter {}

impl Drop for AddrInfoIter {
    fn drop(&mut self) {
        unsafe { c::freeaddrinfo(self.orig) }
    }
}


// TODO: AsRef<OsStr>
pub fn getaddrinfo(host: Option<&str>, service: Option<&str>, hints: Option<AddrInfoHints>)
    -> io::Result<AddrInfoIter> {
  // We must have at least host or service.
  if host.is_none() && service.is_none() {
    return Err(io::Error::new(io::ErrorKind::Other, "Either host or service must be supplied"));
  }

  // Allocate CStrings, and keep around to free.
  let host = match host {
    Some(host_str) => Some(CString::new(host_str)?),
    None => None
  };
  let c_host = host.as_ref().map_or(ptr::null(), |s| s.as_ptr());
  let service = match service {
    Some(service_str) => Some(CString::new(service_str)?),
    None => None
  };
  let c_service = service.as_ref().map_or(ptr::null(), |s| s.as_ptr());

  let c_hints = unsafe {
    match hints {
      Some(hints) => hints.as_addrinfo(),
      None => mem::zeroed(),
    }
  };

  let mut res = ptr::null_mut();
  unsafe {
    match lookup_errno(c::getaddrinfo(c_host, c_service, &c_hints, &mut res)) {
      Ok(_) => {
        Ok(AddrInfoIter { orig: res, cur: res })
      },
      #[cfg(unix)]
      Err(e) => {
        // The lookup failure could be caused by using a stale /etc/resolv.conf.
        // See https://github.com/rust-lang/rust/issues/41570.
        // We therefore force a reload of the nameserver information.
        // This was fixed in glibc 2.26, so this can probably be removed in five years.
        c::res_init();
        Err(e)
      },
      #[cfg(not(unix))]
      Err(e) => Err(e),
    }
  }
}

#[test]
fn test_getaddrinfo() {
  let hints = AddrInfoHints {
    flags: c::AI_CANONNAME as u32,
    ..AddrInfoHints::default()
  };
  for entry in getaddrinfo(Some("localhost"), None, Some(hints)).unwrap() {
    println!("{:?}", entry);
  }
}
