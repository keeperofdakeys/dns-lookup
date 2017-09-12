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
  /// Unknown (This variant is for error reporting, as undefined numbers may be later defined).
  Other(u16),
}

impl SockType {
  pub fn from_int(int: c::c_int) -> Option<Self> {
    Some(match int {
      c::SOCK_STREAM => SockType::Stream,
      c::SOCK_DGRAM => SockType::DGram,
      c::SOCK_RAW => SockType::Raw,
      c::SOCK_RDM => SockType::RDM,
      c::SOCK_SEQPACKET => SockType::SeqPacket,
      6 => SockType::DCCP,
      10 => SockType::Packet,
      _ => SockType::Other(int as u16),
    })
  }

  pub fn to_int(&self) -> c::c_int {
    match *self {
      SockType::Unspec => 0,
      SockType::Stream => c::SOCK_STREAM,
      SockType::DGram => c::SOCK_DGRAM,
      SockType::Raw => c::SOCK_RAW,
      SockType::RDM => c::SOCK_RDM,
      SockType::SeqPacket => c::SOCK_SEQPACKET,
      SockType::DCCP => 6,
      SockType::Packet => 10,
      SockType::Other(i) => c::c_int::from(i),
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
  /// Other variants, use libc symbols for matching on this.
  Other(u16),
}

impl AddrFamily {
  pub fn from_int(int: c::c_int) -> Option<Self> {
    Some(match int {
      c::AF_UNSPEC => AddrFamily::Unspec,
      c::AF_LOCAL => AddrFamily::Local,
      // These variants will never match.
      // c::AF_UNIX => AddrFamily::Unix,
      // c::AF_LOCAL => AddrFamily::File,
      c::AF_INET => AddrFamily::Inet,
      c::AF_INET6 => AddrFamily::Inet6,
      c::AF_PACKET => AddrFamily::Packet,
      _ => AddrFamily::Other(int as u16),
    })
  }

  pub fn to_int(&self) -> c::c_int {
    match *self {
      AddrFamily::Unspec => c::AF_UNSPEC,
      AddrFamily::Local => c::AF_LOCAL,
      AddrFamily::Unix => c::AF_UNIX,
      AddrFamily::File => c::AF_LOCAL,
      AddrFamily::Inet => c::AF_INET,
      AddrFamily::Inet6 => c::AF_INET6,
      AddrFamily::Packet => c::AF_PACKET,
      AddrFamily::Other(i) => c::c_int::from(i),
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
  /// Other variants, use libc symbols for matching on this.
  Other(u16),
}

impl Protocol {
  pub fn from_int(int: c::c_int) -> Option<Self> {
    Some(match int {
      c::IPPROTO_IP => Protocol::IP,
      c::IPPROTO_ICMP => Protocol::ICMP,
      c::IPPROTO_TCP => Protocol::TCP,
      c::IPPROTO_UDP => Protocol::UDP,
      c::IPPROTO_RAW => Protocol::RAW,
      _ => Protocol::Other(int as u16),
    })
  }

  pub fn to_int(&self) -> c::c_int {
    match *self {
      Protocol::IP => c::IPPROTO_IP,
      Protocol::ICMP => c::IPPROTO_ICMP,
      Protocol::TCP => c::IPPROTO_TCP,
      Protocol::UDP => c::IPPROTO_UDP,
      Protocol::RAW => c::IPPROTO_RAW,
      Protocol::Other(i) => c::c_int::from(i),
    }
  }
}
