use libc as c;
use std::ffi::CStr;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

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

pub struct AddrInfo<'a> {
  pub flags: c::c_int,
  pub family: Family,
  pub socktype: SockType,
  pub protocol: Protocol,
  pub sockaddr: SocketAddr,
  pub canonname: Option<&'a str>,
  next: *const c::addrinfo,
}

use std::ptr;

impl<'a> AddrInfo<'a> {
  // FIXME: Return an appropriate error type.
  /// Create an AddrInfo struct from a c addrinfo struct.
  fn from_addrinfo(a: c::addrinfo) -> Result<Self, ()> {
    let canonname = unsafe {
      CStr::from_ptr(a.ai_canonname)
        .to_str()
        .ok()
    };
    Ok(AddrInfo {
      flags: 0,
      family: Family::Inet,
      socktype: SockType::Stream,
      protocol: Protocol::Inet,
      sockaddr: SocketAddr::V4(
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0)
      ),
      canonname: canonname,
      next: ptr::null()
    })
  }
}
