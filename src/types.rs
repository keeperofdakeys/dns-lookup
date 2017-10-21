#[cfg(unix)]
use libc as c;

#[cfg(windows)]
use winapi as c;

/// Socket Type
///
/// Cross platform enum of common Socket Types. For missing types use
/// the `libc` and `ws2_32` crates, depending on platform.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SockType {
  /// Sequenced, reliable, connection-based byte streams.
  Stream,
  /// Connectionless, unreliable datagrams of fixed max length.
  DGram,
  /// Raw protocol interface.
  Raw,
  /// Reliably-delivered messages.
  RDM,
}

impl From<SockType> for c::c_int {
  fn from(sock: SockType) -> c::c_int {
    match sock {
      SockType::Stream => c::SOCK_STREAM,
      SockType::DGram => c::SOCK_DGRAM,
      SockType::Raw => c::SOCK_RAW,
      SockType::RDM => c::SOCK_RDM,
    }
  }
}

impl PartialEq<c::c_int> for SockType {
  fn eq(&self, other: &c::c_int) -> bool {
    let int: c::c_int = (*self).into();
    *other == int
  }
}

impl PartialEq<SockType> for c::c_int {
  fn eq(&self, other: &SockType) -> bool {
    let int: c::c_int = (*other).into();
    *self == int
  }
}

/// Socket Protocol
///
/// Cross platform enum of common Socket Protocols. For missing types use
/// the `libc` and `ws2_32` crates, depending on platform.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Protocol {
  /// Internet Control Message Protocol.
  ICMP,
  /// Transmission Control Protocol.
  TCP,
  /// User Datagram Protocol.
  UDP,
}

impl From<Protocol> for c::c_int {
  #[cfg(unix)]
  fn from(sock: Protocol) -> c::c_int {
    match sock {
      Protocol::ICMP => c::IPPROTO_ICMP,
      Protocol::TCP => c::IPPROTO_TCP,
      Protocol::UDP => c::IPPROTO_UDP,
    }
  }

  #[cfg(windows)]
  fn from(sock: Protocol) -> c::c_int {
    match sock {
      Protocol::ICMP => c::IPPROTO_ICMP.0 as c::c_int,
      Protocol::TCP => c::IPPROTO_TCP.0 as c::c_int,
      Protocol::UDP => c::IPPROTO_UDP.0 as c::c_int,
    }
  }
}

impl PartialEq<c::c_int> for Protocol {
  fn eq(&self, other: &c::c_int) -> bool {
    let int: c::c_int = (*self).into();
    *other == int
  }
}

impl PartialEq<Protocol> for c::c_int {
  fn eq(&self, other: &Protocol) -> bool {
    let int: c::c_int = (*other).into();
    *self == int
  }
}

/// Address Family
///
/// Cross platform enum of common Address Families. For missing types use
/// the `libc` and `ws2_32` crates, depending on platform.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AddrFamily {
  /// Local to host (pipes and file-domain)
  Unix,
  /// IP protocol family.
  Inet,
  /// IP version 6.
  Inet6
}

impl From<AddrFamily> for c::c_int {
  fn from(sock: AddrFamily) -> c::c_int {
    match sock {
      AddrFamily::Unix => c::AF_UNIX,
      AddrFamily::Inet => c::AF_INET,
      AddrFamily::Inet6 => c::AF_INET6,
    }
  }
}

impl PartialEq<c::c_int> for AddrFamily {
  fn eq(&self, other: &c::c_int) -> bool {
    let int: c::c_int = (*self).into();
    *other == int
  }
}

impl PartialEq<AddrFamily> for c::c_int {
  fn eq(&self, other: &AddrFamily) -> bool {
    let int: c::c_int = (*other).into();
    *self == int
  }
}
