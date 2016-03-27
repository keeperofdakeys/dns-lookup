use libc as c;

/// Address family
pub enum Family {
  /// Ipv4
  AfInet4,
  /// Ipv6
  AfInet6
}

/// Types of Sockets
pub enum SockType {
  /// Sequenced, reliable, connection-based byte streams.
  SockStream,
  /// Connectionless, unreliable datagrams of fixed max length.
  SockDGram,
  /// Raw protocol interface.
  SockRaw,
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
}

impl From<c::addrinfo> for AddrInfo {
  // fn from(addrinfo: c::addrinfo) -> Self {
  //   
  // }
}
