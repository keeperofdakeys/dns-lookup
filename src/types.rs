use libc as c;

#[derive(Copy, Clone, Debug, PartialEq)]
/// Socket Type
pub enum SockType {
  /// Unspecified (used only for hints argument in getaddrinfo).
  Unspec,
  /// Sequenced, reliable, connection-based byte streams.
  Stream,
  /// Connectionless, unreliable datagrams of fixed max length.
  DGram,
  /// Raw protocol interface.
  Raw,
  /// Reliably-delivered messages.
  RDM,
  /// Sequenced, reliable, connection-based, datagrams of fixed maximum length.
  SeqPacket,
  /// Datagram Congestion Control Protocol.
  DCCP,
  /// Linux specific way of getting packets at the dev level.  For writing rarp
  /// and other similar things on the user level.
  Packet,
  /// Other SockType.
  ///
  /// It's recommended not to match or create this variant directly, as new
  /// variants may be added in the future. Instead you should specify libc
  /// symbols directly, and use the From/Into traits to convert to/from this type.
  _Other(u16),
}

impl From<c::c_int> for SockType {
  fn from(int: c::c_int) -> Self {
    match int {
      c::SOCK_STREAM => SockType::Stream,
      c::SOCK_DGRAM => SockType::DGram,
      c::SOCK_RAW => SockType::Raw,
      c::SOCK_RDM => SockType::RDM,
      c::SOCK_SEQPACKET => SockType::SeqPacket,
      6 => SockType::DCCP,
      10 => SockType::Packet,
      _ => SockType::_Other(int as u16),
    }
  }
}

impl From<SockType> for c::c_int {
  fn from(sock: SockType) -> c::c_int {
    match sock {
      SockType::Unspec => 0,
      SockType::Stream => c::SOCK_STREAM,
      SockType::DGram => c::SOCK_DGRAM,
      SockType::Raw => c::SOCK_RAW,
      SockType::RDM => c::SOCK_RDM,
      SockType::SeqPacket => c::SOCK_SEQPACKET,
      SockType::DCCP => 6,
      SockType::Packet => 10,
      SockType::_Other(i) => c::c_int::from(i),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// Socket Address Family
pub enum AddrFamily {
  /// Unspecified.
  Unspec,
  /// Local to host (pipes and file-domain).
  Local,
  /// POSIX name for PF_LOCAL.
  Unix,
  /// Another non-standard name for PF_LOCAL.
  File,
  /// IP protocol family.
  Inet,
  /// IP version 6.
  Inet6,
  /// Packet family.
  Packet,
  /// Other Address Family.
  ///
  /// It's recommended not to match or create this variant directly, as new
  /// variants may be added in the future. Instead you should specify libc
  /// symbols directly, and use the From/Into traits to convert to/from this type.
  _Other(u16),
}

impl From<c::c_int> for AddrFamily {
  fn from(int: c::c_int) -> Self {
    match int {
      c::AF_UNSPEC => AddrFamily::Unspec,
      c::AF_LOCAL => AddrFamily::Local,
      // These variants will never match.
      // c::AF_UNIX => AddrFamily::Unix,
      // c::AF_LOCAL => AddrFamily::File,
      c::AF_INET => AddrFamily::Inet,
      c::AF_INET6 => AddrFamily::Inet6,
      c::AF_PACKET => AddrFamily::Packet,
      _ => AddrFamily::_Other(int as u16),
    }
  }
}

impl From<AddrFamily> for c::c_int {
  fn from(addr: AddrFamily) -> c::c_int {
    match addr {
      AddrFamily::Unspec => c::AF_UNSPEC,
      AddrFamily::Local => c::AF_LOCAL,
      AddrFamily::Unix => c::AF_UNIX,
      AddrFamily::File => c::AF_LOCAL,
      AddrFamily::Inet => c::AF_INET,
      AddrFamily::Inet6 => c::AF_INET6,
      AddrFamily::Packet => c::AF_PACKET,
      AddrFamily::_Other(i) => c::c_int::from(i),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// Socket Protocol
pub enum Protocol {
  /// Dummy protocol for TCP.
  IP,
  /// Internet Control Message Protocol.
  ICMP,
  /// Transmission Control Protocol.
  TCP,
  /// User Datagram Protocol.
  UDP,
  /// Raw IP packets.
  RAW,
  /// Other Protocol.
  ///
  /// It's recommended not to match or create this variant directly, as new
  /// variants may be added in the future. Instead you should specify libc
  /// symbols directly, and use the From/Into traits to convert to/from this type.
  _Other(u16),
}

impl From<c::c_int> for Protocol {
  fn from(int: c::c_int) -> Self {
    match int {
      c::IPPROTO_IP => Protocol::IP,
      c::IPPROTO_ICMP => Protocol::ICMP,
      c::IPPROTO_TCP => Protocol::TCP,
      c::IPPROTO_UDP => Protocol::UDP,
      c::IPPROTO_RAW => Protocol::RAW,
      _ => Protocol::_Other(int as u16),
    }
  }
}

impl From<Protocol> for c::c_int {
  fn from(proto: Protocol) -> c::c_int {
    match proto {
      Protocol::IP => c::IPPROTO_IP,
      Protocol::ICMP => c::IPPROTO_ICMP,
      Protocol::TCP => c::IPPROTO_TCP,
      Protocol::UDP => c::IPPROTO_UDP,
      Protocol::RAW => c::IPPROTO_RAW,
      Protocol::_Other(i) => c::c_int::from(i),
    }
  }
}
