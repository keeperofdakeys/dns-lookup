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
  /// Amateur Radio AX.25.
  Ax25,
  /// Novell Internet Protocol.
  Ipx,
  /// Appletalk DDP.
  Appletalk,
  /// Amateur radio NetROM.
  Netrom,
  /// Multiprotocol bridge.
  Bridge,
  /// ATM PVCs.
  Atmpvc,
  /// Reserved for X.25 project.
  X25,
  /// IP version 6.
  Inet6,
  /// Amateur Radio X.25 PLP.
  Rose,
  /// Reserved for DECnet project.
  Decnet,
  /// Reserved for 802.2LLC project.
  Netbeui,
  /// Security callback pseudo AF.
  Security,
  /// PF_KEY key management API.
  Key,
  /// Alias to emulate 4.4BSD.
  Netlink,
  /// Alias to emulate 4.4BSD.
  Route,
  /// Packet family.
  Packet,
  /// Ash.
  Ash,
  /// Acorn Econet.
  Econet,
  /// ATM SVCs.
  Atmsvc,
  /// RDS sockets.
  Rds,
  /// Linux SNA Project
  Sna,
  /// IRDA sockets.
  Irda,
  /// PPPoX sockets.
  Pppox,
  /// Wanpipe API sockets.
  Wanpipe,
  /// Linux LLC.
  Llc,
  /// Native InfiniBand address.
  Ib,
  /// MPLS.
  Mpls,
  /// Controller Area Network.
  Can,
  /// TIPC sockets.
  Tipc,
  /// Bluetooth sockets.
  Bluetooth,
  /// IUCV sockets.
  Iucv,
  /// RxRPC sockets.
  Rxrpc,
  /// mISDN sockets.
  Isdn,
  /// Phonet sockets.
  Phonet,
  /// IEEE 802.15.4 sockets.
  Ieee802154,
  /// CAIF sockets.
  Caif,
  /// Algorithm sockets.
  Alg,
  /// NFC sockets.
  Nfc,
  /// vSockets.
  Vsock,
  /// For now..
  Max,
  /// Unknown (This variant is for error reporting, as undefined numbers may be later defined).
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
      c::AF_AX25 => AddrFamily::Ax25,
      c::AF_IPX => AddrFamily::Ipx,
      c::AF_APPLETALK => AddrFamily::Appletalk,
      c::AF_NETROM => AddrFamily::Netrom,
      c::AF_BRIDGE => AddrFamily::Bridge,
      c::AF_ATMPVC => AddrFamily::Atmpvc,
      c::AF_X25 => AddrFamily::X25,
      c::AF_INET6 => AddrFamily::Inet6,
      c::AF_ROSE => AddrFamily::Rose,
      // c::AF_DECNET => AddrFamily::Decnet,
      12 => AddrFamily::Decnet,
      c::AF_NETBEUI => AddrFamily::Netbeui,
      c::AF_SECURITY => AddrFamily::Security,
      c::AF_KEY => AddrFamily::Key,
      c::AF_NETLINK => AddrFamily::Netlink,
      // c::AF_ROUTE => AddrFamily::Route,
      c::AF_PACKET => AddrFamily::Packet,
      c::AF_ASH => AddrFamily::Ash,
      c::AF_ECONET => AddrFamily::Econet,
      c::AF_ATMSVC => AddrFamily::Atmsvc,
      c::AF_RDS => AddrFamily::Rds,
      c::AF_SNA => AddrFamily::Sna,
      c::AF_IRDA => AddrFamily::Irda,
      c::AF_PPPOX => AddrFamily::Pppox,
      c::AF_WANPIPE => AddrFamily::Wanpipe,
      c::AF_LLC => AddrFamily::Llc,
      c::AF_IB => AddrFamily::Ib,
      c::AF_MPLS => AddrFamily::Mpls,
      c::AF_CAN => AddrFamily::Can,
      c::AF_TIPC => AddrFamily::Tipc,
      c::AF_BLUETOOTH => AddrFamily::Bluetooth,
      c::AF_IUCV => AddrFamily::Iucv,
      c::AF_RXRPC => AddrFamily::Rxrpc,
      c::AF_ISDN => AddrFamily::Isdn,
      c::AF_PHONET => AddrFamily::Phonet,
      c::AF_IEEE802154 => AddrFamily::Ieee802154,
      c::AF_CAIF => AddrFamily::Caif,
      c::AF_ALG => AddrFamily::Alg,
      c::AF_NFC => AddrFamily::Nfc,
      c::AF_VSOCK => AddrFamily::Vsock,
      c::AF_MAX => AddrFamily::Max,
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
      AddrFamily::Ax25 => c::AF_AX25,
      AddrFamily::Ipx => c::AF_IPX,
      AddrFamily::Appletalk => c::AF_APPLETALK,
      AddrFamily::Netrom => c::AF_NETROM,
      AddrFamily::Bridge => c::AF_BRIDGE,
      AddrFamily::Atmpvc => c::AF_ATMPVC,
      AddrFamily::X25 => c::AF_X25,
      AddrFamily::Inet6 => c::AF_INET6,
      AddrFamily::Rose => c::AF_ROSE,
      // AddrFamily::Decnet => c::AF_DECNET,
      AddrFamily::Decnet => 12,
      AddrFamily::Netbeui => c::AF_NETBEUI,
      AddrFamily::Security => c::AF_SECURITY,
      AddrFamily::Key => c::AF_KEY,
      AddrFamily::Netlink => c::AF_NETLINK,
      AddrFamily::Route => c::AF_ROUTE,
      AddrFamily::Packet => c::AF_PACKET,
      AddrFamily::Ash => c::AF_ASH,
      AddrFamily::Econet => c::AF_ECONET,
      AddrFamily::Atmsvc => c::AF_ATMSVC,
      AddrFamily::Rds => c::AF_RDS,
      AddrFamily::Sna => c::AF_SNA,
      AddrFamily::Irda => c::AF_IRDA,
      AddrFamily::Pppox => c::AF_PPPOX,
      AddrFamily::Wanpipe => c::AF_WANPIPE,
      AddrFamily::Llc => c::AF_LLC,
      AddrFamily::Ib => c::AF_IB,
      AddrFamily::Mpls => c::AF_MPLS,
      AddrFamily::Can => c::AF_CAN,
      AddrFamily::Tipc => c::AF_TIPC,
      AddrFamily::Bluetooth => c::AF_BLUETOOTH,
      AddrFamily::Iucv => c::AF_IUCV,
      AddrFamily::Rxrpc => c::AF_RXRPC,
      AddrFamily::Isdn => c::AF_ISDN,
      AddrFamily::Phonet => c::AF_PHONET,
      AddrFamily::Ieee802154 => c::AF_IEEE802154,
      AddrFamily::Caif => c::AF_CAIF,
      AddrFamily::Alg => c::AF_ALG,
      AddrFamily::Nfc => c::AF_NFC,
      AddrFamily::Vsock => c::AF_VSOCK,
      AddrFamily::Max => c::AF_MAX,
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
