use libc as c;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::io;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
use std::ptr;

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
  unsafe fn from_ptr<'a>(a: *mut c::addrinfo) -> Result<Self, AddrInfoError> {
    if a.is_null() {
      return try!(Err("Pointer is null."));
    }
    let addrinfo = *a;

    Ok(AddrInfo {
      flags: 0,
      family: try!(
        Family::from_int(addrinfo.ai_family)
          .ok_or("Could not find valid address family")
      ),
      socktype: try!(
        SockType::from_int(addrinfo.ai_socktype)
          .ok_or("Could not find valid socket type")
      ),
      protocol: try!(
        Protocol::from_int(addrinfo.ai_protocol)
          .ok_or("Could not find valid protocol")
      ),
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

pub struct AddrInfoIter {
  orig: *mut c::addrinfo,
  cur: *mut c::addrinfo,
}

impl AddrInfoIter {
  // FIXME: Return an appropriate error type.
  /// Create an AddrInfo struct from a c addrinfo struct.
  fn new(host: &str) -> Result<Self, ()> {
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

impl Iterator for AddrInfoIter {
  type Item = Result<AddrInfo, AddrInfoError>;

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

pub enum AddrInfoError {
  IOError(io::Error),
  Other(String)
}

impl From<io::Error> for AddrInfoError {
  fn from(err: io::Error) -> Self {
    AddrInfoError::IOError(err)
  }
}

impl<'a> From<&'a str> for AddrInfoError {
  fn from(err: &'a str) -> Self {
    AddrInfoError::Other(err.to_owned())
  }
}

impl Error for AddrInfoError {
  fn description(&self) -> &str {
    match *self {
      AddrInfoError::IOError(ref err) => "IO Error",
      AddrInfoError::Other(ref err_str) => &err_str
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      AddrInfoError::IOError(ref err) => Some(err),
      AddrInfoError::Other(_) => None
    }
  }
}

impl fmt::Display for AddrInfoError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}

impl fmt::Debug for AddrInfoError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.description())
  }
}
