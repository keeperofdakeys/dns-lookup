use libc as c;
use std::ffi::CStr;
use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr);

/// Address family
pub enum Family {
  /// Ipv4
  Inet4,
  /// Ipv6
  Inet6
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
  /// IP Prootcol Family.
  Inet,
}

pub struct AddrInfo<'a> {
  pub flags: c::c_int,
  pub family: Family,
  pub socktype: SockType,
  pub protocol: Portcool,
  pub sockaddr: SocketAddr,
  pub canonname: Option<&'a str>,
  next: *const c::addrinfo,
}

impl AddrInfo {
  // FIXME: Return an appropriate error type.
  /// Create an AddrInfo struct from a c addrinfo struct.
  fn from_addrinfo(a: c::addrinfo) -> Result<Self, ()> {
    let canonname = try!(
      CStr::from_ptr(a.canonname)
        .as_str()
        .or(())
    );
    Ok(AddrInfo {
      flags: 0,
      family: Family::AfInet4,
      socktype: SockType::Inet4,
      protocol: Protocol::Inet,
      sockaddr: SocketAddr::V4(
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0)
      ),
      canonname: canonname,
      next: 0 as *const _ as *const c::addrinfo
    })
  }
}
