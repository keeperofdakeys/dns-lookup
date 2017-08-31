use libc as c;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::io;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use std::ptr;

use err::lookup_errno;

/// Address family
pub enum Family {
  /// Unspecified
  Unspec,
  /// Ipv4
  Inet,
  /// Ipv6
  Inet6
}

impl Family {
  fn from_int(int: c::c_int) -> Option<Self> {
    Some(match int {
      0 => Family::Unspec,
      c::AF_INET => Family::Inet,
      c::AF_INET6 => Family::Inet6,
      _ => return None,
    })
  }

  fn to_int(&self) -> c::c_int {
    match *self {
      Family::Unspec => 0,
      Family::Inet => c::AF_INET,
      Family::Inet6 => c::AF_INET6,
    }
  }
}

/// Types of Sockets
pub enum SockType {
  /// Sequenced, reliable, connection-based byte streams.
  Stream,
  /// Connectionless, unreliable datagrams of fixed max length.
  DGram,
  /// Raw protocol interface.
  Raw,
}

impl SockType {
  fn from_int(int: c::c_int) -> Option<Self> {
    Some(match int {
      c::SOCK_STREAM => SockType::Stream,
      c::SOCK_DGRAM => SockType::DGram,
      c::SOCK_RAW => SockType::Raw,
      _ => return None,
    })
  }

  fn to_int(&self) -> c::c_int {
    match *self {
      SockType::Stream => c::SOCK_STREAM,
      SockType::DGram => c::SOCK_DGRAM,
      SockType::Raw => c::SOCK_RAW,
    }
  }
}

/// Socket Protocol
pub enum Protocol {
  /// Unspecificed.
  Unspec,
  /// Local to host (pipes and file-domain).
  Local,
  /// POSIX name for PF_LOCAL.
  Unix,
  /// POSIX name for PF_LOCAL.
  File,
  /// IP Protocol Family.
  Inet,
}

impl Protocol {
  fn from_int(int: c::c_int) -> Option<Self> {
    Some(match int {
      0 => Protocol::Unspec,
      1 => Protocol::Local,
      2 => Protocol::Inet,
      _ => return None,
    })
  }

  fn to_int(&self) -> c::c_int {
    match *self {
      Protocol::Unspec => 0,
      Protocol::Local => 1,
      Protocol::Unix => 1,
      Protocol::File => 1,
      Protocol::Inet => 2,
    }
  }
}

pub struct AddrInfo {
  pub flags: c::c_int,
  pub family: Family,
  pub socktype: SockType,
  pub protocol: Protocol,
  pub sockaddr: SocketAddr,
  pub canonname: String,
}

impl AddrInfo {
  unsafe fn from_ptr<'a>(a: *mut c::addrinfo) -> io::Result<Self> {
    if a.is_null() {
      return Err(io::Error::new(io::ErrorKind::Other, "Supplied pointer is null."))?;
    }
    let addrinfo = *a;

    Ok(AddrInfo {
      flags: 0,
      family: Family::from_int(addrinfo.ai_family)
        .ok_or(io::Error::new(io::ErrorKind::Other, "Could not find valid address family"))?,
      socktype: SockType::from_int(addrinfo.ai_socktype)
        .ok_or(io::Error::new(io::ErrorKind::Other, "Could not find valid socket type"))?,
      protocol: Protocol::from_int(addrinfo.ai_protocol)
        .ok_or(io::Error::new(io::ErrorKind::Other, "Could not find valid protocol"))?,
      sockaddr: SocketAddr::V4(
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0)
      ),
      canonname:
        CStr::from_ptr(addrinfo.ai_canonname)
          .to_str()
          .unwrap()
          .to_owned()
    })
  }
}

pub fn getaddrinfo() {
  fn new(host: Option<&str>, service: Option<&str>, hints: Option<&AddrInfo>) -> io::Result<AddrInfoIter> {
    // We must have at least host or service.
    if host.is_none() && service.is_none() {
      return Err(io::Error::new(io::ErrorKind::Other, "Either host or service must be supplied"));
    }

    if hints.is_none() {
      unimplemented!();
    }

    // Allocate CStrings, and keep around to free.
    let host = match host {
      Some(host_str) => Some(CString::new(host_str)?),
      None => None
    };
    let c_host = host.map_or(ptr::null(), |s| s.as_ptr());
    let service = match service {
      Some(service_str) => Some(CString::new(service_str)?),
      None => None
    };
    let c_service = service.map_or(ptr::null(), |s| s.as_ptr());

    let mut res = ptr::null_mut();
    unsafe {
      lookup_errno(
        c::getaddrinfo(c_host, c_service, ptr::null(), &mut res)
      )?
    };

    Ok(AddrInfoIter {
      orig: res,
      cur: res,
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



impl AddrInfoIter {
  /// Create an AddrInfo struct from a c addrinfo struct.
  fn new(host: &str) -> io::Result<Self> {
    let c_host = CString::new(host).unwrap();
    let mut res = ptr::null_mut();
    unsafe {
      c::getaddrinfo(c_host.as_ptr(), ptr::null(), ptr::null(), &mut res);
    }
    Ok(AddrInfoIter {
      orig: res,
      cur: res,
    })
  }
}

unsafe impl Sync for AddrInfoIter {}
unsafe impl Send for AddrInfoIter {}

impl Drop for AddrInfoIter {
    fn drop(&mut self) {
        unsafe { c::freeaddrinfo(self.orig) }
    }
}
